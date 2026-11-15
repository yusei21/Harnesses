# Skill multi-agent-routing — manual de engenharia

**Responsabilidade:** propor a topologia de execução mais simples que atende aos requisitos. Esta Skill não escalona nem autoriza agentes: o Rust valida os contratos e controla a execução.

**Acionamento:** planejamento de tarefas com múltiplas etapas, paralelismo, handoffs, dependências ou redução de custo. Para uma tarefa atômica, evite ativar a Skill.

**Arquivos:** [SKILL.md](../../skills/multi-agent-routing/SKILL.md), [matriz de decisão](../../skills/multi-agent-routing/references/decision-matrix.md), [contrato Rust](../../skills/multi-agent-routing/references/execution-contract.md), [casos de teste](../../skills/multi-agent-routing/evals/scenarios.json).

## Exemplo
Objetivo: analisar uma arquitetura; planejamento primeiro, pesquisa e engenharia independentes depois, revisão por último. A notação CLI Rust é:
```bash
cargo run --manifest-path rust/Cargo.toml --bin harnesses-core -- run \
  --provider ollama --model qwen3 \
  --stages 'planner|researcher,engineer|reviewer' \
  --max-calls 4 --max-parallel 2 \
  --task 'Revisar arquitetura'
```
O exemplo requer Ollama configurado e executa inferência. O cenário JSON é verificado **sem fazer chamadas LLM** por `cargo test --manifest-path rust/Cargo.toml --all-targets`.

## Aceitação
Exigir plano válido, dependências claras, sem nomes repetidos, sem execução de nuvem não consentida e justificativa para paralelismo. Casos negativos devem ser rejeitados por `validate_plan`.

## Limites
Roteamento heurístico e testes estruturais não demonstram melhora de qualidade, custo ou latência de LLMs. A execução atual não habilita ferramentas de escrita nem A2A.
