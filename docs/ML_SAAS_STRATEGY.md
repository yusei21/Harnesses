# Estratégia: IA/ML primeiro, SaaS quando necessário

## Perfil científico
O núcleo atende experimentação de mestrado e engenharia de IA: começar por hipótese, dados e reprodutibilidade; distinguir protocolo experimental de avaliação, evitar leakage e registrar resultados negativos. Ativar Skills apenas se tarefas demandarem o domínio, com convenções reproduzíveis e orçamento de GPU. Recomendação de workflow: `ml-experiment-design` -> `dataset-leakage-audit` -> `pytorch-training-review` (se PyTorch) -> `llm-evaluation-lab` (se LLM). Não executar todas em toda tarefa.

## Perfil RAG
RAG precisa distinguir recuperação de geração: testar recall@k/context relevance, grounding, abstenção, citações e ACL por tenant. Não usar uma única nota de LLM como verdade. Indexação e RAG local-first ainda são planejados, não implementados.

## Perfil SaaS
Para projetos freelancer, começar por MVP de monólito modular, contratos de backend, interface acessível, autorização entre tenants e processos de deploy reversíveis. Ativar `saas-system-design`, `backend-api-contracts`, `frontend-interface-review` e `saas-security-threat-model` somente conforme o escopo. Payment webhooks necessitam assinatura e idempotência.

## Organização
O registro `capabilities/catalog.json` funciona como índice de metadados. Os SKILL.md permanecem portáveis para Claude Code, Codex, OpenCode e Kimi, mas a instalação automática nesses CLIs ainda não existe. Não copiar Skills de terceiro em massa, pois isso pode aumentar o contexto e as contradições.

## Limites verificáveis
O CI valida frontmatter, links e integridade de rubricas; não executa avaliação com datasets de ML nem mede eficácia real de Skills. Antes de promover Skill como comprovada, conduzir experimento A/B com modelo e tarefas representativas, medindo qualidade, segurança, latência e custo.
