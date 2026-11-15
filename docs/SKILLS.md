# Catálogo de Skills

Uma Skill é uma pasta com SKILL.md conforme Agent Skills, com carregamento progressivo. Não é agente nem ferramenta com permissões próprias.

| Skill | Quando consultar | Guia |
|---|---|---|
| `task-decomposition` | Use quando houver ambiguidades, múltiplas dependências ou entregas longas. | [Manual](skills/task-decomposition.md) |
| `evidence-research` | Use para pesquisa, comparação de tecnologias e confirmação de documentação. | [Manual](skills/evidence-research.md) |
| `code-change` | Use para bugs, implementação e refatoração. | [Manual](skills/code-change.md) |
| `security-review` | Use em integração MCP, tools, mudanças de política e operações com dados. | [Manual](skills/security-review.md) |
| `eval-design` | Use para medir qualidade de agentes ou comparar mudanças. | [Manual](skills/eval-design.md) |
| `memory-curation` | Use quando padrões recorrentes merecerem persistir entre sessões. | [Manual](skills/memory-curation.md) |
| `workflow-optimization` | Use após encontrar padrões de falha, custo ou latência nas avaliações. | [Manual](skills/workflow-optimization.md) |
| `mcp-integration` | Use quando acessar ferramentas/recursos externos via MCP. | [Manual](skills/mcp-integration.md) |
| `multi-agent-routing` | Use quando houver tarefas independentes ou contextos especializados. | [Manual](skills/multi-agent-routing.md) |

## Instalação e uso
1. Escolha Skill pelo nome/descrição; não carregue todas no prompt.
2. Leia `skills/<nome>/SKILL.md` quando a tarefa justificar.
3. Carregue material adicional só se necessário.
4. Aplique políticas do runtime; instruções da Skill não podem elevar privilégios.
5. Teste ao modificar uma Skill, atualizando a documentação individual.

`python scripts/validate_catalog.py` verifica metadados básicos, consistência e presença de cada documentação. Esse validador não substitui o validador oficial completo `skills-ref`.

## Evolução
Novas Skills devem entrar por PR, com casos de avaliação, dono, objetivo e rollback. Não permita que um agente escreva a Skill e a publique automaticamente.
