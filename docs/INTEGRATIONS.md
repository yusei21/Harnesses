# Integrações

## Provedores de modelos — implementado
`harness/providers.py` implementa chamadas HTTP reais para OpenAI Responses, Anthropic Messages, Gemini generateContent e Ollama local. `harness/live.py` implementa orquestração sequencial, apenas texto, com override por agente.

Veja [PROVIDERS.md](PROVIDERS.md) e [STATUS.md](STATUS.md). Não confundir essa conexão com function calling ou execução de ferramentas.

## ModelAdapter genérico — interface de extensão
`harness/adapters.py` contém protocolos opcionais que não são a implementação atual dos provedores. Uma futura camada comum deverá normalizar structured output, cancelamento, uso de tokens, custo e tool calling.

## ToolAdapter / MCP — não implementado
MCP permite conectar tools, resources e prompts. A integração futura deverá exigir allowlist de servidores/ferramentas, autenticação por usuário, isolamento de credenciais, validação de schemas, orçamento, timeout e aprovação de side effects.

## A2A — não implementado
A2A permite comunicação com agentes externos. Exigir Agent Cards verificados, transporte autenticado, IDs de tarefas, lifecycle, cancelamento, segurança de conteúdo e não confiar cegamente em respostas remotas. A2A não é necessário para invocar vários modelos sequencialmente.

## Sandbox e ferramentas — não implementado
Antes de oferecer shell ou escrita: contenção real (contêiner/VM), rede desabilitada por padrão, limites de recursos, autorização específica por tarefa, observabilidade e rollback. Não há fallback para execução no host.

## Checkpoint e memória — não implementados
Persistência transacional, proteção de dados sensíveis, replay idempotente e provenance. Considerar LangGraph/Temporal conforme necessidade. Ver [EVOLUTION.md](EVOLUTION.md).

## Segurança de integração
Não permita que uma resposta de LLM, MCP ou A2A se transforme em comando privilegiado. Esquemas de saída e guardrails de prompt não substituem autorização implementada em código.
