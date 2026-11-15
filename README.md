# Harnesses — Rust-first, Python model adapters

Harness de engenharia multiagente **local-first**. O núcleo de orquestração está em **Rust**; Python é somente um adaptador de inferência para OpenAI, Anthropic, Gemini e Ollama e permanece disponível para avaliações experimentais.

## Primeiro teste (sem chaves)
```bash
python scripts/validate_catalog.py
python -m unittest discover -s tests -v
cargo test --manifest-path rust/Cargo.toml --all-targets
```

## Executar núcleo Rust + modelo Ollama local
Requisitos: Rust stable, Python 3.11+, Ollama iniciado e modelo já instalado.

```bash
ollama pull qwen3
cargo run --manifest-path rust/Cargo.toml -- run \
  --provider ollama --model qwen3 \
  --task "Avalie minha arquitetura"
```

## Orquestração paralela controlada
```bash
cargo run --manifest-path rust/Cargo.toml -- run \
  --provider ollama --model qwen3 \
  --stages 'planner|researcher,engineer|reviewer' \
  --max-parallel 2 --max-calls 4 --task "Analise a migração"
```

## Provedores distintos por agente
```bash
export OPENAI_API_KEY="..."
cargo run --manifest-path rust/Cargo.toml -- run \
  --provider ollama --model qwen3 \
  --stages 'planner|engineer|reviewer' \
  --agent-model reviewer=openai:SEU_MODELO \
  --allow-cloud --task "Revise uma proposta"
```
`--allow-cloud` consente em transmitir objetivo e respostas anteriores para o provedor escolhido. O modelo na nuvem é cobrado conforme sua conta. Não há quota financeira automática.

## Guias
- [Rust: operação, contratos e CLI](docs/RUST_CORE.md)
- [Arquitetura e componentes](docs/ARCHITECTURE.md)
- [Agentes](docs/AGENTS.md) e [manuais individuais](docs/agents/)
- [Skills](docs/SKILLS.md) e [manuais individuais](docs/skills/)
- [Provedores](docs/PROVIDERS.md)
- [Segurança](docs/SECURITY.md)
- [Estado real do sistema](docs/STATUS.md)
- [Próximas fases](docs/EVOLUTION.md)
- [Decisões arquiteturais](docs/DECISIONS.md)
- [Fontes](docs/RESEARCH.md)

## Limitações importantes
O núcleo Rust agenda invocações de modelo em estágios, admite paralelismo de até 3 agentes, valida rotas e orçamentos de chamadas, e falha fechado em ações de risco. Não há execução de shell, acesso a arquivos por agentes, MCP/A2A, aprovação autenticada, memória persistente ou autoevolução. A ponte Python usa subprocesso de **código confiável do repositório**, mas não é um sandbox de segurança. O CLI Python anterior continua disponível para compatibilidade, não é o núcleo recomendado.

Licença MIT.
