"""Validate domain-specific Skills without executing code or consulting external models."""
from pathlib import Path
import json
import re
import sys

ROOT = Path(__file__).resolve().parents[1]
SKILLS = [
    "ml-experiment-design", "dataset-leakage-audit", "pytorch-training-review",
    "rag-pipeline-evaluation", "llm-evaluation-lab", "saas-system-design",
    "frontend-interface-review", "backend-api-contracts", "saas-security-threat-model"
]
errors = []
catalog = json.loads((ROOT / "capabilities/catalog.json").read_text(encoding="utf-8"))
entries = {item["id"]: item for item in catalog["capabilities"]}
for name in SKILLS:
    root = ROOT / "skills" / name
    md = root / "SKILL.md"
    if not md.is_file():
        errors.append(f"{name}: missing SKILL.md")
        continue
    content = md.read_text(encoding="utf-8")
    lines = content.splitlines()
    if len(lines) < 4 or lines[0] != "---" or lines.count("---") < 2:
        errors.append(f"{name}: invalid frontmatter")
        continue
    front = "\n".join(lines[1:lines.index("---", 1)])
    if f"name: {name}" not in front or not re.search(r"(?m)^description: .{35,}", front):
        errors.append(f"{name}: missing name/description")
    if not all(part in content for part in ("## Quando usar","## Entrada","## Procedimento","## Saída","## Limites")):
        errors.append(f"{name}: missing operational sections")
    if not all((root / relative).is_file() for relative in ("references/checklist.md","evals/scenarios.json")):
        errors.append(f"{name}: missing reference/evaluations")
        continue
    entry = entries.get(name)
    if not entry or entry.get("path") != f"skills/{name}/SKILL.md":
        errors.append(f"{name}: not in canonical catalog")
    if not (ROOT / "docs" / "skills" / (name + ".md")).is_file():
        errors.append(f"{name}: missing individual docs")
    try:
        obj = json.loads((root / "evals/scenarios.json").read_text(encoding="utf-8"))
        cases = obj["cases"]
        if obj.get("skill") != name or len(cases) < 3:
            errors.append(f"{name}: inadequate cases")
        if len(set(case.get("id") for case in cases)) != len(cases):
            errors.append(f"{name}: duplicate case IDs")
        for case in cases:
            if not isinstance(case.get("task"), str) or not case.get("expected") or not case.get("negative"):
                errors.append(f"{name}: incomplete scenario")
    except (OSError, TypeError, ValueError, KeyError):
        errors.append(f"{name}: invalid JSON scenarios")
if errors:
    for error in errors: print("ERROR:", error)
    sys.exit(1)
print(f"OK: {len(SKILLS)} domain Skills, with docs, references and negative scenarios")
