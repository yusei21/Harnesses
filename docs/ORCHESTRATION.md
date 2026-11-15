# Orquestração multiagente

## Heurística de escolha
1. Tarefa pequena, poucos passos: um agente e ferramentas.
2. Pesquisa independente de execução: `researcher` como ferramenta, retornando evidências.
3. Mudança de código: `planner` (opcional) -> `engineer` -> `reviewer` -> `evaluator`.
4. Verificações paralelizáveis: paralelizar pesquisa, teste e revisão **somente** se não houver dependências.
5. Workflow regulado ou efeitos colaterais: máquina de estados determinística com aprovação humana.

## Evitar
Debates artificiais entre agentes, loops sem teto, agentes com credenciais amplas, subagentes que compartilham todas as memórias, múltiplos especialistas trabalhando simultaneamente nos mesmos arquivos e revisão pelo próprio autor.

## Contrato de delegação
```json
{
  "task_id": "uuid",
  "agent": "researcher",
  "goal": "Verificar requisitos da API",
  "context_refs": ["doc-id"],
  "allowed_tools": ["read"],
  "deadline_seconds": 120,
  "max_steps": 8,
  "budget_usd": 0.2,
  "output_schema": "evidence-v1"
}
```

Na implementação real, o escalonador deve impor os limites, não apenas descrevê-los no prompt. Fan-out recomendado inicialmente: máximo 3, profundidade 2, com cancelamento propagado. São defaults de projeto, não limites universais.

## Modos de falha
Se ferramenta estiver indisponível, falhe explicitamente ou use fallback autorizado. Nunca finja que executou uma etapa. Agregue evidência e falhas parciais; não passe raciocínio privado entre agentes.
