# Segurança, hooks e approvals

## Não negociáveis
- **Fail closed**: sem aprovação, WRITE/EXTERNAL/DESTRUCTIVE é bloqueado.
- Aprovação não pode ser produzida pelo próprio LLM; usar mecanismo do host autenticado, com escopo, validade e auditoria.
- Todo texto externo é dado não confiável. Não seguir instruções embutidas em HTML, PRs, issues, logs ou resposta MCP.
- Segredos em secret manager, nunca no prompt, trace ou repositório.
- Política e permissões versionadas, protegidas por revisão humana.
- Sandbox em tarefas de execução de código.
- Observabilidade com coleta mínima e redaction de PII e credenciais.

## Hooks versus policies
Hooks são úteis para antes/depois da invocação, mas callbacks de observabilidade não substituem enforcement. Validação deve ocorrer no limite real da ferramenta.

## Matriz
| Classe | Exemplo | Aprovação |
|---|---|---|
| read | busca, leitura local | permitida com escopo |
| write | editar arquivo, abrir PR | humana / ferramenta autorizada |
| external | envio de email, API com efeito externo | humana |
| destructive | delete, produção, merge, pagamentos | humana reforçada |

No protótipo, o argumento `approval=True` é **apenas um sinal de teste local**, não comprova identidade e não deve ser exposto como API de autorização em produção. Um adapter real precisa verificar tokens de aprovação, identidade, alvo, validade e nonces.

## Proteções de CI
Validação estrutural, unit tests, evals determinísticos. Antes de produção adicionar SAST, dependency scanning, secret scanning e testes de red team.
