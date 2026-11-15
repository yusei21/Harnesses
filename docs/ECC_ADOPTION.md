# Incorporação seletiva de conceitos do ECC

Fonte pesquisada: [affaan-m/ECC](https://github.com/affaan-m/ECC), licença MIT, copyright (c) 2026 Affaan Mustafa. Pesquisa em 15/11/2026. **O código deste PR foi escrito para Harnesses; não é um espelho nem contém cópia substancial do ECC.**

## Por que não importar o ECC inteiro
O ECC distribui grande número de agents, Skills, comandos, hooks e integrações de múltiplos clientes. Copiar tudo aumentaria custo de contexto, superfície de execução, conflitos de regras e manutenção. O ECC tem filosofia e runtime diferentes: o Harnesses é Rust-first, Linux local-first e Python é apenas adaptador para LLM.

## Conceitos aproveitados e onde entram
| Conceito ECC | Harnesses | Garantia nesta fase |
|---|---|---|
| Skills especializadas | capabilities/catalog.json | Indexação por domínio e seleção textual |
| harness-audit | rust/src/ops.rs audit | Validação estrutural, paths, frontmatter e alguns casos de docs obsoletas |
| harness-optimizer / evolve | insights-triage + observations | Sugestões somente após recorrência, sem aplicar mudanças |
| rust-reviewer | rust-quality-gate | Procedimento de testes/lints, execução humana/CI |
| aprendizagem por projeto | memória futura | Somente desenho; não coleta sessões automaticamente |
| hooks | proposta futura | Nenhum hook externo instalado nem executado |

## Limitações atuais
- `suggest` usa gatilhos textuais, sem modelo ou RAG; não carrega Skills automaticamente na execução LLM.
- `audit` não detecta todas as contradições semânticas; serve como primeiro gate determinístico.
- `insights` analisa arquivo local explícito de observações tipadas; não acessa Claude, Codex, OpenCode, Kimi ou AGY.
- Nada escreve ~/.claude, ~/.codex ou arquivos de configuração de clientes.
- Nenhuma promoção automática, execução de shell por agente, instalação de hook, alteração de política ou uso de credenciais.

## Estratégia de interoperabilidade
Manter catálogo canônico em Harnesses e gerar adapters configuráveis em uma fase futura. Primeiro importador read-only com consentimento, depois testes contra versões reais dos CLIs instalados e somente então escrita em config com preview/diff/rollback.

## Atribuição
Créditos ao projeto original pelas ideias de harness-audit, evolução de Skills, especialização e conhecimento progressivo. Reutilização de arquivos do ECC em mudanças futuras deve preservar licença MIT e avisos de copyright e fazer revisão de segurança individual.

## Adaptação para IA/ML e SaaS — fase seguinte

Inspirada por módulos existentes no ECC, incluindo `mle-workflow`, `pytorch-patterns`, `rag-pipeline-reviewer`, `fastapi-patterns`, `frontend-patterns` e `security-review`. Não são cópias: as nove Skills foram escritas com escopo e critérios específicos do Harnesses local-first e exigem validação antes de serem declaradas eficientes.

- Pesquisa: experimentos reprodutíveis, dataset leakage, PyTorch e avaliação robusta de LLM/RAG.
- SaaS: tenancy, contratos de API, UI acessível, autenticação e autorização, billing e operações seguras.
- Formato portátil `SKILL.md` com carregamento progressivo. O catálogo Rust é fonte de metadados, não um executor de scripts do ECC.
- Nenhum arquivo de hook ou agente executável foi copiado do ECC. Respeitar licença MIT e avisos de copyright ao reutilizar conteúdo do projeto original no futuro.

Fonte: https://github.com/affaan-m/ECC (MIT, consultado em 15/11/2026).
