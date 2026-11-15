# Skill: insights-triage

**Objetivo:** Analisar eventos locais sanitizados e sugerir Skills, hooks ou refatorações.

**Quando usar:** Use após múltiplas falhas ou padrões repetidos de trabalho.

**Passos:**
1. Recolha apenas eventos autorizados, sem prompts, credenciais ou conteúdo privado.
2. Execute insights no Rust sobre o arquivo JSON explícito.
3. Considere recorrência e custo; evite recomendar Skills para um único evento.
4. Gere proposta com evidências, métrica de melhora e plano de rollback.
5. Não instale hooks nem altere permissões automaticamente.

**Uso local:** `cargo run --manifest-path rust/Cargo.toml --bin harnesses-ops -- audit` para auditoria; `suggest QUERY` para consulta seletiva; `insights examples/observations.json` para recomendações. Essas operações são somente leitura.

**Critérios de sucesso:** resultado verificável, achados respaldados por arquivos ou eventos, nenhuma alteração automática.

**Limitações:** sem embeddings, sem auditoria semântica por LLM, sem execução de hooks e sem captura automática de sessões.
