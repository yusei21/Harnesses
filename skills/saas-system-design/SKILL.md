---
name: saas-system-design
description: Arquitetar SaaS de forma incremental incluindo escopo MVP, multi-tenancy, identidade, cobranças e operabilidade. Use ao planejar aplicações SaaS comerciais ou freelances com evolução para produção.
---

# saas-system-design

## Quando usar
Planejar produto SaaS, arquitetura de assinatura, colaboração ou multi-tenant.

## Entrada
Usuários, fluxos, dados sensíveis, volume, requisitos financeiros, stack e operação esperada.

## Procedimento
1. Defina MVP, jornadas, não-objetivos, critérios de aceitação e custo operacional máximo aceitável.
2. Escolha modelo de tenancy e estratégias de isolamento por dados, cache, busca, arquivos e filas; defina tenant_id e enforcement no backend.
3. Modele autenticação, autorização por recurso e papéis, onboarding e ciclo de vida de conta; nunca confiar no tenant_id do cliente.
4. Planeje cobrança webhooks idempotentes, assinatura, faturamento e tratamento de falhas sem conceder acesso apenas por evento não verificado.
5. Defina migrações, backups, alertas, rate limits, ambientes e rollback; prefira serviços simples até justificar maior complexidade.
6. Entregue ADR, threat model, roadmap por fases, contrato API e testes de fronteiras de tenant.

## Referências
Consulte [critérios e antipadrões](references/checklist.md) apenas se precisar revisar pontos detalhados. Avaliações: [casos](evals/scenarios.json).

## Saída
Entregue decisões, evidências disponíveis, critérios de aceite, lacunas e testes marcados como EXECUTADOS ou NÃO EXECUTADOS. Nunca invente acesso a dados, arquivos, ferramentas ou resultados.

## Limites
A Skill orienta, mas não habilita ferramentas, não concede permissões e não altera políticas do Rust. Não indexe segredos ou dados privados em terceiros sem autorização explícita.
