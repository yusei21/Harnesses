import os
import unittest
from unittest.mock import patch
from harness.providers import generate, ProviderError, http_post

class Tests(unittest.TestCase):
    def setUp(self):
        p = patch.dict(os.environ, {"OPENAI_API_KEY":"key-o",
            "ANTHROPIC_API_KEY":"key-a","GEMINI_API_KEY":"key-g"})
        p.start()
        self.addCleanup(p.stop)
    def test_openai(self):
        def fake(url,headers,body,timeout):
            self.assertEqual(url,"https://api.openai.com/v1/responses")
            self.assertEqual(body["store"],False)
            self.assertEqual(headers["Authorization"],"Bearer key-o")
            return {"output":[{"type":"message","content":[{"type":"output_text","text":"ok"}]}]}
        self.assertEqual(generate("openai","model","system","prompt",transport=fake).text,"ok")
    def test_anthropic(self):
        def fake(url,headers,body,timeout):
            self.assertEqual(url,"https://api.anthropic.com/v1/messages")
            self.assertEqual(headers["anthropic-version"],"2026-10-21")
            return {"content":[{"type":"text","text":"ok"}]}
        self.assertEqual(generate("anthropic","model","system","prompt",transport=fake).text,"ok")
    def test_gemini(self):
        def fake(url,headers,body,timeout):
            self.assertTrue(url.endswith("/models/model:generateContent"))
            self.assertEqual(headers["x-goog-api-key"],"key-g")
            return {"candidates":[{"content":{"parts":[{"text":"ok"}]}}]}
        self.assertEqual(generate("gemini","model","system","prompt",transport=fake).text,"ok")
    def test_ollama(self):
        def fake(url,headers,body,timeout):
            self.assertEqual(url,"http://127.0.0.1:11434/api/chat")
            self.assertFalse(body["stream"])
            return {"message":{"content":"ok"}}
        self.assertEqual(generate("ollama","model","system","prompt",transport=fake).text,"ok")
    def test_no_key(self):
        with patch.dict(os.environ, {"OPENAI_API_KEY":""}):
            with self.assertRaisesRegex(ProviderError,"missing_env"):
                generate("openai","model","s","p")
    def test_model_is_not_url(self):
        with self.assertRaisesRegex(ProviderError,"invalid_model"):
            generate("gemini","../evil","s","p")
    def test_destination_rejected(self):
        with self.assertRaisesRegex(ProviderError,"destination_not_allowed"):
            http_post("https://evil.example/",{},{},1)
    def test_errors(self):
        with self.assertRaisesRegex(ProviderError,"invalid_response_shape"):
            generate("ollama","test","s","p",transport=lambda *args: {})
        with self.assertRaisesRegex(ProviderError,"invalid_context"):
            generate("ollama","test","s","x"*24001)
if __name__ == "__main__": unittest.main()
