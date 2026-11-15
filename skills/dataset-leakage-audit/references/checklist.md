# dataset-leakage-audit — critérios e antipadrões

## Gatilhos
Construção de dataset, preparação de features, split temporal ou suspeita de métricas artificialmente elevadas.

## Contexto necessário
Origem e licença dos dados, identificadores, esquema, timestamps, estratégia de split e transformações de features.

## Critérios
Leakage pode ocorrer por alvo embutido em feature, pré-processamento global, entidades duplicadas e janelas temporais sobrepostas. Um bom teste replica construção de features usando apenas histórico disponível no momento da previsão.

## Procedimento de revisão
1. Rastreie proveniência, autorização de uso, consentimento quando aplicável e sensibilidade dos dados.
2. Identifique a unidade de independência: usuário, paciente, documento, dispositivo ou janela temporal; divida com base nessa unidade.
3. Verifique near-duplicates, entidades compartilhadas e contaminação treino/validação/teste, inclusive pré-treinamento se houver evidência.
4. Confirme que normalização, imputação, seleção de features e geração de embeddings foram ajustadas somente no treino quando exigido.
5. Em dados temporais, elimine features construídas com informação futura; audite point-in-time joins.
6. Defina relatório com evidência reproduzível, criticidade, impacto na métrica e plano de correção sem alterar dados automaticamente.

## Evidência exigida
Resultados devem ter origem verificável (dataset/version/hash quando relevante, teste executado com log, ou documento identificado). Sem ambiente executor, o trabalho permanece uma proposta.
