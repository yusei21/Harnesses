# Estratégia de verificação — code-change

## Mínimo por tipo
| Alteração | Evidência |
|---|---|
| Bug reproduzível | Teste falhando antes + passando depois, quando há executor real |
| API pública | Testes de contrato, compatibilidade e documentação |
| Refatoração | Mesmo comportamento observável e regressão verde |
| Segurança | Revisão de autorização, entrada não confiável e testes negativos |
| Configuração de agentes | Validação do catálogo, testes Rust/Python e auditoria |

## Ordem
1. Checks focados no arquivo/contrato alterado.
2. Regressões relevantes ao módulo.
3. Verificação ampla exigida no CI.
4. Inspeção de diff e rollback.

O agente pode **sugerir** comandos. Somente a ferramenta autorizada deve executá-los; registros de execução precisam conter exit code e caminho de artefato. Não converter saída de outro agente em evidência de teste executado.
