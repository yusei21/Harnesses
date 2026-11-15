# Skill: dataset-leakage-audit

**Departamento:** machine-learning

**O que faz:** Auditar datasets e pipelines ML contra vazamento de informação, duplicação entre splits, viés de amostragem e problemas de proveniência. Use antes de treinar, validar ou publicar resultados de modelos.

**Quando ativar:** Construção de dataset, preparação de features, split temporal ou suspeita de métricas artificialmente elevadas.

**Como utilizar:** registrar `skills/dataset-leakage-audit/SKILL.md` em cliente compatível ou selecionar pelo catálogo Rust com `harnesses-ops suggest`. O catálogo ainda não injeta automaticamente o arquivo no modelo. Leia as referências apenas quando forem necessárias.

**Entradas:** Origem e licença dos dados, identificadores, esquema, timestamps, estratégia de split e transformações de features.

**Procedimento:**
1. Rastreie proveniência, autorização de uso, consentimento quando aplicável e sensibilidade dos dados.
2. Identifique a unidade de independência: usuário, paciente, documento, dispositivo ou janela temporal; divida com base nessa unidade.
3. Verifique near-duplicates, entidades compartilhadas e contaminação treino/validação/teste, inclusive pré-treinamento se houver evidência.
4. Confirme que normalização, imputação, seleção de features e geração de embeddings foram ajustadas somente no treino quando exigido.
5. Em dados temporais, elimine features construídas com informação futura; audite point-in-time joins.
6. Defina relatório com evidência reproduzível, criticidade, impacto na métrica e plano de correção sem alterar dados automaticamente.

**Saída:** relatório com decisões, evidências, métricas pertinentes, limites, critérios de aceite e estado real dos testes.

**Arquivos:** [Skill](../../skills/dataset-leakage-audit/SKILL.md), [referências](../../skills/dataset-leakage-audit/references/checklist.md), [rubricas](../../skills/dataset-leakage-audit/evals/scenarios.json).

**Avaliação:** 3 cenários com expectativas positivas e falhas proibidas. CI verifica integridade, mas não mede ainda qualidade do LLM.

**Limites:** nenhum script executado automaticamente, nenhum acesso a dados privados, sem autopromoção de mudanças.
