import unittest
from harness.live import run_live, Route, LiveError, parse_overrides
from harness.providers import Completion

class Tests(unittest.TestCase):
    def test_chain(self):
        calls=[]
        def fake(p,m,i,q):
            calls.append((p,m,i,q))
            return Completion(p,m,"resp"+str(len(calls)))
        output=run_live("Review",("planner","engineer","reviewer"),
            Route("ollama","qwen3"),call=fake)
        self.assertEqual(output["final"],"resp3")
        self.assertIn("resp1",calls[1][3])
        self.assertEqual(len(output["events"]),3)
    def test_cross_provider(self):
        calls=[]
        def fake(p,m,i,q):
            calls.append(p)
            return Completion(p,m,"ok")
        run_live("Review",("planner","reviewer"),Route("ollama","qwen3"),
            {"reviewer":Route("gemini","model")},allow_cloud=True,max_calls=2,call=fake)
        self.assertEqual(calls,["ollama","gemini"])
    def test_requires_cloud_consent_prior_to_call(self):
        with self.assertRaisesRegex(LiveError,"cloud_requires_allow_cloud"):
            run_live("Review",("planner","reviewer"),Route("ollama","qwen3"),
                {"reviewer":Route("openai","test")},max_calls=2,
                call=lambda *x: self.fail("should not call"))
    def test_limits(self):
        with self.assertRaises(LiveError):
            run_live("t",("planner","planner"),Route("ollama","qwen3"))
        with self.assertRaises(LiveError):
            run_live("t",("planner","reviewer"),Route("ollama","qwen3"),max_calls=1)
    def test_overrides(self):
        self.assertEqual(parse_overrides(["reviewer=openai:m"],("reviewer",))["reviewer"].provider,"openai")
if __name__ == "__main__": unittest.main()
