"""Structural checks, not a replacement for official Agent Skills validation."""
import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
errors = []
slug = re.compile(r"^[a-z0-9]+(?:-[a-z0-9]+)*$")
agents = list((ROOT / "agents").glob("*.json"))
skills = list((ROOT / "skills").glob("*/SKILL.md"))

for path in agents:
    try:
        obj = json.loads(path.read_text(encoding="utf-8"))
        mandatory = {"name", "description", "inputs", "outputs", "risk", "docs"}
        if mandatory - obj.keys():
            errors.append(f"{path}: missing fields {mandatory - obj.keys()}")
        if obj.get("name") != path.stem:
            errors.append(f"{path}: name must match filename")
        if obj.get("risk") not in {"read", "write", "external", "destructive"}:
            errors.append(f"{path}: invalid risk")
        if not (ROOT / "docs" / "agents" / (path.stem + ".md")).is_file():
            errors.append(f"{path}: missing individual documentation")
    except Exception as exc:
        errors.append(f"{path}: {exc}")

for path in skills:
    text = path.read_text(encoding="utf-8")
    name = path.parent.name
    if not slug.fullmatch(name):
        errors.append(f"{path}: invalid slug")
    if not text.startswith("---\n") or "\n---\n" not in text[4:]:
        errors.append(f"{path}: invalid frontmatter")
        continue
    fm = text.split("---", 2)[1]
    if f"name: {name}" not in fm or not re.search(r"(?m)^description: .+", fm):
        errors.append(f"{path}: missing/mismatched name or description")
    if not (ROOT / "docs" / "skills" / f"{name}.md").is_file():
        errors.append(f"{path}: missing individual documentation")

if not agents or not skills:
    errors.append("Expected at least one agent and one skill")

if errors:
    for item in errors:
        print("ERROR:", item)
    sys.exit(1)
print(f"OK: {len(agents)} agents, {len(skills)} skills, each with docs")
