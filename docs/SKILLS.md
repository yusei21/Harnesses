# Catálogo de Skills

Skills são procedimentos portáveis, não agentes nem fontes de permissão. O catálogo por domínio é `capabilities/catalog.json` e pode ser consultado sem carregar o conteúdo de todos os SKILL.md.

| Domínio | Skill | Manual |
|---|---|---|
| planning | `task-decomposition` | [Manual](skills/task-decomposition.md) |
| research | `evidence-research` | [Manual](skills/evidence-research.md) |
| engineering | `code-change` | [Manual](skills/code-change.md) |
| engineering | `rust-quality-gate` | [Manual](skills/rust-quality-gate.md) |
| security | `security-review` | [Manual](skills/security-review.md) |
| quality | `eval-design` | [Manual](skills/eval-design.md) |
| quality | `consistency-audit` | [Manual](skills/consistency-audit.md) |
| intelligence | `memory-curation` | [Manual](skills/memory-curation.md) |
| intelligence | `workflow-optimization` | [Manual](skills/workflow-optimization.md) |
| intelligence | `insights-triage` | [Manual](skills/insights-triage.md) |
| integrations | `mcp-integration` | [Manual](skills/mcp-integration.md) |
| orchestration | `multi-agent-routing` | [Manual](skills/multi-agent-routing.md) |

## Uso
```bash
cargo run --manifest-path rust/Cargo.toml --bin harnesses-ops -- catalog
cargo run --manifest-path rust/Cargo.toml --bin harnesses-ops -- suggest "auditoria de docs"
cargo run --manifest-path rust/Cargo.toml --bin harnesses-ops -- audit
```

O comando `suggest` somente retorna candidatos; o Rust não injeta o SKILL.md automaticamente em prompts. Para usar uma Skill no momento, leia `skills/<nome>/SKILL.md` e seu manual, ou instale-a em um CLI compatível após verificar permissões e origem.

Ao adicionar uma nova Skill, crie seu SKILL.md, documentação em `docs/skills/`, entrada no catálogo e testes. Não execute scripts vindos de Skills de terceiros sem avaliação de segurança.

Veja [CAPABILITIES.md](CAPABILITIES.md) e [ECC_ADOPTION.md](ECC_ADOPTION.md).

## Pilotos de nível avançado

- [Multi-agent routing](skills/multi-agent-routing.md): matriz de decisão, contrato Rust e cenários válidos/negativos.
- [Code change](skills/code-change.md): protocolo de mudança mínima, evidências de testes, rollback e rubricas.
- [Rust quality gate](skills/rust-quality-gate.md): checks Cargo, revisão de riscos Tokio/subprocesso e rubricas.

Os arquivos das três Skills têm `references/` e `evals/`. Para padrões, maturidade e limitações dos testes, veja [SKILL_ENGINEERING.md](SKILL_ENGINEERING.md).
