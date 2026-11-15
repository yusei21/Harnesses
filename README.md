# Harnesses — Multi-Agent Engineering Platform

Framework de referência, **agnóstico de modelo**, para engenharia de agentes com contratos explícitos, Skills portáveis, execução rastreável, gates de segurança e evolução baseada em avaliações.

> Estado: **foundation / reference implementation**. O núcleo Python funciona sem chave de API (demo determinística). Não há integração real com LLM, MCP, A2A ou sandbox remoto nesta versão. Essas integrações possuem interfaces e guias de implementação, mas não devem ser consideradas prontas para produção.

## Comece aqui

```bash
python -m harness.cli catalog
python -m harness.cli demo --task "Investigar falha na API e propor uma correção"
python -m harness.cli eval
python -m unittest discover -s tests -v
python scripts/validate_catalog.py
```

Requisitos: Python 3.11+; apenas biblioteca padrão no núcleo.

## Documentação

- [Arquitetura](docs/ARCHITECTURE.md) — camadas, fluxo e limites
- [Agentes](docs/AGENTS.md) — o que cada agente faz, quando ativar e contratos
- [Skills](docs/SKILLS.md) — o que cada Skill faz e como ativar
- [Orquestração](docs/ORCHESTRATION.md) — quando usar 1 agente, delegação e paralelismo
- [MCP e A2A](docs/INTEGRATIONS.md) — adapters e segurança de conectores
- [Hooks e segurança](docs/SECURITY.md) — política, approvals e isolamento
- [Memória e evolução](docs/EVOLUTION.md) — consolidação, evals e gates
- [Operação](docs/OPERATIONS.md) — comandos e caminho para produção
- [Decisões arquiteturais](docs/DECISIONS.md) — por que esta composição
- [Pesquisa e fontes](docs/RESEARCH.md) — referências oficiais e data de consulta
- [Limitações](docs/STATUS.md) — implementado x planejado

## Estrutura

```text
harness/               # Python: contratos, roteador, políticas, eventos, avaliações e CLI
agents/                # Contratos declarativos + guias individuais
skills/                # SKILL.md por Skill, com docs individuais
policies/              # Política de aprovação explícita
evals/                 # Casos de avaliação determinística
docs/                  # Manuais em português
scripts/               # Verificação do catálogo
tests/                 # Testes offline
.github/workflows/     # CI de validação e testes
```

## Princípios

1. Single-agent first; subagentes apenas quando houver isolamento, paralelismo útil ou competência distinta.
2. O orquestrador define limites; especialistas retornam artefatos estruturados.
3. Ferramentas perigosas exigem aprovação externa; o modelo não pode aprovar a si próprio.
4. Skills são conhecimento carregado sob demanda, não agentes executores.
5. Traces locais com redaction antes de exportar; memória de longo prazo só é promovida após validação.
6. Sem autoedição de políticas, permissões ou produção. Melhorias são propostas versionadas.
7. Roteamento e evals determinísticos no protótipo; qualidade de modelos reais depende de testes adicionais.

## Compatibilidade

O formato SKILL.md segue a especificação Agent Skills. Integrações MCP/A2A são pontos de extensão, não conexões ativas. O conteúdo pode ser reutilizado como catálogo por diferentes plataformas, respeitando as capacidades de cada runtime.

Licença: MIT.
