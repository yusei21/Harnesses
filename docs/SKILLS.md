# Catálogo de Skills

Skills são pastas portáveis no formato `SKILL.md`, com documentação individual. **Skills não são agentes, não concedem privilégios e não são ativadas automaticamente pelo núcleo Rust nesta versão.**

| Skill | Papel | Manual |
|---|---|---|
| task-decomposition | Planos e critérios de aceite | [Guia](skills/task-decomposition.md) |
| evidence-research | Evidências verificáveis | [Guia](skills/evidence-research.md) |
| code-change | Planejar patches, testes e rollback | [Guia](skills/code-change.md) |
| security-review | Revisão de riscos e acesso | [Guia](skills/security-review.md) |
| eval-design | Rubricas e benchmarks | [Guia](skills/eval-design.md) |
| memory-curation | Propostas de memória | [Guia](skills/memory-curation.md) |
| workflow-optimization | Experimentos de melhoria | [Guia](skills/workflow-optimization.md) |
| mcp-integration | Planejar conexões MCP | [Guia](skills/mcp-integration.md) |
| multi-agent-routing | Escolha entre 1 e vários agentes | [Guia](skills/multi-agent-routing.md) |

## Uso atual
1. Consulte a Skill correspondente e seu manual individual.
2. Em um cliente compatível com Agent Skills, registre a pasta e use o carregamento progressivo oferecido pelo cliente.
3. Para uso manual, leia `skills/<nome>/SKILL.md` antes de aplicar o procedimento.
4. O comando Rust `run` **não lê os SKILL.md automaticamente** nem invoca scripts. Isso exige um resolvedor controlado, testes e avaliação de segurança futuros.

Verifique o catálogo com `python scripts/validate_catalog.py`. Não instale Skills de terceiros ou execute scripts sem revisar origem, conteúdo e permissões.
