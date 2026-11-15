# Skill: rust-quality-gate

**Objetivo:** Definir verificações reproduzíveis de qualidade Rust com testes e revisão de risco.

**Quando usar:** Use em alterações do runtime e integrações de ferramentas.

**Passos:**
1. Verifique formato com cargo fmt --check, depois cargo test.
2. Verifique lints com cargo clippy --all-targets -- -D warnings quando instalado.
3. Examine alterações de políticas e processos, cuidado com fuga de dados e permissões.
4. Registre resultados exatos e erros; não declare testes executados sem evidência.
5. Aplique patches somente com execução autorizada.

**Uso local:** `cargo run --manifest-path rust/Cargo.toml --bin harnesses-ops -- audit` para auditoria; `suggest QUERY` para consulta seletiva; `insights examples/observations.json` para recomendações. Essas operações são somente leitura.

**Critérios de sucesso:** resultado verificável, achados respaldados por arquivos ou eventos, nenhuma alteração automática.

**Limitações:** sem embeddings, sem auditoria semântica por LLM, sem execução de hooks e sem captura automática de sessões.
