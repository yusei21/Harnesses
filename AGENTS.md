# Instruções para agentes que colaboram neste repositório

A arquitetura é **Rust-first**. Leia `README.md`, `docs/RUST_CORE.md`, `docs/ARCHITECTURE.md` e `docs/SECURITY.md` antes de modificar o runtime.

- Rust controla execução, validação, budgets de chamadas, paralelismo e políticas. Python serve como adapter de modelos e experimentos.
- Não conceda ferramentas aos LLMs nem execute shell, escrita, deleção ou chamadas de efeito colateral antes de implementar gates autenticados e sandbox.
- Nunca armazene segredos em prompts, traces, git ou logs.
- Não confunda contracts de agente (`agents/*.json`) ou Skills (`skills/*/SKILL.md`) com processos invocáveis que possuam privilégios.
- Qualquer nova Skill exige `docs/skills/<nome>.md` e atualização de `docs/SKILLS.md`.
- Qualquer agente novo exige `agents/<nome>.json`, `docs/agents/<nome>.md`, inclusão na allowlist Rust e prompt da ponte Python, com testes.
- Memória e evolução devem ser propostas versionadas, nunca autopromoção de mudanças.
- Antes de PR, execute:
```bash
cargo test --manifest-path rust/Cargo.toml --all-targets
python scripts/validate_catalog.py
python -m unittest discover -s tests -v
python -m harness.cli eval
```
