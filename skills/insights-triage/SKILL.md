---
name: insights-triage
description: Analisar eventos locais sanitizados e sugerir Skills, hooks ou refatorações.
---

# insights triage

Use após múltiplas falhas ou padrões repetidos de trabalho.

## Procedimento
1. Recolha apenas eventos autorizados, sem prompts, credenciais ou conteúdo privado.
2. Execute insights no Rust sobre o arquivo JSON explícito.
3. Considere recorrência e custo; evite recomendar Skills para um único evento.
4. Gere proposta com evidências, métrica de melhora e plano de rollback.
5. Não instale hooks nem altere permissões automaticamente.

## Referências
- [Manual](../../docs/skills/insights-triage.md)

## Limites
Esta Skill é conhecimento procedimental, não um grant de ferramenta. Não ativa scripts nem permissões por conta própria.
