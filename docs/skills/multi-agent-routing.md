# Skill: multi-agent-routing

**O que faz:** Escolher entre agente único, agente como ferramenta, handoff e paralelismo.

**Quando usar:** Use quando houver tarefas independentes ou contextos especializados.

**Como utilizar:** carregue `skills/multi-agent-routing/SKILL.md` sob demanda após identificar intenção compatível; siga o procedimento e valide a saída. Em plataformas compatíveis, registre a pasta como Skill. No CLI offline, é apenas catálogo, não invocação automática.

**Procedimento principal:** Prefira agente único. Delegue apenas quando houver benefício verificável; limite fanout, profundidade e orçamento; sintetize artefatos, não raciocínios privados.

**Artefato esperado:** resultado rastreável com evidências, critérios de aceite e limites explícitos.

**O que não faz:** não executa ações externas por si só, não concede permissões e não substitui um agente ou orquestrador.

**Teste recomendado:** verifique o formato do artefato e teste um caso positivo e um adversarial antes de integrar o fluxo real.
