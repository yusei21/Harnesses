# Skill: consistency-audit

**Objetivo:** Auditar divergências entre contratos, Skills, regras e documentação do repositório.

**Quando usar:** Utilize em PRs que modificam instruções, contratos ou caminhos de execução.

**Passos:**
1. Execute o auditor estrutural do Rust em modo leitura.
2. Compare regras e exemplos de CLI com o código executável.
3. Classifique achados por evidência: conflito direto, documentação obsoleta, ambiguidade.
4. Apresente arquivo, linha, regra concorrente e proposta mínima de correção.
5. Não altere arquivos até obter autorização e verificação por testes.

**Uso local:** `cargo run --manifest-path rust/Cargo.toml --bin harnesses-ops -- audit` para auditoria; `suggest QUERY` para consulta seletiva; `insights examples/observations.json` para recomendações. Essas operações são somente leitura.

**Critérios de sucesso:** resultado verificável, achados respaldados por arquivos ou eventos, nenhuma alteração automática.

**Limitações:** sem embeddings, sem auditoria semântica por LLM, sem execução de hooks e sem captura automática de sessões.
