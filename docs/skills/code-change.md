# Skill: code-change

**O que faz:** Planejar alterações de software com escopo pequeno, testes e rollback.

**Quando usar:** Use para bugs, implementação e refatoração.

**Como utilizar:** carregue `skills/code-change/SKILL.md` sob demanda após identificar intenção compatível; siga o procedimento e valide a saída. Em plataformas compatíveis, registre a pasta como Skill. No CLI offline, é apenas catálogo, não invocação automática.

**Procedimento principal:** Inspecione contexto, proponha diff mínimo, execute testes em sandbox quando disponível. Solicite aprovação externa para escrita real, deploy e merge.

**Artefato esperado:** resultado rastreável com evidências, critérios de aceite e limites explícitos.

**O que não faz:** não executa ações externas por si só, não concede permissões e não substitui um agente ou orquestrador.

**Teste recomendado:** verifique o formato do artefato e teste um caso positivo e um adversarial antes de integrar o fluxo real.
