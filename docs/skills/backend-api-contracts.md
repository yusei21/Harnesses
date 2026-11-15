# Skill: backend-api-contracts

**Departamento:** backend

**O que faz:** Projetar e revisar APIs e serviços backend com contratos, validação, autorização, migrações e testes idempotentes. Use ao desenvolver FastAPI, APIs REST, serviços de dados ou integrações SaaS.

**Quando ativar:** Criar endpoint, alterar schemas, persistência, contratos externos ou filas.

**Como utilizar:** registrar `skills/backend-api-contracts/SKILL.md` em cliente compatível ou selecionar pelo catálogo Rust com `harnesses-ops suggest`. O catálogo ainda não injeta automaticamente o arquivo no modelo. Leia as referências apenas quando forem necessárias.

**Entradas:** Recursos e operações, esquema, autenticação, casos de erro, consumidores e banco.

**Procedimento:**
1. Defina contratos de request/response e erros com status corretos; documente compatibilidade e versionamento.
2. Implemente validação e autorização por recurso antes de consultas ou mutações; evite confiar em IDs de tenant enviados pelo cliente.
3. Revise transações, idempotência de operações repetidas, timeouts de terceiros e concorrência.
4. Planeje migrações reversíveis, constraints de banco e paginação; não fazer mudanças destrutivas sem gate.
5. Especifique testes unitários, contrato, integração, limites, rate-limit e operações com erro.
6. Relate ameaças, impacto em consumidores e evidência de testes executados vs propostos.

**Saída:** relatório com decisões, evidências, métricas pertinentes, limites, critérios de aceite e estado real dos testes.

**Arquivos:** [Skill](../../skills/backend-api-contracts/SKILL.md), [referências](../../skills/backend-api-contracts/references/checklist.md), [rubricas](../../skills/backend-api-contracts/evals/scenarios.json).

**Avaliação:** 3 cenários com expectativas positivas e falhas proibidas. CI verifica integridade, mas não mede ainda qualidade do LLM.

**Limites:** nenhum script executado automaticamente, nenhum acesso a dados privados, sem autopromoção de mudanças.
