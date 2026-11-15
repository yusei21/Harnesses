import json
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]

class DomainSkillTests(unittest.TestCase):
    def test_domain_catalog_and_scenarios(self):
        catalog = json.loads((ROOT / "capabilities/catalog.json").read_text(encoding="utf-8"))
        entries = {item["id"]: item for item in catalog["capabilities"]}
        expected = {
            "ml-experiment-design":"machine-learning",
            "dataset-leakage-audit":"machine-learning",
            "pytorch-training-review":"machine-learning",
            "rag-pipeline-evaluation":"rag",
            "llm-evaluation-lab":"ai",
            "saas-system-design":"saas",
            "frontend-interface-review":"frontend",
            "backend-api-contracts":"backend",
            "saas-security-threat-model":"security",
        }
        for slug, domain in expected.items():
            with self.subTest(slug=slug):
                self.assertEqual(entries[slug]["domain"], domain)
                skill = ROOT / "skills" / slug
                cases = json.loads((skill / "evals/scenarios.json").read_text(encoding="utf-8"))["cases"]
                self.assertGreaterEqual(len(cases), 3)
                self.assertTrue(all(x["expected"] and x["negative"] for x in cases))
                self.assertTrue((ROOT / "docs" / "skills" / (slug + ".md")).is_file())

if __name__ == "__main__":
    unittest.main()
