# Agente: optimizer

**Papel:** Recebe falhas agregadas e baseline; propõe alteração versionada com experimento.

**Quando usar:** Use quando há evidência repetível de problema de qualidade, custo ou latência.

**Como chamar:** o orquestrador seleciona o contrato em `agents/optimizer.json`, fornece apenas o contexto necessário e recebe saída estruturada. No modo offline, `OfflineWorker` simula a resposta: nenhum modelo é chamado.

**Entrada / saída:** consulte `inputs` e `outputs` no JSON. Saídas devem carregar evidências, identificador da tarefa e critérios de aceite.

**Permissões e limites:** Não altera política, credenciais, modelos ou produção automaticamente.

**Critério de sucesso:** artefato auditável, sem extrapolar o escopo delegado, compatível com testes e políticas.

**Falha e fallback:** devolva erro estruturado; não reinicie ações com efeitos colaterais sem idempotência e autorização.
