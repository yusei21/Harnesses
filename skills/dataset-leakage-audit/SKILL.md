---
name: dataset-leakage-audit
description: Auditar datasets e pipelines ML contra vazamento de informação, duplicação entre splits, viés de amostragem e problemas de proveniência. Use antes de treinar, validar ou publicar resultados de modelos.
---

# dataset-leakage-audit

## Quando usar
Construção de dataset, preparação de features, split temporal ou suspeita de métricas artificialmente elevadas.

## Entrada
Origem e licença dos dados, identificadores, esquema, timestamps, estratégia de split e transformações de features.

## Procedimento
1. Rastreie proveniência, autorização de uso, consentimento quando aplicável e sensibilidade dos dados.
2. Identifique a unidade de independência: usuário, paciente, documento, dispositivo ou janela temporal; divida com base nessa unidade.
3. Verifique near-duplicates, entidades compartilhadas e contaminação treino/validação/teste, inclusive pré-treinamento se houver evidência.
4. Confirme que normalização, imputação, seleção de features e geração de embeddings foram ajustadas somente no treino quando exigido.
5. Em dados temporais, elimine features construídas com informação futura; audite point-in-time joins.
6. Defina relatório com evidência reproduzível, criticidade, impacto na métrica e plano de correção sem alterar dados automaticamente.

## Referências
Consulte [critérios e antipadrões](references/checklist.md) apenas se precisar revisar pontos detalhados. Avaliações: [casos](evals/scenarios.json).

## Saída
Entregue decisões, evidências disponíveis, critérios de aceite, lacunas e testes marcados como EXECUTADOS ou NÃO EXECUTADOS. Nunca invente acesso a dados, arquivos, ferramentas ou resultados.

## Limites
A Skill orienta, mas não habilita ferramentas, não concede permissões e não altera políticas do Rust. Não indexe segredos ou dados privados em terceiros sem autorização explícita.
