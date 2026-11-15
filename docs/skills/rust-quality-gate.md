# Skill rust-quality-gate — manual de engenharia

**Função:** reunir procedimento de verificação Rust para mudança em runtime, concorrência Tokio, bridge Python e políticas.

**Material:** [SKILL.md](../../skills/rust-quality-gate/SKILL.md), [comandos e superfícies de risco](../../skills/rust-quality-gate/references/checks.md), [avaliações](../../skills/rust-quality-gate/evals/scenarios.json).

## Uso com executor autorizado
```bash
cargo fmt --manifest-path rust/Cargo.toml -- --check
cargo clippy --manifest-path rust/Cargo.toml --all-targets -- -D warnings
cargo test --manifest-path rust/Cargo.toml --all-targets
```

Sem executor, indicar os comandos como recomendações e marcar status `NOT_RUN`. Consulte o CI para identificar o que é realmente exigido pelo projeto; uma Skill não altera gates existentes.

## Aceitação
Status e evidência para cada check; revisão de concorrência, paths, processos e segredos; lista de falhas e sua gravidade. Não confundir compilação sem erro com garantia de segurança.

## Avaliação
Os três cenários em `evals/scenarios.json` cobrem ausência de shell, concorrência Tokio e subprocessos. A validação automatizada é estrutural, não benchmark de qualidade LLM.
