//! Local CLI, Rust owned. Python is only a model inference transport.
use harnesses_core::{Agent, Plan, PythonGateway, Route, Runner, Stage, validate_plan};
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;

fn help() {
    eprintln!("Usage: cargo run --manifest-path rust/Cargo.toml -- run --task TEXT [--provider ollama] --model MODEL [--stages planner|researcher,engineer|reviewer] [--agent-model agent=provider:model] [--allow-cloud] [--max-calls 5] [--max-parallel 3]");
}
fn parse_route(input: &str) -> Result<Route, String> {
    let (provider, model) = input.split_once(':').ok_or("expected provider:model")?;
    Ok(Route { provider: provider.into(), model: model.into() })
}
fn plan_from_args(args: &[String]) -> Result<Plan, String> {
    let mut task = None;
    let mut provider = "ollama".to_owned();
    let mut model = None;
    let mut stages = "planner|engineer|reviewer".to_owned();
    let mut overrides = HashMap::new();
    let mut allow_cloud = false;
    let mut max_calls = 5usize;
    let mut max_parallel = 3usize;
    let mut i = 0;
    while i < args.len() {
        let option = args[i].as_str();
        if option == "--allow-cloud" {
            allow_cloud = true;
            i += 1;
            continue;
        }
        let value = args.get(i + 1).ok_or("missing flag argument")?.clone();
        match option {
            "--task" => task = Some(value),
            "--provider" => provider = value,
            "--model" => model = Some(value),
            "--stages" => stages = value,
            "--max-calls" => max_calls = value.parse().map_err(|_| "invalid max-calls")?,
            "--max-parallel" => max_parallel = value.parse().map_err(|_| "invalid max-parallel")?,
            "--agent-model" => {
                let (agent, route) = value.split_once('=').ok_or("expected agent=provider:model")?;
                if overrides.insert(agent.to_owned(), parse_route(route)?).is_some() {
                    return Err("duplicate override".into());
                }
            }
            _ => return Err("unknown option".into()),
        }
        i += 2;
    }
    let default = Route { provider, model: model.ok_or("missing --model")? };
    let mut seen_overrides = HashMap::new();
    let stages: Vec<Stage> = stages.split('|').map(|part| {
        let agents = part.split(',').map(|role| {
            seen_overrides.insert(role.to_owned(), true);
            Agent { name: role.to_owned(),
                    route: overrides.get(role).cloned().unwrap_or_else(|| default.clone()) }
        }).collect();
        Stage { agents }
    }).collect();
    if overrides.keys().any(|name| !seen_overrides.contains_key(name)) {
        return Err("unknown override agent".into());
    }
    let plan = Plan { goal: task.ok_or("missing --task")?, stages,
                      max_calls, max_parallel, allow_cloud };
    validate_plan(&plan).map_err(|e| e.to_string())?;
    Ok(plan)
}
#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.first().map(String::as_str) != Some("run") {
        help();
        std::process::exit(2);
    }
    let plan = match plan_from_args(&args[1..]) {
        Ok(plan) => plan,
        Err(error) => {
            eprintln!("{}", json!({"status":"error","reason":error}));
            help();
            std::process::exit(2);
        }
    };
    let runner = Runner::new(Arc::new(PythonGateway::repository_default()));
    match runner.run(plan).await {
        Ok(result) => println!("{}", serde_json::to_string_pretty(&result).unwrap_or_default()),
        Err(error) => {
            eprintln!("{}", json!({"status":"error","reason":error.to_string()}));
            std::process::exit(1);
        }
    }
}
