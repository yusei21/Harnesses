# Skill: saas-security-threat-model

**Departamento:** security

**O que faz:** Modelar ameaças de SaaS e integrações com IA incluindo autorização entre tenants, uploads, segredos, injection, abuso e privacidade. Use em revisão de autenticação, APIs, RAG e fluxos com dados de clientes.

**Quando ativar:** Mudança envolvendo credenciais, auth, billing, uploads, IA com fontes externas ou armazenamento sensível.

**Como utilizar:** registrar `skills/saas-security-threat-model/SKILL.md` em cliente compatível ou selecionar pelo catálogo Rust com `harnesses-ops suggest`. O catálogo ainda não injeta automaticamente o arquivo no modelo. Leia as referências apenas quando forem necessárias.

**Entradas:** Fluxo de dados, trust boundaries, identidade, ferramentas, provedores e cenário de ameaça.

**Procedimento:**
1. Desenhe fontes, destinos, identidades, privilégios e dados sensíveis em cada fronteira de confiança.
2. Teste autorização por recurso e tenant independentemente do frontend; inclua IDOR/BOLA e separação em RAG.
3. Revise validação de input, queries parametrizadas, proteção a SSRF, conteúdo de uploads e dependências de terceiros.
4. Proteja segredos, logs, prompts e traces; aplique limites de retenção e redaction.
5. Modele prompt injection em documentos e resultados de ferramentas; nunca transformar saída de modelo em privilégio de execução.
6. Defina cenários negativos, severidade, evidências, ações de mitigação e regressões sem automatizar exploração em produção.

**Saída:** relatório com decisões, evidências, métricas pertinentes, limites, critérios de aceite e estado real dos testes.

**Arquivos:** [Skill](../../skills/saas-security-threat-model/SKILL.md), [referências](../../skills/saas-security-threat-model/references/checklist.md), [rubricas](../../skills/saas-security-threat-model/evals/scenarios.json).

**Avaliação:** 3 cenários com expectativas positivas e falhas proibidas. CI verifica integridade, mas não mede ainda qualidade do LLM.

**Limites:** nenhum script executado automaticamente, nenhum acesso a dados privados, sem autopromoção de mudanças.
