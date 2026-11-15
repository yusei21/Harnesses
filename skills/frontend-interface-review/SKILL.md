---
name: frontend-interface-review
description: Revisar interfaces React, Next.js ou frontend web quanto à acessibilidade, estados, performance e consistência visual. Use ao construir ou auditar formulários, dashboards, navegação e experiências SaaS.
---

# frontend-interface-review

## Quando usar
Implementação/revisão de UI, fluxos de formulário, mobile, acessibilidade ou performance.

## Entrada
Fluxo do usuário, telas, componentes, design system, público e dispositivo.

## Procedimento
1. Liste estados reais: loading, vazio, erro, sucesso, sem permissão e offline quando relevante.
2. Valide estrutura semântica, keyboard navigation, foco, labels, contraste e mensagens de erro acessíveis.
3. Faça comparação com tokens e padrões de DESIGN.md sem inventar componentes inexistentes.
4. Verifique renderização responsiva, hidratação, divisão server/client, dados sensíveis em bundles e limitações de rede.
5. Defina testes por interação (teclado, leitor de tela, e2e) e critérios visuais verificáveis; não declarar testes de navegador realizados sem ferramenta.
6. Priorize defeitos por impacto, com evidência de componente/estado e correção mínima.

## Referências
Consulte [critérios e antipadrões](references/checklist.md) apenas se precisar revisar pontos detalhados. Avaliações: [casos](evals/scenarios.json).

## Saída
Entregue decisões, evidências disponíveis, critérios de aceite, lacunas e testes marcados como EXECUTADOS ou NÃO EXECUTADOS. Nunca invente acesso a dados, arquivos, ferramentas ou resultados.

## Limites
A Skill orienta, mas não habilita ferramentas, não concede permissões e não altera políticas do Rust. Não indexe segredos ou dados privados em terceiros sem autorização explícita.
