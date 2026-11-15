# rag-pipeline-evaluation — critérios e antipadrões

## Gatilhos
Assistente conectado a documentos, busca semântica ou QA com recuperação e citações.

## Contexto necessário
Corpus e permissões, consultas representativas, chunks com IDs, baseline retrieval e métricas definidas.

## Critérios
Grounding e retrieval são eixos distintos: resposta pode soar correta sem recuperação relevante. Avaliadores LLM podem servir de sinal, mas precisam ser calibrados com rótulos humanos e testes adversariais. Filtragem por tenant nunca depende do prompt.

## Procedimento de revisão
1. Defina consultas e respostas esperadas, incluindo perguntas sem resposta, documentos conflitantes e tenants distintos.
2. Avalie separadamente retrieval (recall@k, precisão/relevância, MRR ou nDCG quando apropriado) e geração (faithfulness, cobertura, abstention).
3. Audite parsing, chunking, metadados, embeddings, híbrido BM25+vetor, reranking e versionamento do índice.
4. Exija checagem de ACL antes da recuperação e novamente antes de expor trechos; filtre por tenant na consulta, não só no prompt.
5. Citações precisam referenciar IDs/trechos recuperados; sem contexto suficiente, exigir abstenção e explicitar incerteza.
6. Compare baseline por slices de idioma, origem e dificuldade; não assumir ganho por aumentar top-k ou tamanho do modelo.

## Evidência exigida
Resultados devem ter origem verificável (dataset/version/hash quando relevante, teste executado com log, ou documento identificado). Sem ambiente executor, o trabalho permanece uma proposta.
