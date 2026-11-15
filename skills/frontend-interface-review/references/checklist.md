# frontend-interface-review — critérios e antipadrões

## Gatilhos
Implementação/revisão de UI, fluxos de formulário, mobile, acessibilidade ou performance.

## Contexto necessário
Fluxo do usuário, telas, componentes, design system, público e dispositivo.

## Critérios
A11y não é só checklist. Assegure foco após ações, rótulos associados, erros anunciados e fluxo completo por teclado; avalie com ferramentas e inspeção quando disponíveis.

## Procedimento de revisão
1. Liste estados reais: loading, vazio, erro, sucesso, sem permissão e offline quando relevante.
2. Valide estrutura semântica, keyboard navigation, foco, labels, contraste e mensagens de erro acessíveis.
3. Faça comparação com tokens e padrões de DESIGN.md sem inventar componentes inexistentes.
4. Verifique renderização responsiva, hidratação, divisão server/client, dados sensíveis em bundles e limitações de rede.
5. Defina testes por interação (teclado, leitor de tela, e2e) e critérios visuais verificáveis; não declarar testes de navegador realizados sem ferramenta.
6. Priorize defeitos por impacto, com evidência de componente/estado e correção mínima.

## Evidência exigida
Resultados devem ter origem verificável (dataset/version/hash quando relevante, teste executado com log, ou documento identificado). Sem ambiente executor, o trabalho permanece uma proposta.
