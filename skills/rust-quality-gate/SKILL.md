---
name: rust-quality-gate
description: Verifica mudanças Rust com formatação, Clippy, testes, revisão de erros e segurança, registrando resultados reais. Use para alterações em crates Rust, runtime Tokio, bridges de processo, políticas ou CI Cargo.
---

# Rust quality gate

## Entrada
Caminhos modificados, manifest Cargo, ambiente e permissões para executar comandos, critérios de CI.

## Procedimento
1. Detecte a raiz do workspace/Cargo.toml; nunca deduza que o projeto está na raiz do diretório atual.
2. Se houver executor autorizado, execute verificações nesta ordem conforme [comandos por escopo](references/checks.md): formato, lints e testes. Preserve exit code e erro real de cada uma.
3. Inspecione o diff em busca de `unsafe`, `unwrap/expect` em produção, comandos do SO, paths e operações assíncronas bloqueantes.
4. Verifique se mudanças de política, ponte Python, rede, credenciais e permissões não expandem autoridade implicitamente.
5. Se houver falha, descreva causa e ação mínima; não altere políticas para fazer a suite passar.
6. Registre claramente o que não foi executado. A revisão manual não substitui testes, nem testes garantem ausência de vulnerabilidades.

## Saída
Tabela check/comando/status/evidência; achados por severidade; riscos pendentes e próximos passos. Não declare “CI verde” sem resultado real.

## Limites
Esta Skill não executa por si só; somente ferramentas com autorização podem realizar comandos. Não rode scripts de terceiros automaticamente.

## Avaliações
Consulte [cenários](evals/scenarios.json). São critérios qualitativos, não avaliações já executadas com modelos reais.
