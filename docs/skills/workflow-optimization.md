# Skill: workflow-optimization

**O que faz:** Otimizar processos de agentes com experimentos controlados.

**Quando usar:** Use após encontrar padrões de falha, custo ou latência nas avaliações.

**Como utilizar:** carregue `skills/workflow-optimization/SKILL.md` sob demanda após identificar intenção compatível; siga o procedimento e valide a saída. Em plataformas compatíveis, registre a pasta como Skill. No CLI offline, é apenas catálogo, não invocação automática.

**Procedimento principal:** Formule hipótese, altere uma variável, compare com baseline, execute regressões e crie PR com rollback. Nunca autopromova.

**Artefato esperado:** resultado rastreável com evidências, critérios de aceite e limites explícitos.

**O que não faz:** não executa ações externas por si só, não concede permissões e não substitui um agente ou orquestrador.

**Teste recomendado:** verifique o formato do artefato e teste um caso positivo e um adversarial antes de integrar o fluxo real.
