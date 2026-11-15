---
name: rust-quality-gate
description: Definir verificações reproduzíveis de qualidade Rust com testes e revisão de risco.
---

# rust quality gate

Use em alterações do runtime e integrações de ferramentas.

## Procedimento
1. Verifique formato com cargo fmt --check, depois cargo test.
2. Verifique lints com cargo clippy --all-targets -- -D warnings quando instalado.
3. Examine alterações de políticas e processos, cuidado com fuga de dados e permissões.
4. Registre resultados exatos e erros; não declare testes executados sem evidência.
5. Aplique patches somente com execução autorizada.

## Referências
- [Manual](../../docs/skills/rust-quality-gate.md)

## Limites
Esta Skill é conhecimento procedimental, não um grant de ferramenta. Não ativa scripts nem permissões por conta própria.
