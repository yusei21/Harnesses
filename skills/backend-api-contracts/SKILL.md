---
name: backend-api-contracts
description: Projetar e revisar APIs e serviços backend com contratos, validação, autorização, migrações e testes idempotentes. Use ao desenvolver FastAPI, APIs REST, serviços de dados ou integrações SaaS.
---

# backend-api-contracts

## Quando usar
Criar endpoint, alterar schemas, persistência, contratos externos ou filas.

## Entrada
Recursos e operações, esquema, autenticação, casos de erro, consumidores e banco.

## Procedimento
1. Defina contratos de request/response e erros com status corretos; documente compatibilidade e versionamento.
2. Implemente validação e autorização por recurso antes de consultas ou mutações; evite confiar em IDs de tenant enviados pelo cliente.
3. Revise transações, idempotência de operações repetidas, timeouts de terceiros e concorrência.
4. Planeje migrações reversíveis, constraints de banco e paginação; não fazer mudanças destrutivas sem gate.
5. Especifique testes unitários, contrato, integração, limites, rate-limit e operações com erro.
6. Relate ameaças, impacto em consumidores e evidência de testes executados vs propostos.

## Referências
Consulte [critérios e antipadrões](references/checklist.md) apenas se precisar revisar pontos detalhados. Avaliações: [casos](evals/scenarios.json).

## Saída
Entregue decisões, evidências disponíveis, critérios de aceite, lacunas e testes marcados como EXECUTADOS ou NÃO EXECUTADOS. Nunca invente acesso a dados, arquivos, ferramentas ou resultados.

## Limites
A Skill orienta, mas não habilita ferramentas, não concede permissões e não altera políticas do Rust. Não indexe segredos ou dados privados em terceiros sem autorização explícita.
