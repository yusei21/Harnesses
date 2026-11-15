# ml-experiment-design — critérios e antipadrões

## Gatilhos
Projeto de pesquisa, comparação de algoritmos, escolha de métricas ou resultado experimental contestável.

## Contexto necessário
Pergunta científica, dados disponíveis, restrições computacionais, métricas e hipótese falsificável.

## Critérios
Inclua baseline não trivial, teste final reservado, protocolo estatístico proporcional à natureza dos dados e distinção entre tuning e avaliação. Reprodutibilidade não equivale a determinismo bit a bit em toda GPU.

## Procedimento de revisão
1. Defina hipótese e efeito esperado; separe observação exploratória de resultado confirmatório.
2. Estabeleça baseline forte, dataset/versionamento, splits sem overlap e protocolo de seleção de hiperparâmetros separado do teste final.
3. Escolha métricas primárias e secundárias, unidade de análise, intervalos de confiança e avaliações por subconjunto; não escolha a métrica após observar o resultado.
4. Planeje sementes múltiplas ou repetições quando a variância importa; registre hardware, versões, orçamento e procedimento de reprodução.
5. Defina ablações e controles negativos; registre falhas, resultados nulos e limites de generalização.
6. Entregue tabela de experimentos e critérios de aceite sem inventar resultados.

## Evidência exigida
Resultados devem ter origem verificável (dataset/version/hash quando relevante, teste executado com log, ou documento identificado). Sem ambiente executor, o trabalho permanece uma proposta.
