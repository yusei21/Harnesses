# Catálogo de Skills

As Skills seguem pastas `skills/<nome>/SKILL.md` e possuem documentação `docs/skills/<nome>.md`. São procedimentos reutilizáveis; não são agentes nem ferramentas que concedem acesso à máquina.

| Skill | O que oferece | Manual |
|---|---|---|
| `task-decomposition` | Planejamento e critérios de aceite | [Guia](skills/task-decomposition.md) |
| `evidence-research` | Pesquisa e análise de evidências | [Guia](skills/evidence-research.md) |
| `code-change` | Patches pequenos, testes e rollback | [Guia](skills/code-change.md) |
| `security-review` | Threat modeling e revisão de permissões | [Guia](skills/security-review.md) |
| `eval-design` | Conjuntos de testes e rubricas | [Guia](skills/eval-design.md) |
| `memory-curation` | Propostas de memória com proveniência | [Guia](skills/memory-curation.md) |
| `workflow-optimization` | Propostas de experimentos controlados | [Guia](skills/workflow-optimization.md) |
| `mcp-integration` | Contratos de conectores MCP | [Guia](skills/mcp-integration.md) |
| `multi-agent-routing` | Critérios para delegação e paralelismo | [Guia](skills/multi-agent-routing.md) |

## Como usar
1. Examine o catálogo e selecione apenas a Skill pertinente à tarefa.
2. Em clientes compatíveis com Agent Skills, registre a pasta correspondente e use seu mecanismo de ativação/carregamento progressivo.
3. Sem cliente compatível, leia manualmente `skills/<nome>/SKILL.md` e siga as orientações.
4. Edite a Skill e seu manual juntos; valide com `python scripts/validate_catalog.py`.

**Atenção:** o comando `python -m harness.cli run` executa agentes, mas **não ativa automaticamente Skills**. Esse carregador sob demanda ainda não foi implementado. O runtime não executa scripts dentro de Skills e não fornece credenciais a essas instruções.

Modificar Skills deve ocorrer via PR, com testes e revisão. Não habilite scripts remotos como ferramentas sem verificar origem e permissões.
