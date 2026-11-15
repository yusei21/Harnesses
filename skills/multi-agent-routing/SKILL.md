---
name: multi-agent-routing
description: Escolhe entre agente único, agentes como ferramentas, handoff e etapas paralelas para tarefas de engenharia com subtarefas, dependências ou especialistas distintos. Use quando for necessário criar ou revisar um plano multiagente, reduzir custo de delegação ou controlar concorrência.
---

# Multi-agent routing

## Entrada
Objetivo e critérios de aceite; subtarefas e suas dependências; agentes, ferramentas e permissões disponíveis; orçamento de chamadas e concorrência.

## Procedimento
1. Identifique a menor unidade de trabalho verificável. Se uma única chamada resolve, **não delegue**.
2. Separe dependências reais das tarefas independentes. Uma etapa com leitura de arquivos compartilhados pode paralelizar; alterações simultâneas no mesmo worktree, não.
3. Escolha o padrão pela [matriz de decisão](references/decision-matrix.md). Prefira a topologia de menor complexidade com evidência de ganho.
4. Para etapas paralelas, defina entradas independentes, limites de fan-out, prazo, erros e ordem estável dos resultados. Considere custos de integração e revisão.
5. Produza um plano com `goal`, `stages`, rotas, `max_calls`, `max_parallel` e `allow_cloud` conforme [contrato](references/execution-contract.md).
6. Antes de executar, valide o plano com a política **do runtime Rust**. Se a operação requer ferramentas ou permissões não implementadas, retorne um bloqueio explícito, sem inventar execução.
7. Compare duração, custo, completude e erros com a opção de agente único. Sem evidência, registre a escolha como hipótese.

## Saída
Plano de execução proposto; justificativa de delegação; dependências, limites, riscos e critérios de aceite. **Não** alegue que gerou ou executou subagentes se apenas escreveu o plano.

## Casos de teste
Consulte [cenários](evals/scenarios.json) antes de alterar este procedimento. Os casos de rota verificam contratos Rust, não a qualidade das decisões de um LLM.

## Não usar
Não use para uma pergunta simples sem dependências. Uma Skill não altera permissões, escalonador ou ferramentas do host.
