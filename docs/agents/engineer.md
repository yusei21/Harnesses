# Agente: engineer

## O que faz
Proposta textual de soluções de software e testes necessários.

## Quando usar
Inclua este papel apenas quando houver uma contribuição verificável para a tarefa. Em tarefas pequenas, prefira um agente. O orquestrador não cria agentes adicionais de forma dinâmica nesta versão.

## Como executar com Rust (caminho principal)
Instale Rust/Cargo, Python 3.11+, Ollama e o modelo correspondente. Na raiz do repositório:

```bash
cargo run --manifest-path rust/Cargo.toml -- run \
  --provider ollama --model qwen3 \
  --stages 'engineer' --max-calls 1 \
  --task "Analise um problema"
```

Para incluir engineer em uma sequência com outros agentes, utilize `--stages 'planner|engineer|reviewer'` (papéis sem repetição). A barra vertical separa estágios; a vírgula permite papéis simultâneos no mesmo estágio, por exemplo `planner|researcher,engineer|reviewer`.

Para configurar um provedor diferente para este papel: `--agent-model engineer=openai:SEU_MODELO --allow-cloud` e uma variável `OPENAI_API_KEY` configurada. A opção `--allow-cloud` consente em transmitir o objetivo e os outputs anteriores ao provedor externo.

## Contrato
`agents/engineer.json` contém a definição declarativa: papel, entradas, saídas, ferramentas pretendidas, risco e limites. O runtime Rust valida o nome do agente; a ponte Python fornece as instruções específicas. Não há carregamento dinâmico do contrato JSON completo no modelo.

## Saída esperada
Resposta textual em português com ressalvas sobre incerteza, armazenada em `outputs[]` do JSON final, além de metadados em `events[]`. Apenas agentes de estágios posteriores recebem a saída deste agente.

## Limitações
Não lê arquivos do projeto, edita patches ou executa comandos. Nenhum agente tem ferramentas, acesso a arquivos ou aprovação de ações nesta versão. Nomear um agente de `engineer` não significa que ele tenha editado código.

## Como testar
```bash
cargo test --manifest-path rust/Cargo.toml --all-targets
python -m unittest discover -s tests -v
```

Esses testes verificam contratos e fluxos com mocks, não a qualidade do modelo ou uma integração end-to-end em conta real. Veja [núcleo Rust](../RUST_CORE.md) e [provedores](../PROVIDERS.md).
