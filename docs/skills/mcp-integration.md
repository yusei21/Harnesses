# Skill: mcp-integration

**O que faz:** Projetar conectores MCP com limites, schemas e isolamento.

**Quando usar:** Use quando acessar ferramentas/recursos externos via MCP.

**Como utilizar:** carregue `skills/mcp-integration/SKILL.md` sob demanda após identificar intenção compatível; siga o procedimento e valide a saída. Em plataformas compatíveis, registre a pasta como Skill. No CLI offline, é apenas catálogo, não invocação automática.

**Procedimento principal:** Identifique servidor confiável, capacidades, permissões, timeouts e validação de entradas/saídas. MCP é conexão com ferramentas, não A2A.

**Artefato esperado:** resultado rastreável com evidências, critérios de aceite e limites explícitos.

**O que não faz:** não executa ações externas por si só, não concede permissões e não substitui um agente ou orquestrador.

**Teste recomendado:** verifique o formato do artefato e teste um caso positivo e um adversarial antes de integrar o fluxo real.
