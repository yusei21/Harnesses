---
name: llm-evaluation-lab
description: Criar benchmarks para LLMs e agentes com baselines, rubricas, avaliação de ferramentas e regressão de custo e segurança. Use ao comparar modelos, prompts, Skills ou sistemas multiagentes.
---

# llm-evaluation-lab

## Quando usar
Comparação de prompts/modelos, experimentos com agents, lançamento ou modificação de Skill.

## Entrada
Objetivo, tarefas representativas, dados de referência, distribuição de falhas e budgets.

## Procedimento
1. Fixe objetivo, população de tarefas, métricas primárias e limitações do avaliador.
2. Construa conjunto train/dev e holdout; evite que o agente veja respostas do holdout antes do teste.
3. Faça baseline sem intervenção e variante com Skill; mantenha modelos, temperatura quando aplicável e ambiente comparáveis.
4. Avalie sucesso por tarefa, tool-use válido, groundedness, segurança, tokens, custos e latência por distribuição e não apenas média.
5. Inspecione falhas reais, divergência entre juízes automáticos e humanos, e efeitos em tarefas fora da distribuição.
6. Publique relatório com exemplos positivos/negativos, intervalos/incertezas e critérios de promoção/rollback.

## Referências
Consulte [critérios e antipadrões](references/checklist.md) apenas se precisar revisar pontos detalhados. Avaliações: [casos](evals/scenarios.json).

## Saída
Entregue decisões, evidências disponíveis, critérios de aceite, lacunas e testes marcados como EXECUTADOS ou NÃO EXECUTADOS. Nunca invente acesso a dados, arquivos, ferramentas ou resultados.

## Limites
A Skill orienta, mas não habilita ferramentas, não concede permissões e não altera políticas do Rust. Não indexe segredos ou dados privados em terceiros sem autorização explícita.
