"""Offline and real multiprovider CLI."""
import argparse
import json
import sys
from .core import Orchestrator, make_task, trace_envelope
from .catalog import agent_catalog, skill_catalog
from .evaluation import evaluate
from .live import run_live, Route, parse_overrides, LiveError
from .providers import ProviderError

def main():
    parser = argparse.ArgumentParser(prog="python -m harness.cli")
    sub = parser.add_subparsers(dest="cmd", required=True)
    sub.add_parser("catalog")
    demo = sub.add_parser("demo")
    demo.add_argument("--task", required=True)
    sub.add_parser("eval")
    live = sub.add_parser("run", help="real text-only LLM inference; no tool use")
    live.add_argument("--task", required=True)
    live.add_argument("--provider", choices=["ollama","openai","anthropic","gemini"], default="ollama")
    live.add_argument("--model", required=True)
    live.add_argument("--agents", default="planner,engineer,reviewer")
    live.add_argument("--agent-model", action="append", default=[],
        help="agent=provider:model; repeat for multiple agents")
    live.add_argument("--max-calls", type=int, default=3)
    live.add_argument("--allow-cloud", action="store_true")
    args = parser.parse_args()
    try:
        if args.cmd == "catalog":
            result = {"agents": agent_catalog(), "skills": skill_catalog()}
        elif args.cmd == "demo":
            output = Orchestrator().run(make_task(args.task))
            result = {"result": output.as_dict(), "safe_trace": trace_envelope(output)}
        elif args.cmd == "eval":
            result = evaluate()
        else:
            agents = tuple(x.strip() for x in args.agents.split(","))
            routes = parse_overrides(args.agent_model, agents)
            result = run_live(args.task, agents, Route(args.provider,args.model),
                routes, allow_cloud=args.allow_cloud, max_calls=args.max_calls)
        print(json.dumps(result, ensure_ascii=False, indent=2))
        return 1 if args.cmd == "eval" and not result["passed"] else 0
    except (LiveError, ProviderError) as exc:
        print(json.dumps({"status":"error","reason":str(exc)}), file=sys.stderr)
        return 2

if __name__ == "__main__":
    sys.exit(main())
