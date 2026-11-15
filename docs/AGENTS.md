# Catálogo de agentes

Agentes são **papéis contratuais**, não processos em execução neste protótipo. O roteador offline em harness/core.py escolhe uma sequência demonstrativa. Runtime LLM, paralelismo e agentes remotos ainda precisam de adapters.

| Agente | O que faz | Ativação | Documentação |
|---|---|---|---|
| `planner` | Recebe objetivo e restrições; devolve plano, critérios de aceite, dependências e tarefas candidatas. | Inicie por ele somente em tarefas complexas. O orquestrador pode dispensá-lo em atividades simples. | [Guia](agents/planner.md) |
| `researcher` | Recebe perguntas e fontes autorizadas; devolve evidências citadas e incertezas. | Use quando decisões dependem de documentação, repositórios e pesquisa externa. | [Guia](agents/researcher.md) |
| `engineer` | Recebe requisitos e contexto de código; devolve proposta de patch e testes. | Use para implementação/refatoração; em produção conecte sandbox controlado. | [Guia](agents/engineer.md) |
| `reviewer` | Recebe diff, requisitos e testes; devolve achados priorizados por severidade. | Use após mudanças técnicas, idealmente sem compartilhar o contexto inteiro do executor. | [Guia](agents/reviewer.md) |
| `evaluator` | Recebe baseline, candidato, casos de avaliação e rubrica; devolve métricas e regressões. | Use antes de promover mudanças de prompts, Skills e workflows. | [Guia](agents/evaluator.md) |
| `memory-curator` | Recebe traces sanitizados e evidências; propõe memórias com fontes e validade. | Use em consolidação periódica ou após incidentes resolvidos. | [Guia](agents/memory-curator.md) |
| `optimizer` | Recebe falhas agregadas e baseline; propõe alteração versionada com experimento. | Use quando há evidência repetível de problema de qualidade, custo ou latência. | [Guia](agents/optimizer.md) |

## Contrato
Cada `agents/<nome>.json` declara nome, entradas, saídas, triggers, ferramentas pretendidas, risco e documentação. Contexto mínimo, evidências e limites são obrigatórios. O agente de revisão deve ter contexto independente do engenheiro.

## Roteamento
O processo preferido é agente único -> agente como ferramenta -> delegação especializada -> paralelismo limitado. Não implementar agentes conversando indefinidamente. Cada delegado recebe objetivo, budget, limite de passos, ferramentas permitidas e esquema de saída.

## Experimente
`python -m harness.cli catalog` lista agentes; `python -m harness.cli demo --task "Investigar bug na API"` simula fluxo sem LLM. Para executar agentes reais, implemente adapters conforme docs/INTEGRATIONS.md.
