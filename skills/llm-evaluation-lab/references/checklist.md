# llm-evaluation-lab — critérios e antipadrões

## Gatilhos
Comparação de prompts/modelos, experimentos com agents, lançamento ou modificação de Skill.

## Contexto necessário
Objetivo, tarefas representativas, dados de referência, distribuição de falhas e budgets.

## Critérios
Evals precisam modelar o trabalho real. Testes de estrutura YAML/JSON não são evidência de melhora de agente. Evite benchmark contamination e otimização exclusiva para um dataset público.

## Procedimento de revisão
1. Fixe objetivo, população de tarefas, métricas primárias e limitações do avaliador.
2. Construa conjunto train/dev e holdout; evite que o agente veja respostas do holdout antes do teste.
3. Faça baseline sem intervenção e variante com Skill; mantenha modelos, temperatura quando aplicável e ambiente comparáveis.
4. Avalie sucesso por tarefa, tool-use válido, groundedness, segurança, tokens, custos e latência por distribuição e não apenas média.
5. Inspecione falhas reais, divergência entre juízes automáticos e humanos, e efeitos em tarefas fora da distribuição.
6. Publique relatório com exemplos positivos/negativos, intervalos/incertezas e critérios de promoção/rollback.

## Evidência exigida
Resultados devem ter origem verificável (dataset/version/hash quando relevante, teste executado com log, ou documento identificado). Sem ambiente executor, o trabalho permanece uma proposta.
