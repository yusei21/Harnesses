# Referências verificadas em 15/11/2026

- [OpenAI Responses API](https://developers.openai.com/api/reference/resources/responses/methods/create): `POST /v1/responses`, `instructions`, `input`, `store=false`.
- [Anthropic Messages API](https://platform.claude.com/docs/en/api/messages/create): `POST /v1/messages`, `anthropic-version: 2026-10-21`.
- [Gemini generateContent](https://ai.google.dev/api/generate-content): `POST /v1beta/models/{model}:generateContent`.
- [Gemini Interactions](https://ai.google.dev/gemini-api/docs/interactions-overview): recomendada desde junho/2026; migração futura.
- [Ollama API](https://docs.ollama.com/api/introduction): `POST /api/chat` via loopback.
- [Agent Skills](https://agentskills.io/specification): `SKILL.md`.
- [MCP](https://modelcontextprotocol.io/specification): ferramentas, ainda não integrado.
- [A2A](https://a2a-protocol.org/latest/): delegação remota, ainda não integrada.
- [LangGraph](https://docs.langchain.com/oss/python/langgraph/overview): inspiração para durabilidade.
- [Temporal](https://docs.temporal.io/): persistência de workflows.
- [DSPy](https://dspy.ai/): otimização via evals.
- [Letta](https://docs.letta.com/): memória persistente.

O runtime foi mantido pequeno e independente de SDKs; não equivale a um framework completo de execução autônoma.
