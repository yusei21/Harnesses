//! Typed multi-agent workflow and fail-closed policy. No tool execution.
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tokio::task::JoinSet;

pub const MAX_CALLS: usize = 5;
pub const MAX_PARALLEL: usize = 3;
const MAX_GOAL_BYTES: usize = 8_000;
const MAX_SHARED_CONTEXT_BYTES: usize = 16_000;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Risk {
    Read,
    Write,
    External,
    Destructive,
}
/// Authenticated side-effect approvals are deliberately NOT implemented.
pub struct Policy;
impl Policy {
    pub fn can_execute(&self, risk: Risk) -> bool {
        matches!(risk, Risk::Read)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Route {
    pub provider: String,
    pub model: String,
}
impl Route {
    pub fn validate(&self, allow_cloud: bool) -> Result<(), RunError> {
        if !matches!(self.provider.as_str(), "ollama" | "openai" | "anthropic" | "gemini") {
            return Err(RunError::InvalidRoute);
        }
        if self.model.is_empty() || self.model.len() > 128
            || self.model.contains("..")
            || !self.model.chars().all(|c| c.is_ascii_alphanumeric() || "._:-".contains(c))
        {
            return Err(RunError::InvalidRoute);
        }
        if self.provider != "ollama" && !allow_cloud {
            return Err(RunError::CloudConsentRequired);
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Agent {
    pub name: String,
    pub route: Route,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Stage {
    pub agents: Vec<Agent>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Plan {
    pub goal: String,
    pub stages: Vec<Stage>,
    pub max_calls: usize,
    pub max_parallel: usize,
    pub allow_cloud: bool,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Answer {
    pub agent: String,
    pub text: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelRequest {
    pub agent: String,
    pub route: Route,
    pub goal: String,
    pub prior: Vec<Answer>,
    pub allow_cloud: bool,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelResponse {
    pub text: String,
    pub input_tokens: Option<u64>,
    pub output_tokens: Option<u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Event {
    pub agent: String,
    pub provider: String,
    pub model: String,
    pub status: String,
    pub input_tokens: Option<u64>,
    pub output_tokens: Option<u64>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RunResult {
    pub status: String,
    pub mode: String,
    pub events: Vec<Event>,
    pub outputs: Vec<Answer>,
    pub final_text: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RunError {
    InvalidPlan,
    InvalidRoute,
    CloudConsentRequired,
    BudgetExceeded,
    ContextTooLarge,
    WorkerFailure,
    TaskFailed,
}
impl std::fmt::Display for RunError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::InvalidPlan => "invalid_plan",
            Self::InvalidRoute => "invalid_route",
            Self::CloudConsentRequired => "cloud_consent_required",
            Self::BudgetExceeded => "budget_exceeded",
            Self::ContextTooLarge => "context_too_large",
            Self::WorkerFailure => "worker_failure",
            Self::TaskFailed => "task_failed",
        };
        write!(f, "{s}")
    }
}
impl std::error::Error for RunError {}

pub type GatewayFuture<'a> =
    Pin<Box<dyn Future<Output = Result<ModelResponse, RunError>> + Send + 'a>>;
pub trait Gateway: Send + Sync {
    fn complete(&self, request: ModelRequest) -> GatewayFuture<'_>;
}

fn allowed_agent(name: &str) -> bool {
    matches!(name, "planner" | "researcher" | "engineer" | "reviewer"
        | "evaluator" | "memory-curator" | "optimizer")
}

/// Validate all routes and limits BEFORE invoking a single model.
pub fn validate_plan(plan: &Plan) -> Result<(), RunError> {
    if plan.goal.trim().is_empty() || plan.goal.len() > MAX_GOAL_BYTES
        || plan.stages.is_empty() || plan.max_calls == 0 || plan.max_calls > MAX_CALLS
        || plan.max_parallel == 0 || plan.max_parallel > MAX_PARALLEL
    {
        return Err(RunError::InvalidPlan);
    }
    let mut seen = HashSet::new();
    let mut count = 0usize;
    for stage in &plan.stages {
        if stage.agents.is_empty() || stage.agents.len() > plan.max_parallel {
            return Err(RunError::InvalidPlan);
        }
        count += stage.agents.len();
        for agent in &stage.agents {
            if !allowed_agent(&agent.name) || !seen.insert(agent.name.as_str()) {
                return Err(RunError::InvalidPlan);
            }
            agent.route.validate(plan.allow_cloud)?;
        }
    }
    if count > plan.max_calls {
        return Err(RunError::BudgetExceeded);
    }
    Ok(())
}

pub struct Runner {
    gateway: Arc<dyn Gateway>,
}
impl Runner {
    pub fn new(gateway: Arc<dyn Gateway>) -> Self {
        Self { gateway }
    }
    pub async fn run(&self, plan: Plan) -> Result<RunResult, RunError> {
        validate_plan(&plan)?;
        let mut previous: Vec<Answer> = Vec::new();
        let mut events: Vec<Event> = Vec::new();
        for stage in plan.stages {
            let context = serde_json::to_vec(&previous).map_err(|_| RunError::TaskFailed)?;
            if context.len() > MAX_SHARED_CONTEXT_BYTES {
                return Err(RunError::ContextTooLarge);
            }
            // Each stage receives a snapshot of prior stages, not peer outputs.
            // Tasks are scoped and awaited; dropping the JoinSet aborts pending tasks.
            let mut tasks = JoinSet::new();
            for (index, agent) in stage.agents.iter().enumerate() {
                let gateway = self.gateway.clone();
                let request = ModelRequest {
                    agent: agent.name.clone(),
                    route: agent.route.clone(),
                    goal: plan.goal.clone(),
                    prior: previous.clone(),
                    allow_cloud: plan.allow_cloud,
                };
                tasks.spawn(async move { (index, gateway.complete(request).await) });
            }
            let mut results: Vec<Option<ModelResponse>> = vec![None; stage.agents.len()];
            while let Some(res) = tasks.join_next().await {
                let (index, reply) = res.map_err(|_| RunError::WorkerFailure)?;
                let reply = reply?;
                if reply.text.trim().is_empty() || reply.text.len() > 24_000 {
                    return Err(RunError::WorkerFailure);
                }
                results[index] = Some(reply);
            }
            // Stable output ordering regardless of completion order.
            for (agent, reply) in stage.agents.iter().zip(results.into_iter()) {
                let reply = reply.ok_or(RunError::WorkerFailure)?;
                events.push(Event {
                    agent: agent.name.clone(),
                    provider: agent.route.provider.clone(),
                    model: agent.route.model.clone(),
                    status: "completed".into(),
                    input_tokens: reply.input_tokens,
                    output_tokens: reply.output_tokens,
                });
                previous.push(Answer { agent: agent.name.clone(), text: reply.text });
            }
        }
        let final_text = previous.last().ok_or(RunError::TaskFailed)?.text.clone();
        Ok(RunResult { status: "completed".into(), mode: "rust-orchestrated-text".into(),
                       events, outputs: previous, final_text })
    }
}
