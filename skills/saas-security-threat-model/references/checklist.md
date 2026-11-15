# saas-security-threat-model — critérios e antipadrões

## Gatilhos
Mudança envolvendo credenciais, auth, billing, uploads, IA com fontes externas ou armazenamento sensível.

## Contexto necessário
Fluxo de dados, trust boundaries, identidade, ferramentas, provedores e cenário de ameaça.

## Critérios
Validação de schema não é autorização. Prompt injection e dados recuperados são não confiáveis, e um LLM não decide permissões. Revisão defensiva deve privilegiar testes em ambiente controlado.

## Procedimento de revisão
1. Desenhe fontes, destinos, identidades, privilégios e dados sensíveis em cada fronteira de confiança.
2. Teste autorização por recurso e tenant independentemente do frontend; inclua IDOR/BOLA e separação em RAG.
3. Revise validação de input, queries parametrizadas, proteção a SSRF, conteúdo de uploads e dependências de terceiros.
4. Proteja segredos, logs, prompts e traces; aplique limites de retenção e redaction.
5. Modele prompt injection em documentos e resultados de ferramentas; nunca transformar saída de modelo em privilégio de execução.
6. Defina cenários negativos, severidade, evidências, ações de mitigação e regressões sem automatizar exploração em produção.

## Evidência exigida
Resultados devem ter origem verificável (dataset/version/hash quando relevante, teste executado com log, ou documento identificado). Sem ambiente executor, o trabalho permanece uma proposta.
