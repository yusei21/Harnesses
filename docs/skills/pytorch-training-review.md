# Skill: pytorch-training-review

**Departamento:** machine-learning

**O que faz:** Revisar pipelines PyTorch de treinamento, inferência e data loading quanto à correção, estabilidade, performance e reprodutibilidade. Use com código de treinamento, OOM, NaN, gradientes ou resultados inconsistentes.

**Quando ativar:** Mudança em treinamento, otimização, batching, gradient accumulation ou modelo PyTorch.

**Como utilizar:** registrar `skills/pytorch-training-review/SKILL.md` em cliente compatível ou selecionar pelo catálogo Rust com `harnesses-ops suggest`. O catálogo ainda não injeta automaticamente o arquivo no modelo. Leia as referências apenas quando forem necessárias.

**Entradas:** Código do loop e modelo, shapes esperados, dispositivos, versões e sintomas observados.

**Procedimento:**
1. Confirme modo train/eval, fluxo de gradientes, zero_grad, backward, optimizer.step e scheduler na ordem efetiva.
2. Inspecione shape, dtype, masking, loss reduction e se labels e logits correspondem à tarefa.
3. Verifique seeds, determinismo condicionado ao backend, ordem dos dados, checkpoints com optimizer e scheduler.
4. Audite AMP/GradScaler quando disponível, gradient clipping, acumulação, DDP e OOM; não recomendar otimização sem perfil.
5. Verifique métricas de validação sem gradiente, ausência de leakage e consistência entre inferência e treinamento.
6. Produza diagnóstico com reproduções sugeridas, hipóteses alternativas e comandos marcados NOT_RUN caso não tenha executor.

**Saída:** relatório com decisões, evidências, métricas pertinentes, limites, critérios de aceite e estado real dos testes.

**Arquivos:** [Skill](../../skills/pytorch-training-review/SKILL.md), [referências](../../skills/pytorch-training-review/references/checklist.md), [rubricas](../../skills/pytorch-training-review/evals/scenarios.json).

**Avaliação:** 3 cenários com expectativas positivas e falhas proibidas. CI verifica integridade, mas não mede ainda qualidade do LLM.

**Limites:** nenhum script executado automaticamente, nenhum acesso a dados privados, sem autopromoção de mudanças.
