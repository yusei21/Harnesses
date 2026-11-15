# backend-api-contracts — critérios e antipadrões

## Gatilhos
Criar endpoint, alterar schemas, persistência, contratos externos ou filas.

## Contexto necessário
Recursos e operações, esquema, autenticação, casos de erro, consumidores e banco.

## Critérios
Para FastAPI, use schemas Pydantic v2 e dependency injection coerentes com projeto; selecione async apenas para IO não bloqueante. Teste autorização em recursos de outro tenant (IDOR).

## Procedimento de revisão
1. Defina contratos de request/response e erros com status corretos; documente compatibilidade e versionamento.
2. Implemente validação e autorização por recurso antes de consultas ou mutações; evite confiar em IDs de tenant enviados pelo cliente.
3. Revise transações, idempotência de operações repetidas, timeouts de terceiros e concorrência.
4. Planeje migrações reversíveis, constraints de banco e paginação; não fazer mudanças destrutivas sem gate.
5. Especifique testes unitários, contrato, integração, limites, rate-limit e operações com erro.
6. Relate ameaças, impacto em consumidores e evidência de testes executados vs propostos.

## Evidência exigida
Resultados devem ter origem verificável (dataset/version/hash quando relevante, teste executado com log, ou documento identificado). Sem ambiente executor, o trabalho permanece uma proposta.
