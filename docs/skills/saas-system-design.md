# Skill: saas-system-design

**Departamento:** saas

**O que faz:** Arquitetar SaaS de forma incremental incluindo escopo MVP, multi-tenancy, identidade, cobranças e operabilidade. Use ao planejar aplicações SaaS comerciais ou freelances com evolução para produção.

**Quando ativar:** Planejar produto SaaS, arquitetura de assinatura, colaboração ou multi-tenant.

**Como utilizar:** registrar `skills/saas-system-design/SKILL.md` em cliente compatível ou selecionar pelo catálogo Rust com `harnesses-ops suggest`. O catálogo ainda não injeta automaticamente o arquivo no modelo. Leia as referências apenas quando forem necessárias.

**Entradas:** Usuários, fluxos, dados sensíveis, volume, requisitos financeiros, stack e operação esperada.

**Procedimento:**
1. Defina MVP, jornadas, não-objetivos, critérios de aceitação e custo operacional máximo aceitável.
2. Escolha modelo de tenancy e estratégias de isolamento por dados, cache, busca, arquivos e filas; defina tenant_id e enforcement no backend.
3. Modele autenticação, autorização por recurso e papéis, onboarding e ciclo de vida de conta; nunca confiar no tenant_id do cliente.
4. Planeje cobrança webhooks idempotentes, assinatura, faturamento e tratamento de falhas sem conceder acesso apenas por evento não verificado.
5. Defina migrações, backups, alertas, rate limits, ambientes e rollback; prefira serviços simples até justificar maior complexidade.
6. Entregue ADR, threat model, roadmap por fases, contrato API e testes de fronteiras de tenant.

**Saída:** relatório com decisões, evidências, métricas pertinentes, limites, critérios de aceite e estado real dos testes.

**Arquivos:** [Skill](../../skills/saas-system-design/SKILL.md), [referências](../../skills/saas-system-design/references/checklist.md), [rubricas](../../skills/saas-system-design/evals/scenarios.json).

**Avaliação:** 3 cenários com expectativas positivas e falhas proibidas. CI verifica integridade, mas não mede ainda qualidade do LLM.

**Limites:** nenhum script executado automaticamente, nenhum acesso a dados privados, sem autopromoção de mudanças.
