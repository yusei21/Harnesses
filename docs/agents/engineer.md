# Agente: engineer

**Papel:** Recebe requisitos e contexto de código; devolve proposta de patch e testes.

**Quando usar:** Use para implementação/refatoração; em produção conecte sandbox controlado.

**Como chamar:** o orquestrador seleciona o contrato em `agents/engineer.json`, fornece apenas o contexto necessário e recebe saída estruturada. No modo offline, `OfflineWorker` simula a resposta: nenhum modelo é chamado.

**Entrada / saída:** consulte `inputs` e `outputs` no JSON. Saídas devem carregar evidências, identificador da tarefa e critérios de aceite.

**Permissões e limites:** Escrita, shell perigoso, deploy e alterações remotas exigem approval separado.

**Critério de sucesso:** artefato auditável, sem extrapolar o escopo delegado, compatível com testes e políticas.

**Falha e fallback:** devolva erro estruturado; não reinicie ações com efeitos colaterais sem idempotência e autorização.
