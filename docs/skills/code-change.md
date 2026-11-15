# Skill code-change — manual de engenharia

**Função:** orientar mudanças pequenas, seguras e verificáveis, distinguindo claramente proposta textual de alteração executada. Utilize para bugs, funcionalidades, refatorações e mudanças de API.

**Material:** [SKILL.md](../../skills/code-change/SKILL.md), [seleção de verificadores](../../skills/code-change/references/verification.md), [avaliações](../../skills/code-change/evals/scenarios.json).

## Exemplo
Ao receber “corrija a API”, identificar o contrato afetado e os testes relevantes. Na implementação atual, os agentes LLM não têm ferramentas de edição. Portanto, a saída correta é proposta de patch, lista de testes como **NÃO EXECUTADOS** e plano de rollback — jamais “corrigi e passei nos testes”.

## Critérios de aceite
Mudança limitada ao escopo; reprodução/impacto descritos; verificações relevantes; riscos explicitados; rollback documentado. Se houver execução real autorizada futuramente, anexar logs e códigos de saída.

## Avaliação
O arquivo `evals/scenarios.json` registra 3 cenários com comportamentos positivos e falhas proibidas. O CI verifica a estrutura e a integridade desses casos; a qualidade das respostas de modelos exige avaliação independente futura.

## Limites
Não conceder permissões, executar deploy ou inferir sucesso apenas de texto gerado por LLM.
