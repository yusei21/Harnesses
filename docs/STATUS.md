# Estado dos recursos

| Recurso | Situação | Evidência |
|---|---|---|
| Contratos de agentes | Implementado (declarativo) | agents/*.json |
| Skills portáveis | Implementado (conteúdo) | skills/*/SKILL.md |
| Documentação individual | Implementado | docs/agents e docs/skills |
| CLI offline e artefatos simulados | Implementado | harness/cli.py |
| Seleção simples de especialistas | Implementado (heurística) | harness/core.py |
| Política de bloqueio e aprovação de teste | Implementado (somente offline) | harness/core.py |
| Traces mínimos | Implementado (em memória) | trace_envelope |
| Evals de roteamento e policy | Implementado | evals + testes |
| LLM real e output schemas robustos | Interface planejada | harness/adapters.py |
| MCP / A2A reais | Interfaces e guias | docs/INTEGRATIONS.md |
| Execução paralela | Planejado | docs/ORCHESTRATION.md |
| Sandbox e shell | Planejado | docs/SECURITY.md |
| Aprovação autenticada | Planejado | docs/SECURITY.md |
| Persistência de checkpoints | Interface planejada | harness/adapters.py |
| Memória e dreaming | Protocolo proposto | docs/EVOLUTION.md |
| Autoevolução por PR | Fluxo proposto | docs/EVOLUTION.md |
| Evals end-to-end com LLM | Planejado | docs/EVOLUTION.md |

Não usar esta foundation para operações com credenciais, dados sensíveis ou infraestrutura de produção.
