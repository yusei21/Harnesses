# Contrato do runtime Rust

O Rust implementa atualmente `Plan { goal, stages, max_calls, max_parallel, allow_cloud }`; cada `Stage` contém `agents`, e cada agente precisa de `name` e `route { provider, model }`.

Exemplo de plano **válido** com modelos locais:

```json
{
  "goal": "Revisar uma proposta de arquitetura",
  "stages": [
    {"agents": [{"name": "planner", "route": {"provider": "ollama", "model": "qwen3"}}]},
    {"agents": [
      {"name": "researcher", "route": {"provider": "ollama", "model": "qwen3"}},
      {"name": "engineer", "route": {"provider": "ollama", "model": "qwen3"}}
    ]},
    {"agents": [{"name": "reviewer", "route": {"provider": "ollama", "model": "qwen3"}}]}
  ],
  "max_calls": 4,
  "max_parallel": 2,
  "allow_cloud": false
}
```

Validações do Rust: máximo de 5 agentes, 3 simultâneos, nomes não repetidos, modelos e provedores aprovados e `allow_cloud` antes de qualquer chamada remota. `max_calls` não representa teto monetário. Esta Skill não instala nem executa ferramentas, não pode conceder autorização e não garante escalonamento dinâmico. A CLI de inferência aceita a notação `--stages 'planner|researcher,engineer|reviewer'`.
