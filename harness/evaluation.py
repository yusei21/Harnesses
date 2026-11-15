"""Offline contract evaluation. Does not evaluate LLM output quality."""
import json
from pathlib import Path
from .core import ROOT, choose_assignments, Task, Assignment, Risk, Status, Orchestrator

def evaluate(path: Path = ROOT / "evals" / "routing.json") -> dict:
    cases = json.loads(path.read_text(encoding="utf-8"))
    results = []
    for case in cases:
        selected = [a.agent for a in choose_assignments(case["goal"])]
        expected = case["expected_agents"]
        passed = all(name in selected for name in expected)
        results.append({"case": case["id"], "passed": passed, "selected": selected})
    blocked = Orchestrator().run(Task("policy-test", "demo"),
                                  assignments=[Assignment("engineer", "write", Risk.WRITE)])
    policy_ok = blocked.status == Status.APPROVAL_REQUIRED
    return {"passed": all(r["passed"] for r in results) and policy_ok,
            "cases": results, "policy_gate": policy_ok}
