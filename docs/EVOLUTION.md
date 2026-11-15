# Memória e autoevolução controlada

## Memória
Três níveis: (1) estado de sessão/checkpoint; (2) fatos e procedimentos validados; (3) propostas de otimização. Não confundir todos com histórico de chat. Cada memória deve ter ID, fonte, timestamp, escopo, validade, classificação de sensibilidade e responsável.

## Loop proposto
1. Coletar traces **sanitizados** com métricas e falhas.
2. Agrupar padrões recorrentes e separar ruído.
3. Curador propõe atualização de memória com evidência.
4. Otimizador gera proposta de mudança de Skill/prompt/roteamento.
5. Executar evals de desenvolvimento + holdout, segurança, custo, latência e qualidade.
6. Reviewer verifica regressões, origem e riscos.
7. Criar PR versionado; humano aprova e faz merge.
8. Monitorar canary e permitir rollback.

## Proibição de auto-promoção
Não alterar políticas, credenciais, lista de ferramentas, acesso a produção ou evals de referência no mesmo ciclo de otimização. Sem deploy automático por agente. É possível gerar propostas, mas o harness inicial não faz isso: não há memory store ou optimizer em execução.

## Métricas
Task success, groundedness, regressões, tool-call errors, custos por tarefa, tempo, taxa de intervenções, recuperações após falha, incidentes de autorização e valor agregado pela memória.

## Evitar Goodhart
Holdout secreto, revisão humana das rubricas, tarefas novas e rastreio de dados de treino/avaliação. Uma melhora em benchmark isolado não implica melhora geral.
