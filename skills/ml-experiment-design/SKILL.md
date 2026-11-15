---
name: ml-experiment-design
description: Planejar experimentos científicos e de ML com hipóteses, baselines, splits, métricas, ablações e reprodutibilidade. Use ao avaliar uma nova arquitetura, comparar modelos ou preparar resultados acadêmicos.
---

# ml-experiment-design

## Quando usar
Projeto de pesquisa, comparação de algoritmos, escolha de métricas ou resultado experimental contestável.

## Entrada
Pergunta científica, dados disponíveis, restrições computacionais, métricas e hipótese falsificável.

## Procedimento
1. Defina hipótese e efeito esperado; separe observação exploratória de resultado confirmatório.
2. Estabeleça baseline forte, dataset/versionamento, splits sem overlap e protocolo de seleção de hiperparâmetros separado do teste final.
3. Escolha métricas primárias e secundárias, unidade de análise, intervalos de confiança e avaliações por subconjunto; não escolha a métrica após observar o resultado.
4. Planeje sementes múltiplas ou repetições quando a variância importa; registre hardware, versões, orçamento e procedimento de reprodução.
5. Defina ablações e controles negativos; registre falhas, resultados nulos e limites de generalização.
6. Entregue tabela de experimentos e critérios de aceite sem inventar resultados.

## Referências
Consulte [critérios e antipadrões](references/checklist.md) apenas se precisar revisar pontos detalhados. Avaliações: [casos](evals/scenarios.json).

## Saída
Entregue decisões, evidências disponíveis, critérios de aceite, lacunas e testes marcados como EXECUTADOS ou NÃO EXECUTADOS. Nunca invente acesso a dados, arquivos, ferramentas ou resultados.

## Limites
A Skill orienta, mas não habilita ferramentas, não concede permissões e não altera políticas do Rust. Não indexe segredos ou dados privados em terceiros sem autorização explícita.
