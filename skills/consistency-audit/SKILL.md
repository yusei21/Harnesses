---
name: consistency-audit
description: Auditar divergências entre contratos, Skills, regras e documentação do repositório.
---

# consistency audit

Utilize em PRs que modificam instruções, contratos ou caminhos de execução.

## Procedimento
1. Execute o auditor estrutural do Rust em modo leitura.
2. Compare regras e exemplos de CLI com o código executável.
3. Classifique achados por evidência: conflito direto, documentação obsoleta, ambiguidade.
4. Apresente arquivo, linha, regra concorrente e proposta mínima de correção.
5. Não altere arquivos até obter autorização e verificação por testes.

## Referências
- [Manual](../../docs/skills/consistency-audit.md)

## Limites
Esta Skill é conhecimento procedimental, não um grant de ferramenta. Não ativa scripts nem permissões por conta própria.
