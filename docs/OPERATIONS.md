# Operação

## Foundation offline
```bash
python scripts/validate_catalog.py
python -m unittest discover -s tests -v
python -m harness.cli eval
python -m harness.cli catalog
python -m harness.cli demo --task "Investigar bug"
```

## Agentes reais com modelo local
```bash
ollama pull qwen3
python -m harness.cli run --provider ollama --model qwen3 --task "Revisar arquitetura"
```
Veja [PROVIDERS.md](PROVIDERS.md) para as APIs OpenAI, Anthropic e Gemini, `--allow-cloud` e mistura de provedores.

## Agentes e Skills
Cada agente possui JSON em `agents/` e manual correspondente em `docs/agents/`. Cada Skill possui `SKILL.md` e manual em `docs/skills/`. Ao alterar, atualize catálogos, docs e testes.

## Segurança operacional
O runtime LLM não tem acesso a arquivos ou ferramentas. Seu resultado não executa mudanças. Não forneça dados privados em cloud sem autorização. Verifique quotas externas para evitar cobranças inesperadas.

## Próximas integrações
Checkpoint, aprovação autenticada, sandbox, MCP read-only, evals reais e depois A2A/auto-otimização com PRs aprovados.
