# Documentação do Projeto crypto-listener

Este diretório contém toda a documentação técnica do projeto.

## Convenções de Nomenclatura

Todos os arquivos de documentação seguem padrões específicos para facilitar a organização e localização:

### Padrões de Nomenclatura

1. **Guias de Migração**: `{FEATURE}_MIGRATION.md`
   - Exemplo: `KAFKA_TOPICS_MIGRATION.md`
   - Propósito: Documentar processos de migração de features ou mudanças breaking

2. **Architecture Decision Records (ADRs)**: `ADR_{NUMBER}_{TITLE}.md`
   - Exemplo: `ADR_001_KAFKA_AS_PRIMARY_DATASTORE.md`
   - Propósito: Registrar decisões arquiteturais importantes com contexto e justificativa

3. **Guias How-To**: `HOWTO_{TOPIC}.md`
   - Exemplo: `HOWTO_SETUP_LOCAL_DEVELOPMENT.md`
   - Propósito: Tutoriais práticos para tarefas específicas

4. **Especificações Técnicas**: `SPEC_{FEATURE}.md`
   - Exemplo: `SPEC_KAFKA_MESSAGE_FORMAT.md`
   - Propósito: Especificações detalhadas de features, protocolos ou formatos

5. **Runbooks Operacionais**: `RUNBOOK_{SCENARIO}.md`
   - Exemplo: `RUNBOOK_KAFKA_OUTAGE.md`
   - Propósito: Procedimentos para cenários operacionais e troubleshooting

## Regras Gerais

- ✅ **SEMPRE** coloque novos arquivos `.md` nesta pasta `/docs`
- ✅ Use nomes descritivos em MAIÚSCULAS com underscores
- ✅ Inclua data de criação e versão quando relevante
- ✅ Mantenha o `projectmap.yaml` atualizado com novos documentos
- ✅ Use Markdown padrão com blocos de código bem formatados
- ✅ Adicione índice para documentos longos (>500 linhas)

## Estrutura Recomendada para Documentos

```markdown
# Título do Documento

**Criado:** YYYY-MM-DD  
**Última Atualização:** YYYY-MM-DD  
**Versão:** X.Y.Z  
**Status:** [Draft | Review | Approved | Deprecated]

## Índice
1. [Seção 1](#seção-1)
2. [Seção 2](#seção-2)
...

## Resumo
Breve descrição (2-3 parágrafos)

## Conteúdo Principal
...

## Referências
- Links relevantes
- Documentos relacionados
```

## Documentos Existentes

### Migrações
- **KAFKA_TOPICS_MIGRATION.md** - Guia de migração para novo padrão de nomenclatura de tópicos Kafka (`crypto-listener-{topic-name}`)

## Contribuindo

Ao adicionar nova documentação:

1. Escolha o padrão de nomenclatura apropriado
2. Crie o arquivo em `/docs`
3. Atualize este README com uma entrada na seção apropriada
4. Atualize o `projectmap.yaml` na seção `project.documentation.existing_files`

## Manutenção

- Revise documentos a cada release
- Marque documentos obsoletos como `[DEPRECATED]` no título
- Mova documentos deprecados para `/docs/archive/` após 6 meses

