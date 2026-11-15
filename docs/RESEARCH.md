# Pesquisa e referências

Consulta de documentação pública: 15/11/2026. Verifique versões e especificações antes de implementar adapters.

- [OpenAI — Building agents](https://developers.openai.com/tracks/building-agents) — handoffs, guardrails, sessões e tracing.
- [OpenAI — Agents SDK](https://developers.openai.com/api/docs/guides/agents/sdk) — orquestração gerenciada em código.
- [Agent Skills — spec](https://agentskills.io/specification) — estrutura portável de SKILL.md.
- [LangChain — Deep Agents](https://docs.langchain.com/oss/python/deepagents/overview) — filesystem, tarefas e subagentes.
- [Google ADK](https://google.github.io/adk-docs/) — workflows e agentes.
- [A2A](https://a2a-protocol.org/latest/) — interoperabilidade entre agentes.
- [MCP](https://modelcontextprotocol.io/specification) — conexão com ferramentas e recursos.
- [Microsoft Agent Framework](https://learn.microsoft.com/en-us/agent-framework/) — agentes, workflows, estado.
- [Temporal](https://docs.temporal.io/) — durable execution.
- [DSPy](https://dspy.ai/) — otimização baseada em exemplos e métricas.
- [Letta](https://docs.letta.com/) — memória persistente.
- [OpenHands](https://docs.openhands.dev/) — agentes de desenvolvimento em ambientes executáveis.
- [MCP security practices](https://modelcontextprotocol.io/specification/2025-06-18/basic/security_best_practices) — segurança e identidade (consulte a versão vigente).

## Síntese
Não adotar tudo de uma vez. O harness deve permanecer independente do framework até que um requisito concreto demande LangGraph, ADK, Microsoft ou outro. A2A não é necessário para subagentes locais. MCP não é sinônimo de autorizar qualquer ferramenta. Autoevolução necessita evals, aprovações e rollback.
