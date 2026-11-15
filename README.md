# Harnesses — Rust-first, Linux local-first, multi-CLI

Harness de engenharia multiagente local, com núcleo em Rust e Python como ponte de inferência para OpenAI, Anthropic, Gemini e Ollama. Além do pipeline de modelos, o projeto tem um catálogo seletivo de Skills por domínio, auditoria e um `insights` determinístico baseado em eventos.

## Início rápido
```bash
python scripts/validate_catalog.py
python -m unittest discover -s tests -v
cargo test --manifest-path rust/Cargo.toml --all-targets
cargo run --manifest-path rust/Cargo.toml --bin harnesses-ops -- catalog
cargo run --manifest-path rust/Cargo.toml --bin harnesses-ops -- suggest "auditoria de documentacao"
cargo run --manifest-path rust/Cargo.toml --bin harnesses-ops -- audit
cargo run --manifest-path rust/Cargo.toml --bin harnesses-ops -- insights examples/observations.json
```

O comando `insights` só lê um arquivo explicitamente fornecido, sem capturar sessões de agentes automaticamente; os dados do exemplo são fictícios.

## Executar modelos via Rust e Python
```bash
ollama pull qwen3
cargo run --manifest-path rust/Cargo.toml --bin harnesses-core -- run \
  --provider ollama --model qwen3 --task "Avalie esta arquitetura"
```
Para mistura de modelos, detalhes em [RUST_CORE.md](docs/RUST_CORE.md) e [PROVIDERS.md](docs/PROVIDERS.md). Cloud exige `--allow-cloud`.

## Documentação
- [Catálogo de capacidades, seleção, auditoria e insights](docs/CAPABILITIES.md)
- [Incorporação seletiva dos conceitos do ECC e atribuição](docs/ECC_ADOPTION.md)
- [Roadmap Linux / multi-CLI / memória local](docs/ROADMAP_LINUX.md)
- [Agentes](docs/AGENTS.md) e [manuais individuais](docs/agents/)
- [Skills](docs/SKILLS.md) e [manuais individuais](docs/skills/)
- [Arquitetura e Rust](docs/ARCHITECTURE.md) e [RUST_CORE.md](docs/RUST_CORE.md)
- [Segurança](docs/SECURITY.md), [status](docs/STATUS.md), [pesquisa](docs/RESEARCH.md)

Consulte [Skills](docs/SKILLS.md), [guia de especialização](docs/ML_SAAS_STRATEGY.md) e [origem das ideias do ECC](docs/ECC_ADOPTION.md).

## Skills operacionais com cenários de regressão

As Skills `multi-agent-routing`, `code-change` e `rust-quality-gate` agora incluem referências e avaliações específicas, mantendo os nove domínios de IA/ML e SaaS já presentes na `main`. Veja [SKILL_ENGINEERING.md](docs/SKILL_ENGINEERING.md). Verifique com `python scripts/validate_skill_quality.py`.
