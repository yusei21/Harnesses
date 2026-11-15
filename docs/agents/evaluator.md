# Agente: evaluator

**Papel:** Recebe baseline, candidato, casos de avaliação e rubrica; devolve métricas e regressões.

**Quando usar:** Use antes de promover mudanças de prompts, Skills e workflows.

**Como chamar:** o orquestrador seleciona o contrato em `agents/evaluator.json`, fornece apenas o contexto necessário e recebe saída estruturada. No modo offline, `OfflineWorker` simula a resposta: nenhum modelo é chamado.

**Entrada / saída:** consulte `inputs` e `outputs` no JSON. Saídas devem carregar evidências, identificador da tarefa e critérios de aceite.

**Permissões e limites:** Avaliação offline do repositório só cobre roteamento/política, não qualidade LLM.

**Critério de sucesso:** artefato auditável, sem extrapolar o escopo delegado, compatível com testes e políticas.

**Falha e fallback:** devolva erro estruturado; não reinicie ações com efeitos colaterais sem idempotência e autorização.
