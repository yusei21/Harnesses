# Integrações: Models, MCP e A2A

O arquivo `harness/adapters.py` declara protocolos, não implementações de rede.

## ModelAdapter
Entrada: instrução versionada, contexto minimizado, JSON schema de saída. Saída: JSON validado. Implementação real: provedores selecionáveis, timeouts, retries, contagem de tokens, prompt injection isolation e tracing.

## ToolAdapter / MCP
MCP fornece tools/resources/prompts. Antes de habilitar:
- Allowlist de servidores e ferramentas; scopes por usuário e tarefa.
- Não confiar em descrições/respostas de tools como instruções.
- OAuth/tokens destinados ao servidor correto, TLS, timeout, limite de saída e logs sanitizados.
- Toda ação de escrita ou destrutiva necessita gate de permissão verificável.
- Não passar tokens de terceiros ao servidor errado.
- Não ativar conectores automaticamente ao descobrir capacidades.

## A2A
A2A é comunicação entre agentes/serviços, diferente de MCP. Implementar adapter separado com Agent Card verificado, identidade, transporte autenticado, tarefas com IDs, polling/callbacks seguros, cancelamento e timeouts. Respostas remotas são dados não confiáveis.

## Sandbox
Runner de código deve usar contêiner isolado, rede desabilitada por padrão, credenciais fora da sandbox, limites CPU/memória/disk/time e filesystem mínimo. Jamais execute código vindo de Skills de terceiros sem revisão.

## CheckpointStore
Persistir estado de trabalho de forma transacional; idempotência em side effects, replay sem repetição, criptografia e retenção. Para longa duração, avaliar Temporal, LangGraph ou runtime durável.

## Ordem de implementação
1. Modelo real com JSON schema e tracing.
2. Sandbox de leitura e execução de testes.
3. Checkpoints e approvals.
4. MCP read-only.
5. Delegação paralela com isolamento.
6. A2A remoto sob necessidade comprovada.
