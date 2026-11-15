# Agente: memory-curator

**Papel:** Recebe traces sanitizados e evidências; propõe memórias com fontes e validade.

**Quando usar:** Use em consolidação periódica ou após incidentes resolvidos.

**Como chamar:** o orquestrador seleciona o contrato em `agents/memory-curator.json`, fornece apenas o contexto necessário e recebe saída estruturada. No modo offline, `OfflineWorker` simula a resposta: nenhum modelo é chamado.

**Entrada / saída:** consulte `inputs` e `outputs` no JSON. Saídas devem carregar evidências, identificador da tarefa e critérios de aceite.

**Permissões e limites:** Não adiciona memória ativa sem validação e revisão de dados sensíveis.

**Critério de sucesso:** artefato auditável, sem extrapolar o escopo delegado, compatível com testes e políticas.

**Falha e fallback:** devolva erro estruturado; não reinicie ações com efeitos colaterais sem idempotência e autorização.
