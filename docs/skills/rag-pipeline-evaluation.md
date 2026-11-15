# Skill: rag-pipeline-evaluation

**Departamento:** rag

**O que faz:** Projetar e auditar RAG medindo retrieval, evidências, grounding, filtros por tenant e respostas sem suporte. Use ao construir ou investigar sistemas RAG, indexação vetorial, chunking, reranking e citações.

**Quando ativar:** Assistente conectado a documentos, busca semântica ou QA com recuperação e citações.

**Como utilizar:** registrar `skills/rag-pipeline-evaluation/SKILL.md` em cliente compatível ou selecionar pelo catálogo Rust com `harnesses-ops suggest`. O catálogo ainda não injeta automaticamente o arquivo no modelo. Leia as referências apenas quando forem necessárias.

**Entradas:** Corpus e permissões, consultas representativas, chunks com IDs, baseline retrieval e métricas definidas.

**Procedimento:**
1. Defina consultas e respostas esperadas, incluindo perguntas sem resposta, documentos conflitantes e tenants distintos.
2. Avalie separadamente retrieval (recall@k, precisão/relevância, MRR ou nDCG quando apropriado) e geração (faithfulness, cobertura, abstention).
3. Audite parsing, chunking, metadados, embeddings, híbrido BM25+vetor, reranking e versionamento do índice.
4. Exija checagem de ACL antes da recuperação e novamente antes de expor trechos; filtre por tenant na consulta, não só no prompt.
5. Citações precisam referenciar IDs/trechos recuperados; sem contexto suficiente, exigir abstenção e explicitar incerteza.
6. Compare baseline por slices de idioma, origem e dificuldade; não assumir ganho por aumentar top-k ou tamanho do modelo.

**Saída:** relatório com decisões, evidências, métricas pertinentes, limites, critérios de aceite e estado real dos testes.

**Arquivos:** [Skill](../../skills/rag-pipeline-evaluation/SKILL.md), [referências](../../skills/rag-pipeline-evaluation/references/checklist.md), [rubricas](../../skills/rag-pipeline-evaluation/evals/scenarios.json).

**Avaliação:** 3 cenários com expectativas positivas e falhas proibidas. CI verifica integridade, mas não mede ainda qualidade do LLM.

**Limites:** nenhum script executado automaticamente, nenhum acesso a dados privados, sem autopromoção de mudanças.
