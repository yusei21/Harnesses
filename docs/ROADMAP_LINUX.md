# Roadmap: Harnesses Linux, Rust-first, multi-CLI, local-first

## Princípios
- CLI Rust e serviços locais como ponto de integração; UI web/desktop opcional depois.
- Python somente adaptador de inferência e experimentação.
- Modelo executa instruções, mas não concede permissões; side-effects somente com escopo e aprovação real.
- Interoperar com Claude Code, Codex, OpenCode, Kimi e AGY por contratos distintos; nunca assumir que todos expõem as mesmas capacidades.
- Sem cópia indiscriminada de ECC nem instalação global automática.

## Fase A — esta PR
Catálogo de capacidades por domínio, auditoria determinística, Skills de auditoria/qualidade/insights, CLI Rust read-only, CI e documentação. Dados locais e exemplos explícitos.

## Fase B — integração com CLI existentes
Descobrir versões e capacidades, exportar Skills selecionadas com preview, diff, backup e rollback. Padrão: sem tocar diretórios globais; instalação apenas por projeto e confirmação. Adaptadores testados um a um.

## Fase C — memória de projeto
SQLite/FTS5 com índices textuais, proveniência e TTL; AST/code graph sob demanda, embeddings locais opcionais. Não indexar segredos; isolamento por repo/projeto.

## Fase D — execução real
Workers com worktrees, sandbox, limite de rede e recursos, allowlists, ferramentas MCP read-only antes de writes, checkpoints idempotentes e aprovação humana autenticada.

## Fase E — melhoria contínua
Observações sanitizadas -> proposta de Skill/hook/refatoração -> testes e avaliações holdout -> PR para aprovação. Nenhuma autopromoção, deploy ou relaxamento de política pelo modelo.
