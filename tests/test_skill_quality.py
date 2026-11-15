import json
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
PILOTS = ["multi-agent-routing", "code-change", "rust-quality-gate"]

class SkillQualityTests(unittest.TestCase):
    def test_pilot_skills_have_real_evaluation_scenarios(self):
        for name in PILOTS:
            with self.subTest(skill=name):
                content = json.loads(
                    (ROOT / "skills" / name / "evals" / "scenarios.json").read_text(encoding="utf-8")
                )
                self.assertEqual(content["skill"], name)
                self.assertGreaterEqual(len(content["cases"]), 3)
                self.assertEqual(len({case["id"] for case in content["cases"]}),len(content["cases"]))

    def test_catalog_entries_match_skills(self):
        catalog = json.loads((ROOT / "capabilities/catalog.json").read_text(encoding="utf-8"))
        mapping = {cap["id"]: cap for cap in catalog["capabilities"]}
        for name in PILOTS:
            self.assertEqual(mapping[name]["path"], f"skills/{name}/SKILL.md")
            self.assertEqual(mapping[name]["kind"], "skill")

if __name__ == "__main__":
    unittest.main()
