# Comandos Rust — escopo Harnesses

Na raiz do Harnesses (Linux):

```bash
cargo fmt --manifest-path rust/Cargo.toml -- --check
cargo clippy --manifest-path rust/Cargo.toml --all-targets -- -D warnings
cargo test --manifest-path rust/Cargo.toml --all-targets
```

Execute somente quando uma ferramenta autorizada estiver disponível. O CI atual pode usar subconjunto desses comandos; valide o arquivo `.github/workflows/ci.yml` em vez de presumir gates inexistentes.

## Pontos de revisão
- Tokio: fan-out limitado, cancelamento e tarefas órfãs.
- Processos: `Command` sem concatenação de input do modelo, timeout e stderr sanitizado.
- IO: arquivos e symlinks, limites de tamanho, caminhos canônicos.
- Segredos: não registrar headers, tokens ou prompts sensíveis.
- FFI / Python: validar schemas JSON e limites por fronteira.
- Políticas: `Risk::Write/External/Destructive` permanecem bloqueados sem autorização comprovada.

## Relatório recomendado
`cargo test`: EXECUTED/PASS (link/log), EXECUTED/FAIL (erro), ou NOT_RUN (sem executor). Não invente tempos, cobertura ou números de testes.
