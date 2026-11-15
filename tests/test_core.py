import unittest
from harness.core import Assignment, Orchestrator, Policy, Risk, Status, Task, choose_assignments, trace_envelope
from harness.evaluation import evaluate

class HarnessTests(unittest.TestCase):
    def test_routes_research(self):
        self.assertIn("researcher", [a.agent for a in choose_assignments("Pesquisar docs")])

    def test_offline_execution(self):
        r = Orchestrator().run(Task("t1", "Pesquisar docs"))
        self.assertEqual(r.status, Status.COMPLETED)
        self.assertTrue(all(a["kind"] == "offline-demonstration" for a in r.artifacts))

    def test_fail_closed(self):
        r = Orchestrator().run(Task("t2", "demo"), assignments=[
            Assignment("engineer", "danger", Risk.DESTRUCTIVE)])
        self.assertEqual(r.status, Status.APPROVAL_REQUIRED)
        self.assertEqual(len(r.artifacts), 0)

    def test_budget_and_step_contract(self):
        with self.assertRaises(ValueError):
            Task("t3", " ", budget_usd=-1)
        r = Orchestrator().run(Task("t4", "demo", max_steps=1),
                               assignments=[Assignment("a", "a"), Assignment("b", "b")])
        self.assertEqual(r.status, Status.REJECTED)

    def test_trace_excludes_prompt(self):
        secret = "SENSITIVE_DO_NOT_LOG"
        r = Orchestrator().run(Task("t5", secret))
        self.assertNotIn(secret, str(trace_envelope(r)))

    def test_eval(self):
        self.assertTrue(evaluate()["passed"])

if __name__ == "__main__":
    unittest.main()
