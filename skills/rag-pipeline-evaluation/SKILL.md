---
name: rag-pipeline-evaluation
description: Projetar e auditar RAG medindo retrieval, evidências, grounding, filtros por tenant e respostas sem suporte. Use ao construir ou investigar sistemas RAG, indexação vetorial, chunking, reranking e citações.
---

# rag-pipeline-evaluation

## Quando usar
Assistente conectado a documentos, busca semântica ou QA com recuperação e citações.

## Entrada
Corpus e permissões, consultas representativas, chunks com IDs, baseline retrieval e métricas definidas.

## Procedimento
1. Defina consultas e respostas esperadas, incluindo perguntas sem resposta, documentos conflitantes e tenants distintos.
2. Avalie separadamente retrieval (recall@k, precisão/relevância, MRR ou nDCG quando apropriado) e geração (faithfulness, cobertura, abstention).
3. Audite parsing, chunking, metadados, embeddings, híbrido BM25+vetor, reranking e versionamento do índice.
4. Exija checagem de ACL antes da recuperação e novamente antes de expor trechos; filtre por tenant na consulta, não só no prompt.
5. Citações precisam referenciar IDs/trechos recuperados; sem contexto suficiente, exigir abstenção e explicitar incerteza.
6. Compare baseline por slices de idioma, origem e dificuldade; não assumir ganho por aumentar top-k ou tamanho do modelo.

## Referências
Consulte [critérios e antipadrões](references/checklist.md) apenas se precisar revisar pontos detalhados. Avaliações: [casos](evals/scenarios.json).

## Saída
Entregue decisões, evidências disponíveis, critérios de aceite, lacunas e testes marcados como EXECUTADOS ou NÃO EXECUTADOS. Nunca invente acesso a dados, arquivos, ferramentas ou resultados.

## Limites
A Skill orienta, mas não habilita ferramentas, não concede permissões e não altera políticas do Rust. Não indexe segredos ou dados privados em terceiros sem autorização explícita.
