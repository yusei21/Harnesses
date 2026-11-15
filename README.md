# Harnesses — multiagentes locais e multiprovedor

Harness de referência em Python 3.11+. Possui **integração HTTP real** com OpenAI, Anthropic, Gemini e Ollama local; orquestração **sequencial, texto apenas**, com modelos diferentes para cada agente. Não executa código, não modifica arquivos, não oferece ferramentas MCP/A2A e não contém memória persistente.

## Início rápido
```bash
python scripts/validate_catalog.py
python -m unittest discover -s tests -v
python -m harness.cli eval
python -m harness.cli catalog
```

## Usar no seu computador com Ollama
```bash
ollama pull qwen3
python -m harness.cli run --provider ollama --model qwen3 --task "Proponha melhorias no sistema"
```

## Usar nuvem (exemplo OpenAI)
```bash
export OPENAI_API_KEY="..."
python -m harness.cli run --provider openai --model SEU_MODELO --allow-cloud --task "Revisar arquitetura"
```

## Documentação detalhada
- [Como configurar e usar provedores, inclusive multi-provedor](docs/PROVIDERS.md)
- [Arquitetura](docs/ARCHITECTURE.md)
- [Catálogo de agentes](docs/AGENTS.md) — cada um também em `docs/agents/<nome>.md`
- [Catálogo de Skills](docs/SKILLS.md) — cada uma também em `docs/skills/<nome>.md`
- [Orquestração](docs/ORCHESTRATION.md)
- [Hooks, aprovações e segurança](docs/SECURITY.md)
- [Memória e autoevolução — plano](docs/EVOLUTION.md)
- [Operação](docs/OPERATIONS.md)
- [Estado real e limitações](docs/STATUS.md)
- [Fontes pesquisadas](docs/RESEARCH.md)

## Estrutura
```text
harness/providers.py   Conectores HTTP com destinos fixos
harness/live.py        Agentes LLM em sequência
harness/core.py        Runtime offline e políticas demonstrativas
harness/cli.py         CLI catalog/demo/eval/run
agents/                7 contratos de agentes
skills/                9 Skills Agent Skills
docs/agents/           Documentação individual
docs/skills/           Documentação individual
tests/                 Testes mockados e offline
```

Segurança: `--allow-cloud` obrigatório para transmissão externa; APIs de nuvem só via HTTPS fixo; Ollama só em loopback; sem retries automáticos; no máximo 5 chamadas por execução. **Sem orçamento financeiro efetivo**, não use dados sensíveis sem autorização. Modelos retornam somente texto; não podem alterar o computador.

Licença MIT.
