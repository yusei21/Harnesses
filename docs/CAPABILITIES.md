# Registro de capacidades: arquitetura local-first

## Domínios
As Skills são classificadas em planning, research, engineering, security, quality, intelligence, integrations e orchestration. Diretórios `skills/` existentes continuam portáveis. `capabilities/catalog.json` é o índice canônico; a pasta física não determina o carregamento.

## Uso
```bash
cargo run --manifest-path rust/Cargo.toml --bin harnesses-ops -- catalog
cargo run --manifest-path rust/Cargo.toml --bin harnesses-ops -- suggest "auditoria de documentacao"
cargo run --manifest-path rust/Cargo.toml --bin harnesses-ops -- audit
cargo run --manifest-path rust/Cargo.toml --bin harnesses-ops -- insights examples/observations.json
```

O comando `catalog` exibe metadados; `suggest` pontua gatilhos textuais e apresenta até cinco candidatas; `audit` confere arquivos, metadados e documentação; `insights` agrupa eventos tipados e sugere melhorias quando há ao menos duas observações iguais.

## Segurança de arquivos
O auditor proíbe `../` ou caminhos absolutos e verifica que arquivos referenciados resolvem dentro da raiz do repositório, inclusive em casos de symlink. Não interpreta nem executa scripts dentro das Skills.

## Formato das observações
JSON array de `{"kind":"test_failure","component":"cargo-test"}`. Tipos reconhecidos: test_failure, repeated_workflow, stale_documentation, tool_denied. Component deve ser um slug. Não incluir prompts, paths privados, API keys ou respostas completas de modelos. O exemplo é fictício.

## Próxima evolução
Indexer de árvore/AST e relações de documentação, importadores read-only por CLI, RAG híbrido por projeto, auditoria semântica assistida por LLM, detecção de conflitos de permissão e preview de correções em PR. Para evitar deriva, lint de documentos deve virar requisito do CI.
