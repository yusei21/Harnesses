# Provedores OpenAI, Anthropic, Gemini e Ollama

**Caminho recomendado: CLI Rust**, que controla etapas e limites; Python serve como ponte de inferência via `harness/bridge.py` e os clientes HTTP de `harness/providers.py`.

## Requisitos
Rust stable, Python 3.11+, variável de API do provedor, ou Ollama para uso local. Os testes automatizados usam mock, não chamadas reais.

## Executar Ollama
```bash
ollama pull qwen3
cargo run --manifest-path rust/Cargo.toml -- run --task "Revisar uma API" --provider ollama --model qwen3
```

## Executar OpenAI
```bash
export OPENAI_API_KEY="..."
cargo run --manifest-path rust/Cargo.toml -- run --task "Revisar uma API" \
  --provider openai --model SEU_MODELO --allow-cloud
```

Anthropic usa `ANTHROPIC_API_KEY` e `--provider anthropic`; Gemini usa `GEMINI_API_KEY` e `--provider gemini`. Substitua `SEU_MODELO` pelo identificador habilitado na sua conta.

## Misturar modelos
```bash
export GEMINI_API_KEY="..."
cargo run --manifest-path rust/Cargo.toml -- run --task "Analise uma arquitetura" \
  --provider ollama --model qwen3 \
  --stages 'planner|researcher,engineer|reviewer' \
  --agent-model researcher=gemini:SEU_MODELO_GEMINI \
  --max-calls 4 --max-parallel 2 --allow-cloud
```

`--allow-cloud` é obrigatório ao usar qualquer provedor remoto e permite transmitir tarefa/saídas anteriores. Chaves são lidas do ambiente pelo subprocesso Python; não entram nos argumentos. A API Gemini usa `generateContent`, ainda suportada mas em rota de migração para Interactions API.

## Limites
- Até 5 invocações por execução e 3 simultâneas, independentemente do provedor.
- Limites de contexto e timeouts; **não** há quota monetária ou contagem prévia de preço/tokens.
- HTTP restrito a endpoints fixos; proxies e redirects desabilitados no adaptador.
- Sem ferramentas, shell, leitura de arquivos ou operações de escrita.
- O subprocesso Python é código local confiável do repositório, sem isolamento de SO.
- Os outputs são impressos no terminal; não os redirecione a logs públicos.
- Modo Ollama pode ter opções de offload remoto conforme configuração do próprio Ollama.
- Credenciais e limites de faturamento devem ser configurados no fornecedor; chamadas não foram testadas com sua conta real.

Consulte [RUST_CORE.md](RUST_CORE.md), [SECURITY.md](SECURITY.md) e [STATUS.md](STATUS.md).

Fontes: https://platform.openai.com/docs/api-reference/responses ; https://docs.anthropic.com/en/api/messages ; https://ai.google.dev/api/generate-content ; https://docs.ollama.com/api/introduction
