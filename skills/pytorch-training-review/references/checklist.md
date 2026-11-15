# pytorch-training-review — critérios e antipadrões

## Gatilhos
Mudança em treinamento, otimização, batching, gradient accumulation ou modelo PyTorch.

## Contexto necessário
Código do loop e modelo, shapes esperados, dispositivos, versões e sintomas observados.

## Critérios
PyTorch deve usar inferência sem gradientes em validação, controle explícito de devices/shapes, checkpoints completos e inspeção de gradientes/NaNs. Seeds não garantem identidade numérica em todo hardware.

## Procedimento de revisão
1. Confirme modo train/eval, fluxo de gradientes, zero_grad, backward, optimizer.step e scheduler na ordem efetiva.
2. Inspecione shape, dtype, masking, loss reduction e se labels e logits correspondem à tarefa.
3. Verifique seeds, determinismo condicionado ao backend, ordem dos dados, checkpoints com optimizer e scheduler.
4. Audite AMP/GradScaler quando disponível, gradient clipping, acumulação, DDP e OOM; não recomendar otimização sem perfil.
5. Verifique métricas de validação sem gradiente, ausência de leakage e consistência entre inferência e treinamento.
6. Produza diagnóstico com reproduções sugeridas, hipóteses alternativas e comandos marcados NOT_RUN caso não tenha executor.

## Evidência exigida
Resultados devem ter origem verificável (dataset/version/hash quando relevante, teste executado com log, ou documento identificado). Sem ambiente executor, o trabalho permanece uma proposta.
