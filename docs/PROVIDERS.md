# Provedores reais e execução multiagente local

O runtime implementa chamadas HTTP de texto para OpenAI Responses, Anthropic Messages, Gemini generateContent e Ollama local. Não há dependências Python externas. **Os testes usam mocks, não suas chaves de API.**

## Comandos

Verifique primeiro:
```bash
python scripts/validate_catalog.py
python -m unittest discover -s tests -v
python -m harness.cli eval
```

Ollama: instale/inicie o Ollama e baixe um modelo que seu hardware suporta:
```bash
ollama pull qwen3
python -m harness.cli run --provider ollama --model qwen3 --task "Propor melhoria arquitetural"
```

OpenAI:
```bash
export OPENAI_API_KEY="..."
python -m harness.cli run --provider openai --model SEU_MODELO --allow-cloud --task "Revisar API"
```

Anthropic:
```bash
export ANTHROPIC_API_KEY="..."
python -m harness.cli run --provider anthropic --model SEU_MODELO --allow-cloud --task "Revisar API"
```

Gemini:
```bash
export GEMINI_API_KEY="..."
python -m harness.cli run --provider gemini --model SEU_MODELO --allow-cloud --task "Revisar API"
```

Substitua `SEU_MODELO` por um identificador de modelo habilitado na conta. Variáveis de ambiente não são automaticamente carregadas de arquivos .env.

Misturando provedores em uma execução:
```bash
python -m harness.cli run --task "Analisar migração de sistema" \
  --provider ollama --model qwen3 \
  --agents planner,researcher,reviewer --max-calls 3 \
  --agent-model researcher=gemini:SEU_MODELO_GEMINI \
  --agent-model reviewer=openai:SEU_MODELO_OPENAI \
  --allow-cloud
```

## O que acontece
Cada agente é uma invocação real de modelo, sequencial. O próximo agente recebe objetivo e textos dos anteriores como **dados não confiáveis**, dentro de uma instrução que proíbe obedecer comandos recebidos nos textos. Os modelos NÃO acessam documentos, internet, ferramentas, MCP, shell, sistema de arquivos ou processos externos. As respostas são propostas, não modificações aplicadas.

## Segurança
- Cloud exige `--allow-cloud` explícito; ele indica consentimento de transmissão, **não** uma autorização de ferramenta.
- Chaves permanecem em variáveis de ambiente. Nenhum segredo deve entrar nos prompts ou no repositório.
- URLs de destino fixas, cloud HTTPS, Ollama em 127.0.0.1:11434; nenhum proxy ambiental, redirects ou retries automáticos.
- Máximo de cinco chamadas por execução, com limite de payload/retorno/timeout.
- Não existe controle de custo monetário: configure limites de gastos no provedor. Não há detecção automática de dados sigilosos.
- O modo local Ollama pode chamar modelos cloud dependendo da configuração do seu servidor Ollama; verifique isso se a privacidade for importante.
- A CLI imprime respostas no terminal, portanto cuidado ao salvar saídas em arquivos compartilhados.
- Sem testes end-to-end contra APIs de produção: disponibilidade e compatibilidade dos modelos precisam de smoke tests com suas credenciais.

## Versões das APIs
OpenAI: `/v1/responses`; Anthropic: `/v1/messages` com versão `2026-10-21`; Ollama: `/api/chat`; Gemini: `/v1beta/models/{model}:generateContent`. O Google recomenda **Interactions API** em 2026; generateContent continua suportada mas é considerada legada. Migrar depois com testes de contrato.

Fontes: https://developers.openai.com/api/reference/resources/responses/methods/create ; https://platform.claude.com/docs/en/api/messages/create ; https://ai.google.dev/api/generate-content ; https://ai.google.dev/gemini-api/docs/interactions-overview ; https://docs.ollama.com/api/introduction
