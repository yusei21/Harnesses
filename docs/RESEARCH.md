# Referências técnicas (15/11/2026)

- [Tokio runtime](https://docs.rs/tokio/latest/tokio/runtime/) — execução assíncrona, escalonamento e timers em Rust.
- [Rust async book](https://rust-lang.github.io/async-book/) — fundamentos de async e controle de tarefas.
- [Serde](https://serde.rs/) — serialização tipada Rust/JSON.
- [Rig](https://docs.rs/rig-core/) — opção de framework Rust para agentes; não é dependência atual.
- [rmcp](https://github.com/modelcontextprotocol/rust-sdk) — cliente/servidor MCP em Rust; não integrado.
- [Agent Skills](https://agentskills.io/specification) — Skills portáveis.
- [OpenAI Responses](https://platform.openai.com/docs/api-reference/responses) — conector Python existente.
- [Anthropic Messages](https://docs.anthropic.com/en/api/messages) — conector Python existente.
- [Gemini generateContent](https://ai.google.dev/api/generate-content) — conector Python existente.
- [Ollama API](https://docs.ollama.com/api/introduction) — modelo local.
- [LangGraph](https://docs.langchain.com/oss/python/langgraph/overview) — referência de checkpoints.
- [Temporal](https://docs.temporal.io/) — referência de execução durável.
- [DSPy](https://dspy.ai/) — experimentos e otimização com avaliações.

A escolha Rust-first não garante aceleração de inferência remota: o tempo normalmente é dominado por rede e geração do modelo. A migração privilegia controle de concorrência, tipagem e isolamento de responsabilidades. Não adicionar bibliotecas complexas até que o requisito seja comprovado.
