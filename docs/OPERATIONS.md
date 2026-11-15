# Operação da foundation

## Pré-requisitos
Python 3.11+. Na raiz:
```bash
python -m harness.cli catalog
python -m harness.cli demo --task "Investigar bug na API"
python -m harness.cli eval
python scripts/validate_catalog.py
python -m unittest discover -s tests -v
```

## Saídas
`catalog` lista contratos e Skills; `demo` gera artefatos simulados com `kind=offline-demonstration` e trace sem prompt; `eval` checa testes de roteamento e bloqueio de escrita.

## Adicionar Skill
Crie `skills/<slug>/SKILL.md` (YAML com `name` e `description`); crie `docs/skills/<slug>.md`; atualize `docs/SKILLS.md`; valide. Não registre scripts sem revisão.

## Adicionar agente
Crie `agents/<slug>.json` contendo entradas, saídas, gatilhos, ferramentas e risco; crie `docs/agents/<slug>.md`; atualize `docs/AGENTS.md`; inclua eval.

## Evoluir para produção
Selecione um runtime (Deep Agents/LangGraph, Microsoft Agent Framework, Google ADK ou OpenAI Agents SDK) baseado nos requisitos, em vez de misturar frameworks sem necessidade. Implemente um `ModelAdapter`, ferramentas de sandbox e checkpoint com testes de integração. Habilite operações externas somente após implementar aprovação autenticada e auditoria.

## Incidentes
Cancelar execução, revogar credenciais comprometidas, congelar promoção de versões, preservar traces sanitizados, abrir incidente, reverter versão e rodar evals de regressão. Esta foundation não possui serviços vivos para cancelar.
