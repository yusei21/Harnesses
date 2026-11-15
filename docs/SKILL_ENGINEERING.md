# Padrão de qualidade de Skills — Harnesses

## Fontes técnicas consultadas (15/11/2026)
- [Agent Skills Specification](https://agentskills.io/specification): `SKILL.md` com `name` e `description`, `scripts/`, `references/` e arquivos auxiliares opcionais.
- [Anthropic: Skill authoring best practices](https://platform.claude.com/docs/en/agents-and-tools/agent-skills/best-practices): contexto conciso, carregamento progressivo, avaliações iniciais, exemplos verificáveis e referências diretas.
- [Anthropic: Building effective agents](https://www.anthropic.com/research/building-effective-agents): aumentar complexidade só quando evals demonstram melhora.
- [OpenAI: skill-creator](https://github.com/openai/skills/blob/main/skills/.system/skill-creator/SKILL.md): grau de liberdade proporcional à fragilidade, entradas e saídas, scripts só quando melhoram confiabilidade.
- [OpenAI: Codex best practices](https://learn.chatgpt.com/docs/learn/best-practices): começar com 2–3 cenários e tornar reutilizáveis tarefas recorrentes.
- [ECC](https://github.com/affaan-m/ECC): inspiração para controles de qualidade, especialização e evolução mensurada; **não copiamos suas Skills**.

## Decisão
Usar `skills/<id>/SKILL.md` portátil em múltiplos CLIs. Não adicionar extensões obrigatórias no frontmatter, privilégios no arquivo da Skill, nem subagentes apenas para dividir instruções. O catálogo canônico `capabilities/catalog.json` mantém domínio e gatilhos separados do formato portátil.

### Padrão por maturidade
- **Básica:** SKILL.md válido com descrição orientada a gatilho e procedimento não trivial.
- **Operacional:** procedimento, entradas, saídas, limites e referências necessárias.
- **Avaliada:** pelo menos três cenários positivos/negativos, casos em `evals/`, critérios que diferenciam sucesso de falha, registros de baseline quando disponíveis.
- **Comprovada:** testes reais com e sem Skill em modelos/ambientes relevantes, análise de custo/latência, revisões de regressão e observações de uso.

Os três pilotos `multi-agent-routing`, `code-change` e `rust-quality-gate` estão no nível **avaliada estruturalmente**, não **comprovada em modelos reais**.

## Quem decide o quê
- **Skill**: conhecimento e procedimento sob demanda.
- **Catálogo**: descoberta por metadados e domínio.
- **Agente**: propõe e executa tarefas via ferramentas autorizadas.
- **Rust Policy/Runner**: aplica limites e permissões.
- **CI/evals**: validam artefatos e registram resultados.

## Checagens
```bash
python scripts/validate_catalog.py
python scripts/validate_skill_quality.py
python -m unittest discover -s tests -v
cargo test --manifest-path rust/Cargo.toml --all-targets
```
O validador é deliberadamente restrito aos três pilotos. Para demais Skills, manter checagem de metadados existente, registrar um backlog de refatoração e só aprofundar onde houver benefícios observáveis.

## Backlog das demais Skills
`task-decomposition`, `evidence-research`, `security-review`, `eval-design`, `memory-curation`, `workflow-optimization`, `mcp-integration`, `consistency-audit` e `insights-triage` ainda são versões básicas e precisam de cenários úteis antes de serem reescritas. Não declarar conformidade ao nível avançado para essas Skills.
