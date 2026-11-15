# Estado implementado — Linux local-first

| Capacidade | Situação |
|---|---|
| Orquestração Rust + Tokio e ponte Python | Implementado, texto apenas |
| Provedores OpenAI/Anthropic/Gemini/Ollama | Conectores HTTP existentes; contratos testados com mocks |
| 7 agentes e guias individuais | Implementado (papéis textuais) |
| 12 Skills e documentação individual | Implementado (conhecimento) |
| Registro de capacidades por domínio | Implementado, JSON + CLI Rust |
| Seleção de Skill por gatilhos | Implementado, heurística de metadados |
| Auditoria estrutural/links/duplicatas e docs antigas | Implementado, somente leitura |
| Insights sobre observações fornecidas | Implementado, sugestões de regras por recorrência |
| CI Rust/Python + auditoria | Implementado |
| Ativação automática de Skills | Não implementado |
| Auditoria semântica de contradições | Não implementado |
| Leitura automática de sessões dos CLIs | Não implementado |
| RAG, AST/code graph e memória persistente | Não implementado |
| Hooks, ferramentas MCP, A2A, sandbox e aprovação real | Não implementado |
| Evals de qualidade de LLM e autoevolução | Não implementado |

A execução dos modelos não tem acesso ao filesystem ou a shell; `insights` não altera arquivos. O auditor não deve ser interpretado como teste completo de segurança.
