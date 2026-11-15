# Catálogo de agentes

Sete agentes declarativos, com descrição em `agents/<nome>.json` e documentação individual. A execução **real em texto** usa `harness/live.py`, com uma chamada de modelo para cada papel escolhido, em ordem.

| Agente | Responsabilidade | Manual |
|---|---|---|
| `planner` | Decompõe o objetivo, define estratégia e critérios de aceite. | [Guia](agents/planner.md) |
| `researcher` | Interpreta contexto e evidências que o usuário fornecer. | [Guia](agents/researcher.md) |
| `engineer` | Elabora propostas técnicas e estratégias de implementação. | [Guia](agents/engineer.md) |
| `reviewer` | Revisa criticamente textos das etapas anteriores, identifica riscos e lacunas. | [Guia](agents/reviewer.md) |
| `evaluator` | Propõe métricas, casos de avaliação e critérios de regressão. | [Guia](agents/evaluator.md) |
| `memory-curator` | Propõe memórias candidatas para posterior validação. | [Guia](agents/memory-curator.md) |
| `optimizer` | Propõe hipóteses de melhoria e experimentos comparativos. | [Guia](agents/optimizer.md) |

## Como executar
Use Ollama local (modelo deve estar instalado):
```bash
python -m harness.cli run --provider ollama --model qwen3 --agents planner,engineer,reviewer --task "Proponha uma arquitetura"
```

O padrão é `planner,engineer,reviewer`. É possível trocar a sequência por nomes do catálogo e usar provedores diferentes:

```bash
python -m harness.cli run --provider ollama --model qwen3 \
  --agents planner,researcher,reviewer --max-calls 3 \
  --agent-model reviewer=anthropic:SEU_MODELO --allow-cloud \
  --task "Analise um design"
```

O runtime não lê automaticamente o JSON do agente ao gerar prompts; o papel é definido em `harness/live.py`. Os contratos JSON são recursos declarativos para futura integração.

## O que não faz
A execução dos agentes não inclui filesystem, execução de testes, acesso web, MCP, A2A, persistência, paralelismo ou autoevolução. Os nomes `engineer` e `reviewer` não indicam que tenham executado código ou revisado arquivos reais. Para detalhes técnicos leia [PROVIDERS.md](PROVIDERS.md) e [STATUS.md](STATUS.md).
