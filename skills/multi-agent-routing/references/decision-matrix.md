# Matriz de escolha — multi-agent-routing

| Situação | Padrão sugerido | Critério de aceitação |
|---|---|---|
| Tarefa atômica, contexto cabe em uma chamada | Agente único | Resultado completo em uma etapa |
| Especialista produz evidência, titular continua responsável | Agente como ferramenta | Contrato de saída e integração definidos |
| Etapas dependentes e estado compartilhado | Pipeline sequencial | Dependências verificadas antes de avançar |
| Subtarefas independentes, entradas imutáveis | Fan-out limitado | Concorrência <= limite e agregação determinística |
| Responsabilidade transferida a outro agente/runtime | Handoff explícito | Dono atual, permissões e estado transferidos claramente |
| Múltiplos escritores no mesmo diretório | Não paralelizar ali | Exigir worktrees isolados ou serializar |
| Tarefa envolve tool call sem autorização | Bloquear | Policy engine decide, não o prompt |

## Contraexemplos
- “Explique uma função”: três subagentes acrescentam custo, não capacidade.
- “Leia três documentos independentes”: fan-out pode ajudar, mas a etapa de síntese continua sequencial.
- “Edite os mesmos arquivos em paralelo”: requer isolamento e merge; o Harnesses atual não tem essas ferramentas.
- “Responder com Claude e depois Gemini”: é pipeline sequencial, não A2A e não colaboração com ferramentas.

## Medição
Registre baseline single-agent, latência total, chamadas, custo por tarefa concluída, regressões e taxa de intervenção. Testes de plano aprovados não demonstram melhora semântica.
