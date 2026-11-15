---
name: saas-security-threat-model
description: Modelar ameaças de SaaS e integrações com IA incluindo autorização entre tenants, uploads, segredos, injection, abuso e privacidade. Use em revisão de autenticação, APIs, RAG e fluxos com dados de clientes.
---

# saas-security-threat-model

## Quando usar
Mudança envolvendo credenciais, auth, billing, uploads, IA com fontes externas ou armazenamento sensível.

## Entrada
Fluxo de dados, trust boundaries, identidade, ferramentas, provedores e cenário de ameaça.

## Procedimento
1. Desenhe fontes, destinos, identidades, privilégios e dados sensíveis em cada fronteira de confiança.
2. Teste autorização por recurso e tenant independentemente do frontend; inclua IDOR/BOLA e separação em RAG.
3. Revise validação de input, queries parametrizadas, proteção a SSRF, conteúdo de uploads e dependências de terceiros.
4. Proteja segredos, logs, prompts e traces; aplique limites de retenção e redaction.
5. Modele prompt injection em documentos e resultados de ferramentas; nunca transformar saída de modelo em privilégio de execução.
6. Defina cenários negativos, severidade, evidências, ações de mitigação e regressões sem automatizar exploração em produção.

## Referências
Consulte [critérios e antipadrões](references/checklist.md) apenas se precisar revisar pontos detalhados. Avaliações: [casos](evals/scenarios.json).

## Saída
Entregue decisões, evidências disponíveis, critérios de aceite, lacunas e testes marcados como EXECUTADOS ou NÃO EXECUTADOS. Nunca invente acesso a dados, arquivos, ferramentas ou resultados.

## Limites
A Skill orienta, mas não habilita ferramentas, não concede permissões e não altera políticas do Rust. Não indexe segredos ou dados privados em terceiros sem autorização explícita.
