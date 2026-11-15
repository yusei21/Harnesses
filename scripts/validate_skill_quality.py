"""Offline structural checks for pilot Skills. No model claims or script execution."""
from __future__ import annotations
import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
PILOTS = ("multi-agent-routing", "code-change", "rust-quality-gate")
errors: list[str] = []

def fail(skill: str, message: str) -> None:
    errors.append(f"{skill}: {message}")

for name in PILOTS:
    folder = ROOT / "skills" / name
    file = folder / "SKILL.md"
    if not file.is_file():
        fail(name, "missing SKILL.md")
        continue
    raw = file.read_text(encoding="utf-8")
    lines = raw.splitlines()
    if not lines or lines[0] != "---" or lines.count("---") < 2:
        fail(name, "invalid YAML frontmatter delimiters")
        continue
    close = lines.index("---", 1)
    front = lines[1:close]
    props = dict(line.split(":", 1) for line in front if ":" in line)
    props = {key.strip(): value.strip() for key, value in props.items()}
    if props.get("name") != name:
        fail(name, "name mismatch")
    description = props.get("description", "")
    if not 30 <= len(description) <= 1024 or not re.search(
        r"\b(use|quando|para|upon)\b", description, re.IGNORECASE
    ):
        fail(name, "description must identify purpose and activation")
    if not re.fullmatch(r"[a-z0-9]+(?:-[a-z0-9]+)*", name):
        fail(name, "invalid slug")
    if len(lines) - close > 500:
        fail(name, "SKILL.md too long")
    body = "\n".join(lines[close + 1:])
    for title in ("## Entrada", "## Procedimento", "## Saída"):
        if title not in body:
            fail(name, f"missing {title}")
    matches = re.findall(r"\]\(((?:references|evals)/[^)#]+)\)", body)
    if len(matches) < 2:
        fail(name, "expected references and evaluations linked directly")
    for relative in matches:
        if ".." in relative or not (folder / relative).is_file():
            fail(name, f"broken or unsafe reference: {relative}")
    eval_file = folder / "evals" / "scenarios.json"
    try:
        evals = json.loads(eval_file.read_text(encoding="utf-8"))
        if evals.get("skill") != name:
            fail(name, "evals.skill mismatch")
        cases = evals["cases"]
        if not isinstance(cases, list) or len(cases) < 3:
            fail(name, "requires >=3 evaluation scenarios")
        else:
            ids = [case.get("id") for case in cases]
            if len(set(ids)) != len(ids):
                fail(name, "duplicated case id")
            for case in cases:
                if not isinstance(case.get("description", case.get("task")), str):
                    fail(name, f"missing case scenario: {case.get('id')}")
                if name == "multi-agent-routing":
                    if not isinstance(case.get("expected_valid"), bool):
                        fail(name, "routing case lacks expected_valid")
                    if not isinstance(case.get("plan"), dict):
                        fail(name, "routing case lacks plan")
                else:
                    if not case.get("expected") or not case.get("negative"):
                        fail(name, "rubric must include positive and negative expectations")
    except (OSError, ValueError, KeyError, TypeError) as exc:
        fail(name, f"bad evaluation fixture: {type(exc).__name__}")
    if not (ROOT / "docs" / "skills" / f"{name}.md").is_file():
        fail(name, "missing corresponding manual")

if errors:
    for error in errors:
        print("ERROR:", error)
    sys.exit(1)
print(f"OK: {len(PILOTS)} advanced Skills with references and >=3 eval cases each")
