use harnesses_core::{
    validate_plan, Agent, Gateway, GatewayFuture, ModelRequest, ModelResponse,
    Plan, Policy, Risk, Route, RunError, Runner, Stage,
};
use std::sync::{Arc, Mutex};
use std::time::Duration;

struct Mock {
    calls: Arc<Mutex<Vec<ModelRequest>>>,
}
impl Gateway for Mock {
    fn complete(&self, request: ModelRequest) -> GatewayFuture<'_> {
        let calls = self.calls.clone();
        Box::pin(async move {
            calls.lock().unwrap().push(request.clone());
            if request.agent == "researcher" {
                tokio::time::sleep(Duration::from_millis(40)).await;
            }
            Ok(ModelResponse {
                text: format!("result-{}", request.agent),
                input_tokens: Some(4),
                output_tokens: Some(2),
            })
        })
    }
}
fn a(name: &str, provider: &str) -> Agent {
    Agent { name: name.to_string(), route: Route {
        provider: provider.to_string(), model: "test-model".into()
    }}
}
fn plan() -> Plan {
    Plan {
        goal: "Inspecionar o projeto".into(),
        stages: vec![
            Stage { agents: vec![a("planner", "ollama")] },
            Stage { agents: vec![a("researcher", "ollama"), a("engineer", "ollama")] },
            Stage { agents: vec![a("reviewer", "ollama")] },
        ],
        max_calls: 4,
        max_parallel: 2,
        allow_cloud: false,
    }
}
#[tokio::test]
async fn stable_order_and_stage_context() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let gateway = Arc::new(Mock { calls: calls.clone() });
    let result = Runner::new(gateway).run(plan()).await.unwrap();
    assert_eq!(result.outputs.iter().map(|a| a.agent.as_str()).collect::<Vec<_>>(),
               vec!["planner", "researcher", "engineer", "reviewer"]);
    assert_eq!(result.final_text, "result-reviewer");
    let all = calls.lock().unwrap();
    assert_eq!(all.len(), 4);
    let planner = all.iter().find(|c| c.agent == "planner").unwrap();
    assert!(planner.prior.is_empty());
    let researcher = all.iter().find(|c| c.agent == "researcher").unwrap();
    let engineer = all.iter().find(|c| c.agent == "engineer").unwrap();
    assert_eq!(researcher.prior.len(), 1);
    assert_eq!(engineer.prior.len(), 1); // peers do not see each other
    assert_eq!(all.iter().find(|c| c.agent == "reviewer").unwrap().prior.len(), 3);
}
#[tokio::test]
async fn cloud_requires_consent_before_any_calls() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let mut plan = plan();
    plan.stages[2].agents[0].route.provider = "openai".into();
    let result = Runner::new(Arc::new(Mock { calls: calls.clone() })).run(plan).await;
    assert_eq!(result.unwrap_err(), RunError::CloudConsentRequired);
    assert!(calls.lock().unwrap().is_empty());
}
#[test]
fn limits_prevent_unbounded_fanout() {
    let mut p = plan();
    p.max_parallel = 1;
    assert_eq!(validate_plan(&p), Err(RunError::InvalidPlan));
    let mut p = plan();
    p.max_calls = 3;
    assert_eq!(validate_plan(&p), Err(RunError::BudgetExceeded));
    let mut p = plan();
    p.stages.push(Stage { agents: vec![a("planner", "ollama")] });
    assert_eq!(validate_plan(&p), Err(RunError::InvalidPlan));
}
#[test]
fn model_route_is_validated() {
    let mut p = plan();
    p.stages[0].agents[0].route.model = "../bad".into();
    assert_eq!(validate_plan(&p), Err(RunError::InvalidRoute));
}
#[test]
fn policy_denies_all_side_effects() {
    let policy = Policy;
    assert!(policy.can_execute(Risk::Read));
    assert!(!policy.can_execute(Risk::Write));
    assert!(!policy.can_execute(Risk::External));
    assert!(!policy.can_execute(Risk::Destructive));
}
#[test]
fn rejects_unknown_roles() {
    let mut p = plan();
    p.stages[0].agents[0].name = "system-root".into();
    assert_eq!(validate_plan(&p), Err(RunError::InvalidPlan));
}
