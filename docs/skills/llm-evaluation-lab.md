# Skill: llm-evaluation-lab

**Departamento:** ai

**O que faz:** Criar benchmarks para LLMs e agentes com baselines, rubricas, avaliação de ferramentas e regressão de custo e segurança. Use ao comparar modelos, prompts, Skills ou sistemas multiagentes.

**Quando ativar:** Comparação de prompts/modelos, experimentos com agents, lançamento ou modificação de Skill.

**Como utilizar:** registrar `skills/llm-evaluation-lab/SKILL.md` em cliente compatível ou selecionar pelo catálogo Rust com `harnesses-ops suggest`. O catálogo ainda não injeta automaticamente o arquivo no modelo. Leia as referências apenas quando forem necessárias.

**Entradas:** Objetivo, tarefas representativas, dados de referência, distribuição de falhas e budgets.

**Procedimento:**
1. Fixe objetivo, população de tarefas, métricas primárias e limitações do avaliador.
2. Construa conjunto train/dev e holdout; evite que o agente veja respostas do holdout antes do teste.
3. Faça baseline sem intervenção e variante com Skill; mantenha modelos, temperatura quando aplicável e ambiente comparáveis.
4. Avalie sucesso por tarefa, tool-use válido, groundedness, segurança, tokens, custos e latência por distribuição e não apenas média.
5. Inspecione falhas reais, divergência entre juízes automáticos e humanos, e efeitos em tarefas fora da distribuição.
6. Publique relatório com exemplos positivos/negativos, intervalos/incertezas e critérios de promoção/rollback.

**Saída:** relatório com decisões, evidências, métricas pertinentes, limites, critérios de aceite e estado real dos testes.

**Arquivos:** [Skill](../../skills/llm-evaluation-lab/SKILL.md), [referências](../../skills/llm-evaluation-lab/references/checklist.md), [rubricas](../../skills/llm-evaluation-lab/evals/scenarios.json).

**Avaliação:** 3 cenários com expectativas positivas e falhas proibidas. CI verifica integridade, mas não mede ainda qualidade do LLM.

**Limites:** nenhum script executado automaticamente, nenhum acesso a dados privados, sem autopromoção de mudanças.
