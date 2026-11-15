# Agente: reviewer

**Papel:** Recebe diff, requisitos e testes; devolve achados priorizados por severidade.

**Quando usar:** Use após mudanças técnicas, idealmente sem compartilhar o contexto inteiro do executor.

**Como chamar:** o orquestrador seleciona o contrato em `agents/reviewer.json`, fornece apenas o contexto necessário e recebe saída estruturada. No modo offline, `OfflineWorker` simula a resposta: nenhum modelo é chamado.

**Entrada / saída:** consulte `inputs` e `outputs` no JSON. Saídas devem carregar evidências, identificador da tarefa e critérios de aceite.

**Permissões e limites:** Não aprova seus próprios achados nem tem poder de merge.

**Critério de sucesso:** artefato auditável, sem extrapolar o escopo delegado, compatível com testes e políticas.

**Falha e fallback:** devolva erro estruturado; não reinicie ações com efeitos colaterais sem idempotência e autorização.
