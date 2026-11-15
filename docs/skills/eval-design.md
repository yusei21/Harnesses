# Skill: eval-design

**O que faz:** Criar conjuntos de testes e rubricas com baseline, regressão e critérios.

**Quando usar:** Use para medir qualidade de agentes ou comparar mudanças.

**Como utilizar:** carregue `skills/eval-design/SKILL.md` sob demanda após identificar intenção compatível; siga o procedimento e valide a saída. Em plataformas compatíveis, registre a pasta como Skill. No CLI offline, é apenas catálogo, não invocação automática.

**Procedimento principal:** Separe evals determinísticos de avaliações com LLM. Inclua casos negativos, permissões, latência, custo e qualidade. Evite treinar e testar sobre os mesmos casos.

**Artefato esperado:** resultado rastreável com evidências, critérios de aceite e limites explícitos.

**O que não faz:** não executa ações externas por si só, não concede permissões e não substitui um agente ou orquestrador.

**Teste recomendado:** verifique o formato do artefato e teste um caso positivo e um adversarial antes de integrar o fluxo real.
