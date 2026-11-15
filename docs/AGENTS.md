# Catálogo de agentes (Rust-first)

A plataforma possui **sete papéis** documentados, identificados por nome no runtime Rust. Cada papel possui um `agents/<nome>.json` declarativo e manual próprio em `docs/agents/`. Python somente executa a chamada de modelo e fornece o prompt correspondente ao papel.

| Agente | Função | Manual |
|---|---|---|
| `planner` | Planejamento de tarefas, dependências e critérios de aceite. | [Manual](agents/planner.md) |
| `researcher` | Análise de fatos fornecidos, incertezas e evidências. | [Manual](agents/researcher.md) |
| `engineer` | Proposta textual de soluções de software e testes necessários. | [Manual](agents/engineer.md) |
| `reviewer` | Revisão independente das saídas dos estágios anteriores. | [Manual](agents/reviewer.md) |
| `evaluator` | Projeto de casos de avaliação, rubricas e regressões. | [Manual](agents/evaluator.md) |
| `memory-curator` | Propostas de memória com rastreabilidade e validade. | [Manual](agents/memory-curator.md) |
| `optimizer` | Hipóteses de otimização e proposta de experimentos. | [Manual](agents/optimizer.md) |

## Invocação principal
```bash
cargo run --manifest-path rust/Cargo.toml -- run \
  --provider ollama --model qwen3 \
  --stages 'planner|researcher,engineer|reviewer' \
  --max-calls 4 --max-parallel 2 \
  --task "Investigar a arquitetura de uma API"
```

No exemplo, Researcher e Engineer recebem ambos o resultado do Planner e rodam em paralelo. Reviewer recebe os resultados consolidados em ordem estável. Não há conversação livre, delegação dinâmica, acesso a arquivos, shell ou pesquisa externa. Use `--agent-model reviewer=openai:SEU_MODELO --allow-cloud` para rotear outro modelo, com chave no ambiente.

A antiga CLI Python `python -m harness.cli run` permanece como interface de compatibilidade. Para novos fluxos, use Rust. Detalhes em [RUST_CORE.md](RUST_CORE.md).

## Importante
As definições JSON ainda não são carregadas automaticamente pelo orquestrador como prompts ou ferramentas. O papel é reconhecido e a ponte Python aplica instruções predefinidas. Modificações de contratos precisam de testes e documentação.
