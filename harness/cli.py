"""Offline demonstration CLI, no network/API calls."""
import argparse
import json
import sys
from .core import Orchestrator, make_task, trace_envelope
from .catalog import agent_catalog, skill_catalog
from .evaluation import evaluate

def main() -> int:
    parser = argparse.ArgumentParser(prog="python -m harness.cli")
    sub = parser.add_subparsers(dest="cmd", required=True)
    sub.add_parser("catalog")
    demo = sub.add_parser("demo")
    demo.add_argument("--task", required=True)
    sub.add_parser("eval")
    args = parser.parse_args()
    if args.cmd == "catalog":
        payload = {"agents": agent_catalog(), "skills": skill_catalog()}
    elif args.cmd == "demo":
        result = Orchestrator().run(make_task(args.task))
        payload = {"result": result.as_dict(), "safe_trace": trace_envelope(result)}
    else:
        payload = evaluate()
    print(json.dumps(payload, ensure_ascii=False, indent=2))
    return 1 if args.cmd == "eval" and not payload["passed"] else 0

if __name__ == "__main__":
    sys.exit(main())
