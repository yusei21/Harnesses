//! Contract-level regression for skill-proposed routing cases (not LLM evals).
use harnesses_core::{validate_plan, Plan};
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize)]
struct Scenario {
    id: String,
    expected_valid: bool,
    plan: Plan,
}
#[derive(Deserialize)]
struct Collection {
    cases: Vec<Scenario>,
}
#[test]
fn routing_scenarios_match_runtime_policy() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..");
    let file = root.join("skills/multi-agent-routing/evals/scenarios.json");
    let content = fs::read_to_string(file).expect("scenario fixture");
    let cases: Collection = serde_json::from_str(&content).expect("typed scenario fixture");
    assert!(cases.cases.len() >= 3);
    for case in cases.cases {
        assert_eq!(
            validate_plan(&case.plan).is_ok(),
            case.expected_valid,
            "routing scenario {} contradicts Rust runtime",
            case.id
        );
    }
}
