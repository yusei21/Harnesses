# Estado implementado — Rust-first

| Capacidade | Estado | Arquivo |
|---|---|---|
| Núcleo de orquestração Rust | Implementado, testes de unidade | rust/src/core.rs |
| Stages sequenciais com fan-out paralelo limitado | Implementado, 3 simultâneos | rust/src/core.rs |
| Limites de chamadas e consentimento antecipado | Implementado, não financeiro | rust/src/core.rs |
| Regras fail-closed para ações com efeito colateral | Implementado como política; **sem ferramentas** | rust/src/core.rs |
| Ponte Rust → Python via JSON | Implementada | rust/src/bridge.rs + harness/bridge.py |
| OpenAI, Anthropic, Gemini e Ollama | Integração Python existente, tests mockados | harness/providers.py |
| 7 agentes catalogados/documentados | Implementado | agents + docs/agents |
| 9 Skills catalogadas/documentadas | Implementado; sem seleção dinâmica | skills + docs/skills |
| CI Python/Rust | Implementado | .github/workflows/ci.yml |
| Tools/MCP/A2A | Não implementado | docs/INTEGRATIONS.md |
| Checkpoints persistentes | Não implementado | docs/EVOLUTION.md |
| Shell, sandbox e edição de arquivos | Não implementado | docs/SECURITY.md |
| Hooks autenticados / approvals | Não implementado | docs/SECURITY.md |
| Memória persistente, dreaming, autoevolução | Não implementado | docs/EVOLUTION.md |
| Limites monetários por token | Não implementado | docs/PROVIDERS.md |

As rotas HTTP têm testes de contrato simulados, não testes de conta de produção. A ponte não equivale a processo isolado. Não use como agente de programação autônomo irrestrito.
