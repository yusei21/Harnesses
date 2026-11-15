# Decisões arquiteturais

## ADR-001 — Rust-first com Python como adaptador de IA
Rust controla workflow, limites, concorrência e políticas; Python mantém provedores HTTP existentes e pesquisa/avaliações. Evita reescrita prematura dos conectores.

## ADR-002 — Tokio para etapas paralelas
Conjunto limitado de tarefas async; cada estágio espera todos os seus agentes e preserva ordem estável de apresentação. Não usar debates recursivos infinitos.

## ADR-003 — Ponte JSON entre processos
Esquema simples e testável, sem APIs Python in-process. Ponte executa módulo fixo do repositório, sem invocar comandos do modelo. Não constitui sandbox.

## ADR-004 — Políticas fail-closed
Ações read são permitidas pela política abstrata; escrita/external/destructive são bloqueadas. Não existem ferramentas reais autorizáveis nesta versão.

## ADR-005 — LLM e Skills não elevam privilégios
Respostas anteriores são dados não confiáveis. Skills são documentação carregável manualmente, sem execução automática. Uma futura integração MCP requer aprovação autenticada.

## ADR-006 — Experimentação e evolução
Python disponível para DSPy/evals. Evolução deve gerar proposals/PRs testados, nunca autopromover. Não há memória persistente ativa.

## ADR-007 — Compatibilidade
Manter CLI Python antigo enquanto testes e documentação dos adapters permanecem, mas orientar novos usuários a usar Rust.
