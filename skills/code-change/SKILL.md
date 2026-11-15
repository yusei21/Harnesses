---
name: code-change
description: Planeja e verifica correções, implementações e refatorações de software com impacto limitado, testes e rollback. Use ao receber pedido de bugfix, mudança em API, patch ou revisão de código em repositórios com ferramentas autorizadas.
---

# Verified code change

## Entrada
Objetivo, requisitos e critérios de aceite; escopo do repositório; caminhos alteráveis; ferramentas e permissões realmente disponíveis.

## Procedimento
1. Confirme o estado do projeto e os requisitos; identifique testes e referências diretamente relacionados.
2. Registre comportamento atual e uma reprodução mínima do problema quando possível.
3. Faça uma avaliação de impacto: dependentes, interfaces públicas, riscos de migração, regras do repositório.
4. Proponha um diff pequeno e reversível; preserve comportamento não relacionado. Evite refatoração ampla no mesmo patch.
5. **Somente se houver ferramenta aprovada e sandbox:** aplique o patch, execute testes relevantes e capture evidências reais. Sem ferramentas, produza uma proposta de patch e marque todas as verificações como **não executadas**.
6. Revise segurança, compatibilidade, documentação e mudanças colaterais. Separe bloqueios de melhorias opcionais.
7. Informe como reverter e quais critérios de aceite foram efetivamente demonstrados.

## Escolha de verificadores
Consulte [estratégia de testes](references/verification.md) apenas quando precisar selecionar checks. Para Rust, carregue a Skill `rust-quality-gate` quando o problema exigir validação Rust.

## Saída
Arquivo(s) impactados; patch ou proposta; comandos e status dos testes (**executado**, **falhou**, **não executado**); riscos e rollback.

## Não fazer
Não afirmar execução de shell, edição, testes ou acesso ao repositório se a integração não os forneceu. Não fazer merge, deploy ou conceder permissões.

## Avaliação
Use [casos de aceitação](evals/scenarios.json) para avaliar a estrutura da resposta; qualidade funcional exige testes reais e baseline.
