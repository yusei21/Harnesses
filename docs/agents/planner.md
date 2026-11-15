# Agente: planner

**Papel:** Recebe objetivo e restrições; devolve plano, critérios de aceite, dependências e tarefas candidatas.

**Quando usar:** Inicie por ele somente em tarefas complexas. O orquestrador pode dispensá-lo em atividades simples.

**Como chamar:** o orquestrador seleciona o contrato em `agents/planner.json`, fornece apenas o contexto necessário e recebe saída estruturada. No modo offline, `OfflineWorker` simula a resposta: nenhum modelo é chamado.

**Entrada / saída:** consulte `inputs` e `outputs` no JSON. Saídas devem carregar evidências, identificador da tarefa e critérios de aceite.

**Permissões e limites:** Não executa ferramentas de escrita e não delega sem limites.

**Critério de sucesso:** artefato auditável, sem extrapolar o escopo delegado, compatível com testes e políticas.

**Falha e fallback:** devolva erro estruturado; não reinicie ações com efeitos colaterais sem idempotência e autorização.
