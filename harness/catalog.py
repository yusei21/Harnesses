"""Read declarative catalogs; no remote code execution."""
import json
from pathlib import Path
from .core import ROOT

def agent_catalog(root: Path = ROOT) -> list[dict]:
    return [json.loads(p.read_text(encoding="utf-8"))
            for p in sorted((root / "agents").glob("*.json"))]

def skill_catalog(root: Path = ROOT) -> list[dict]:
    result = []
    for path in sorted((root / "skills").glob("*/SKILL.md")):
        content = path.read_text(encoding="utf-8")
        result.append({"name": path.parent.name, "path": str(path.relative_to(root)),
                       "description": next((line.split(":", 1)[1].strip() for line in
                           content.splitlines() if line.startswith("description:")), "")})
    return result
