---
name: pytorch-training-review
description: Revisar pipelines PyTorch de treinamento, inferência e data loading quanto à correção, estabilidade, performance e reprodutibilidade. Use com código de treinamento, OOM, NaN, gradientes ou resultados inconsistentes.
---

# pytorch-training-review

## Quando usar
Mudança em treinamento, otimização, batching, gradient accumulation ou modelo PyTorch.

## Entrada
Código do loop e modelo, shapes esperados, dispositivos, versões e sintomas observados.

## Procedimento
1. Confirme modo train/eval, fluxo de gradientes, zero_grad, backward, optimizer.step e scheduler na ordem efetiva.
2. Inspecione shape, dtype, masking, loss reduction e se labels e logits correspondem à tarefa.
3. Verifique seeds, determinismo condicionado ao backend, ordem dos dados, checkpoints com optimizer e scheduler.
4. Audite AMP/GradScaler quando disponível, gradient clipping, acumulação, DDP e OOM; não recomendar otimização sem perfil.
5. Verifique métricas de validação sem gradiente, ausência de leakage e consistência entre inferência e treinamento.
6. Produza diagnóstico com reproduções sugeridas, hipóteses alternativas e comandos marcados NOT_RUN caso não tenha executor.

## Referências
Consulte [critérios e antipadrões](references/checklist.md) apenas se precisar revisar pontos detalhados. Avaliações: [casos](evals/scenarios.json).

## Saída
Entregue decisões, evidências disponíveis, critérios de aceite, lacunas e testes marcados como EXECUTADOS ou NÃO EXECUTADOS. Nunca invente acesso a dados, arquivos, ferramentas ou resultados.

## Limites
A Skill orienta, mas não habilita ferramentas, não concede permissões e não altera políticas do Rust. Não indexe segredos ou dados privados em terceiros sem autorização explícita.
