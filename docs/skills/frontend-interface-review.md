# Skill: frontend-interface-review

**Departamento:** frontend

**O que faz:** Revisar interfaces React, Next.js ou frontend web quanto à acessibilidade, estados, performance e consistência visual. Use ao construir ou auditar formulários, dashboards, navegação e experiências SaaS.

**Quando ativar:** Implementação/revisão de UI, fluxos de formulário, mobile, acessibilidade ou performance.

**Como utilizar:** registrar `skills/frontend-interface-review/SKILL.md` em cliente compatível ou selecionar pelo catálogo Rust com `harnesses-ops suggest`. O catálogo ainda não injeta automaticamente o arquivo no modelo. Leia as referências apenas quando forem necessárias.

**Entradas:** Fluxo do usuário, telas, componentes, design system, público e dispositivo.

**Procedimento:**
1. Liste estados reais: loading, vazio, erro, sucesso, sem permissão e offline quando relevante.
2. Valide estrutura semântica, keyboard navigation, foco, labels, contraste e mensagens de erro acessíveis.
3. Faça comparação com tokens e padrões de DESIGN.md sem inventar componentes inexistentes.
4. Verifique renderização responsiva, hidratação, divisão server/client, dados sensíveis em bundles e limitações de rede.
5. Defina testes por interação (teclado, leitor de tela, e2e) e critérios visuais verificáveis; não declarar testes de navegador realizados sem ferramenta.
6. Priorize defeitos por impacto, com evidência de componente/estado e correção mínima.

**Saída:** relatório com decisões, evidências, métricas pertinentes, limites, critérios de aceite e estado real dos testes.

**Arquivos:** [Skill](../../skills/frontend-interface-review/SKILL.md), [referências](../../skills/frontend-interface-review/references/checklist.md), [rubricas](../../skills/frontend-interface-review/evals/scenarios.json).

**Avaliação:** 3 cenários com expectativas positivas e falhas proibidas. CI verifica integridade, mas não mede ainda qualidade do LLM.

**Limites:** nenhum script executado automaticamente, nenhum acesso a dados privados, sem autopromoção de mudanças.
