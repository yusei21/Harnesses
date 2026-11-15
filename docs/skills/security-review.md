# Skill: security-review

**O que faz:** Revisar riscos de prompt injection, acesso a ferramentas e segurança de agentes.

**Quando usar:** Use em integração MCP, tools, mudanças de política e operações com dados.

**Como utilizar:** carregue `skills/security-review/SKILL.md` sob demanda após identificar intenção compatível; siga o procedimento e valide a saída. Em plataformas compatíveis, registre a pasta como Skill. No CLI offline, é apenas catálogo, não invocação automática.

**Procedimento principal:** Aplique least privilege, identidade/autorização por ferramenta, redaction, validação de schemas e separação entre dados e instruções.

**Artefato esperado:** resultado rastreável com evidências, critérios de aceite e limites explícitos.

**O que não faz:** não executa ações externas por si só, não concede permissões e não substitui um agente ou orquestrador.

**Teste recomendado:** verifique o formato do artefato e teste um caso positivo e um adversarial antes de integrar o fluxo real.
