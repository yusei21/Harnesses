"""Restricted real HTTP inference for OpenAI, Anthropic, Gemini and local Ollama."""
import json
import os
import re
import urllib.request
import urllib.error
from dataclasses import dataclass

class ProviderError(RuntimeError): pass

@dataclass(frozen=True)
class Completion:
    provider: str
    model: str
    text: str
    input_tokens: int | None = None
    output_tokens: int | None = None

class NoRedirect(urllib.request.HTTPRedirectHandler):
    def redirect_request(self, req, fp, code, msg, headers, newurl):
        return None

def http_post(url, headers, payload, timeout):
    if url not in ("https://api.openai.com/v1/responses",
                   "https://api.anthropic.com/v1/messages",
                   "http://127.0.0.1:11434/api/chat") and not (
        url.startswith("https://generativelanguage.googleapis.com/v1beta/models/")
        and url.endswith(":generateContent")
    ):
        raise ProviderError("destination_not_allowed")
    data = json.dumps(payload).encode()
    if len(data) > 128000:
        raise ProviderError("request_too_large")
    opener = urllib.request.build_opener(urllib.request.ProxyHandler({}), NoRedirect())
    req = urllib.request.Request(url, data=data, method="POST",
        headers={"Content-Type": "application/json", **headers})
    try:
        with opener.open(req, timeout=timeout) as response:
            if response.status != 200:
                raise ProviderError("unexpected_status")
            raw = response.read(2000001)
    except urllib.error.HTTPError as e:
        raise ProviderError("http_" + str(e.code)) from None
    except (urllib.error.URLError, TimeoutError, OSError):
        raise ProviderError("network_error") from None
    if len(raw) > 2000000:
        raise ProviderError("response_too_large")
    try:
        result = json.loads(raw)
        if not isinstance(result, dict): raise ValueError()
        return result
    except (ValueError, UnicodeError):
        raise ProviderError("invalid_response_json") from None

def _key(name):
    key = os.environ.get(name, "").strip()
    if not key: raise ProviderError("missing_env:" + name)
    if "\r" in key or "\n" in key: raise ProviderError("invalid_env:" + name)
    return key

def generate(provider, model, instructions, prompt, *, transport=http_post,
             timeout=60.0, max_output_tokens=1024):
    if provider not in {"openai", "anthropic", "gemini", "ollama"}:
        raise ProviderError("unknown_provider")
    if not re.fullmatch(r"[A-Za-z0-9][A-Za-z0-9._:-]{0,127}", model) or ".." in model:
        raise ProviderError("invalid_model")
    if not 0 < timeout <= 180 or not 1 <= max_output_tokens <= 4096:
        raise ProviderError("invalid_limits")
    if not instructions.strip() or not prompt.strip() or len(instructions) + len(prompt) > 24000:
        raise ProviderError("invalid_context")
    if provider == "openai":
        url = "https://api.openai.com/v1/responses"
        headers = {"Authorization": "Bearer " + _key("OPENAI_API_KEY")}
        payload = {"model": model, "instructions": instructions, "input": prompt,
                   "max_output_tokens": max_output_tokens, "store": False}
    elif provider == "anthropic":
        url = "https://api.anthropic.com/v1/messages"
        headers = {"Authorization": "Bearer " + _key("ANTHROPIC_API_KEY"),
                   "anthropic-version": "2026-10-21"}
        payload = {"model": model, "system": instructions, "max_tokens": max_output_tokens,
                   "messages": [{"role": "user", "content": prompt}]}
    elif provider == "gemini":
        url = f"https://generativelanguage.googleapis.com/v1beta/models/{model}:generateContent"
        headers = {"x-goog-api-key": _key("GEMINI_API_KEY")}
        payload = {"systemInstruction": {"parts": [{"text": instructions}]},
                   "contents": [{"role": "user", "parts": [{"text": prompt}]}],
                   "generationConfig": {"maxOutputTokens": max_output_tokens}}
    else:
        url = "http://127.0.0.1:11434/api/chat"
        headers = {}
        payload = {"model": model, "stream": False,
                   "messages": [{"role": "system", "content": instructions},
                                {"role": "user", "content": prompt}],
                   "options": {"num_predict": max_output_tokens}}
    response = transport(url, headers, payload, timeout)
    try:
        if provider == "openai":
            text = "\n".join(c["text"] for m in response["output"] if m.get("type") == "message"
                   for c in m.get("content", []) if c.get("type") == "output_text")
            usage = response.get("usage") or {}
            inp, out = usage.get("input_tokens"), usage.get("output_tokens")
        elif provider == "anthropic":
            text = "\n".join(c["text"] for c in response["content"] if c.get("type") == "text")
            usage = response.get("usage") or {}
            inp, out = usage.get("input_tokens"), usage.get("output_tokens")
        elif provider == "gemini":
            text = "\n".join(p["text"] for c in response["candidates"]
                    for p in c["content"]["parts"] if "text" in p)
            usage = response.get("usageMetadata") or {}
            inp, out = usage.get("promptTokenCount"), usage.get("candidatesTokenCount")
        else:
            text = response["message"]["content"]
            inp, out = response.get("prompt_eval_count"), response.get("eval_count")
    except (KeyError, TypeError, ValueError):
        raise ProviderError("invalid_response_shape") from None
    if not isinstance(text, str) or not text.strip():
        raise ProviderError("empty_model_response")
    return Completion(provider, model, text[:24000], inp, out)
