# Núcleo Rust — operação e arquitetura

## Papel do Rust
O Rust controla plano, ordem das etapas, limite de chamadas, paralelismo, consentimento para nuvem, estados e metadados de execução. Os componentes centrais estão em `rust/src/core.rs`, a ponte de subprocesso em `rust/src/bridge.rs` e a CLI em `rust/src/main.rs`.

## Papel do Python
A interface `harness/bridge.py` recebe uma mensagem JSON via stdin e delega uma única geração a `harness/providers.py`. Retorna texto e tokens em JSON via stdout. Não permite executar ferramentas, shell, arquivos ou processos arbitrários a partir do prompt. O código Python do repositório é confiável por premissa e não está isolado em sandbox.

## Instalação
- Rust stable com Cargo (MSRV declarada: 1.85).
- Python 3.11+.
- Ollama para uso local, ou credenciais de provedor para cloud.
- Nenhuma dependência externa de Python necessária no bridge atual.

Execute:
```bash
cargo test --manifest-path rust/Cargo.toml --all-targets
python -m unittest discover -s tests -v
```

## CLI
```bash
cargo run --manifest-path rust/Cargo.toml -- run \
  --task "Especifique uma melhoria" \
  --provider ollama --model qwen3 \
  --stages 'planner|researcher,engineer|reviewer' \
  --max-calls 4 --max-parallel 2
```

A barra `|` separa estágios sequenciais. A vírgula separa agentes paralelos no mesmo estágio. Todos os agentes do mesmo estágio recebem somente as saídas dos estágios anteriores. Saídas são reordenadas pela ordem do plano para garantir determinismo da apresentação.

Override por agente:
```bash
export ANTHROPIC_API_KEY="..."
cargo run --manifest-path rust/Cargo.toml -- run \
  --task "Audite esta proposta" --provider ollama --model qwen3 \
  --agent-model reviewer=anthropic:SEU_MODELO --allow-cloud
```

O nome de cada agente deve corresponder ao catálogo, sem repetição entre estágios. Limites: até 5 chamadas por execução, até 3 agentes simultâneos, até 8 KB no objetivo e até 16 KB de histórico compartilhado antes de cada estágio. O gateway subprocesso aplica timeout de 90 segundos por chamada e falha com erro sanitizado.

## Dados e observabilidade
Em sucesso, o comando retorna JSON com `status`, `mode`, `events` contendo provedor, modelo e tokens quando disponíveis, `outputs` com respostas e `final_text`. As respostas são mostradas no terminal, então não as redirecione a logs públicos. Não há checkpoint, métricas persistentes nem gestão de segredo além das variáveis de ambiente.

## Verificação e segurança
`validate_plan` valida **todas** as rotas antes de enviar qualquer chamada. O `Policy` em Rust permite apenas `Risk::Read`; `Write`, `External`, `Destructive` são bloqueados sem implementar uma falsa aprovação. Não existem ferramentas a serem autorizadas no runtime atual.

A ponte pode lançar somente `python3 -m harness.bridge` com diretório fixado no repositório. Não é um sandbox: subprocessos de código confiável continuam com permissões do usuário local. Não habilite tools de terceiros sem sandbox, validação de argumentos e autorização independente.

## Migração e próximos passos
O CLI em Python permanece para compatibilidade. Evoluir o Rust por etapas: (1) testes reais e budgets, (2) checkpoints seguros, (3) tools read-only com MCP, (4) sandbox e aprovações verificáveis, (5) melhorias por PR com evals holdout. Não adotar A2A para subagentes locais sem necessidade concreta.

Consulte [PROVIDERS.md](PROVIDERS.md), [SECURITY.md](SECURITY.md), [STATUS.md](STATUS.md).
