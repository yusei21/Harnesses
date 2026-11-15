# saas-system-design — critérios e antipadrões

## Gatilhos
Planejar produto SaaS, arquitetura de assinatura, colaboração ou multi-tenant.

## Contexto necessário
Usuários, fluxos, dados sensíveis, volume, requisitos financeiros, stack e operação esperada.

## Critérios
Projetos pequenos não precisam nascer em microserviços. Monólito modular com banco relacional costuma facilitar transações, observabilidade e deploy; reavalie conforme requisitos. Pagamentos exigem idempotência e verificação do emissor.

## Procedimento de revisão
1. Defina MVP, jornadas, não-objetivos, critérios de aceitação e custo operacional máximo aceitável.
2. Escolha modelo de tenancy e estratégias de isolamento por dados, cache, busca, arquivos e filas; defina tenant_id e enforcement no backend.
3. Modele autenticação, autorização por recurso e papéis, onboarding e ciclo de vida de conta; nunca confiar no tenant_id do cliente.
4. Planeje cobrança webhooks idempotentes, assinatura, faturamento e tratamento de falhas sem conceder acesso apenas por evento não verificado.
5. Defina migrações, backups, alertas, rate limits, ambientes e rollback; prefira serviços simples até justificar maior complexidade.
6. Entregue ADR, threat model, roadmap por fases, contrato API e testes de fronteiras de tenant.

## Evidência exigida
Resultados devem ter origem verificável (dataset/version/hash quando relevante, teste executado com log, ou documento identificado). Sem ambiente executor, o trabalho permanece uma proposta.
