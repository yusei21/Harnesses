# Agente: reviewer

## O que faz
Revisa criticamente textos das etapas anteriores, identifica riscos e lacunas.

## Quando utilizar
Use quando a etapa reviewer contribua de maneira verificável para a tarefa. Tarefas simples podem utilizar apenas um agente. Evite acrescentar agentes sem benefício real.

## Como usar no runtime real
Pré-requisito: configurar um provedor em [Provedores](../PROVIDERS.md). Exemplo usando Ollama local:

```bash
python -m harness.cli run --provider ollama --model qwen3 --agents reviewer --max-calls 1 --task "Analise este problema"
```

No pipeline multiprovedor, personalize o modelo deste papel por `--agent-model reviewer=openai:SEU_MODELO --allow-cloud`. A tarefa e respostas anteriores chegam como texto; não há acesso independente a ferramentas.

## Contrato
O arquivo `agents/reviewer.json` define descrição, entradas, saídas, gatilhos e risco previsto. O runtime de texto utiliza as instruções do papel declaradas em `harness/live.py`; o JSON é catálogo e guia de evolução, **não** uma especificação automaticamente aplicada ao modelo.

## Saída esperada
Texto em português com conclusões e ressalvas que permitam revisão. Na orquestração sequencial, a saída fica em `outputs[]` do JSON retornado pelo CLI.

## Restrições
Não executa testes, não aprova mudanças nem faz merge. O agente não pode se conceder aprovação. Se faltar informação, deve registrar a incerteza.

## Teste
Rode `python -m unittest discover -s tests -v` para verificar contratos e roteamento com mocks. Esse teste não garante a qualidade do conteúdo do LLM. Para avaliar qualidade, crie casos de referência com critérios objetivos e avalie usando modelo habilitado separadamente.
