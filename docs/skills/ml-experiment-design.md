# Skill: ml-experiment-design

**Departamento:** machine-learning

**O que faz:** Planejar experimentos científicos e de ML com hipóteses, baselines, splits, métricas, ablações e reprodutibilidade. Use ao avaliar uma nova arquitetura, comparar modelos ou preparar resultados acadêmicos.

**Quando ativar:** Projeto de pesquisa, comparação de algoritmos, escolha de métricas ou resultado experimental contestável.

**Como utilizar:** registrar `skills/ml-experiment-design/SKILL.md` em cliente compatível ou selecionar pelo catálogo Rust com `harnesses-ops suggest`. O catálogo ainda não injeta automaticamente o arquivo no modelo. Leia as referências apenas quando forem necessárias.

**Entradas:** Pergunta científica, dados disponíveis, restrições computacionais, métricas e hipótese falsificável.

**Procedimento:**
1. Defina hipótese e efeito esperado; separe observação exploratória de resultado confirmatório.
2. Estabeleça baseline forte, dataset/versionamento, splits sem overlap e protocolo de seleção de hiperparâmetros separado do teste final.
3. Escolha métricas primárias e secundárias, unidade de análise, intervalos de confiança e avaliações por subconjunto; não escolha a métrica após observar o resultado.
4. Planeje sementes múltiplas ou repetições quando a variância importa; registre hardware, versões, orçamento e procedimento de reprodução.
5. Defina ablações e controles negativos; registre falhas, resultados nulos e limites de generalização.
6. Entregue tabela de experimentos e critérios de aceite sem inventar resultados.

**Saída:** relatório com decisões, evidências, métricas pertinentes, limites, critérios de aceite e estado real dos testes.

**Arquivos:** [Skill](../../skills/ml-experiment-design/SKILL.md), [referências](../../skills/ml-experiment-design/references/checklist.md), [rubricas](../../skills/ml-experiment-design/evals/scenarios.json).

**Avaliação:** 3 cenários com expectativas positivas e falhas proibidas. CI verifica integridade, mas não mede ainda qualidade do LLM.

**Limites:** nenhum script executado automaticamente, nenhum acesso a dados privados, sem autopromoção de mudanças.
