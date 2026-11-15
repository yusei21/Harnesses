# ADR — escolhas de projeto

## ADR-001: Python sem dependências para a foundation
Motivo: validação reproduzível e baixo acoplamento. Não é uma escolha definitiva de runtime de produção.

## ADR-002: Contratos em arquivos
Skills portáveis em `SKILL.md`; agentes em JSON com guias individuais. Facilita revisão, CI e interoperabilidade.

## ADR-003: Single-agent first
Multiagentes aumentam custo, latência, superfície de ataque e complexidade de depuração. Habilitar apenas com ganho medido.

## ADR-004: Policies fora do prompt
Autorização no limite da ferramenta com escopo e aprovação humana autenticada.

## ADR-005: Evolução por PR
Melhorias propostas e testadas; nenhuma publicação autônoma.

## ADR-006: Transparência sobre maturidade
Núcleo offline é demonstração funcional de roteamento e políticas, não framework de LLM pronto para produção.
