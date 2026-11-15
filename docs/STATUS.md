# Estado da implementação (runtime local multi-provedor)

| Componente | Estado |
|---|---|
| 7 contratos de agentes e documentação individual | Implementado |
| 9 Skills com SKILL.md e documentação individual | Implementado |
| CLI offline e CI | Implementado |
| OpenAI, Anthropic, Gemini e Ollama via HTTP | Implementado; testes com mock |
| Multiagente sequencial com overrides de provedores | Implementado (somente texto) |
| Consentimento para cloud e destinos HTTP fechados | Implementado |
| Limites de chamadas/tamanho e timeouts | Implementado |
| Contabilidade de custo monetário | Não implementado |
| MCP, A2A, ferramentas e shell | Não implementado |
| Sandbox de execução, autenticação de approvals | Não implementado |
| Checkpoints duráveis, memória persistente, dreaming | Não implementado |
| Autoevolução e promoção de Skills | Não implementado |
| Evals end-to-end com modelos reais | Não implementado |

Leia docs/PROVIDERS.md. Chamar modelos realmente não significa executar ações locais ou remotas.
