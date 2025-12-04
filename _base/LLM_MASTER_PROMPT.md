# 🤖 LLM MASTER PROMPT - Crypto Trading Ecosystem

> **Versão:** 2.0  
> **Última Atualização:** Dezembro 2025  
> **Tipo:** Prompt Universal para Implementação e Continuação  
> **Aplicável a:** Todos os 6 projetos do ecossistema

---

## 🎯 PROPÓSITO

Este é o **ÚNICO PROMPT** que você precisa para:
1. ✅ Iniciar implementação de qualquer projeto do ecossistema
2. ✅ Continuar de onde parou (mesmo após erros ou interrupções)
3. ✅ Respeitar fronteiras entre projetos (CRÍTICO!)
4. ✅ Seguir padrões arquiteturais estabelecidos

---

## 🌐 VISÃO GERAL DO ECOSSISTEMA

Este ecossistema consiste em **6 microserviços Rust** que se comunicam **EXCLUSIVAMENTE via Kafka**.

```
┌─────────────────────────────────────────────────────────────────┐
│                         KAFKA CLUSTER                            │
│  Topics: prices, signals, orders, notifications, management     │
└─────────────────────────────────────────────────────────────────┘
         ▲                    ▲ │ ▼                    ▲
         │                    │ │ │                    │
┌────────┴────────┐  ┌────────┴─┴─┴────────┐  ┌───────┴────────┐
│ crypto-listener │  │   crypto-signals    │  │ crypto-webhook │
│ "EU ESCUTO"     │  │   "EU ANALISO"      │  │ "EU NORMALIZO" │
│ WebSocket→Kafka │  │   Prices→Signals    │  │ HTTP→Kafka     │
└────────┬────────┘  └────────┬────────────┘  └───────┬────────┘
         │                    │                        │
         └──────────┬─────────┴────────────────────────┘
                    ▼
┌───────────────────────────────────────────────────────────────┐
│                      crypto-trader                             │
│                      "EU EXECUTO"                              │
│                   Signals → Binance API → Orders               │
└──────────────────────────┬────────────────────────────────────┘
                           │
         ┌─────────────────┼─────────────────┐
         ▼                 ▼                 ▼
┌─────────────────┐ ┌──────────────────┐ ┌──────────────────┐
│crypto-management│ │crypto-notificati.│ │   (Monitoring)   │
│ "EU ORQUESTRO"  │ │  "EU NOTIFICO"   │ │   Dashboards     │
│ Posições, Risco │ │ Telegram/Discord │ │                  │
└─────────────────┘ └──────────────────┘ └──────────────────┘
```

### Tabela de Projetos

| Projeto | Mantra | Função | Entrada | Saída |
|---------|--------|--------|---------|-------|
| **crypto-listener** | "EU ESCUTO O MERCADO" | Ingestão via WebSocket | Binance WS | `crypto-listener.prices` |
| **crypto-webhook** | "EU RECEBO E NORMALIZO" | Gateway HTTP | HTTP POST | `signals.buy/sell` |
| **crypto-signals** | "EU ANALISO E SINALIZO" | Análise técnica | `*.prices` | `signals.buy/sell` |
| **crypto-trader** | "EU EXECUTO ORDENS" | Execução na exchange | `signals.*` | `orders.events` |
| **crypto-management** | "EU ORQUESTRO" | Posições e risco | `orders.*`, `signals.*` | `management.*` |
| **crypto-notifications** | "EU NOTIFICO" | Alertas multi-canal | `notifications.send` | Telegram/Discord |

---

## 🚀 INÍCIO RÁPIDO (EXECUTE NESTA ORDEM)

### PASSO 1: Identificar Projeto Atual

```
❓ Em qual projeto estou trabalhando?

Resposta: crypto-[listener|webhook|signals|trader|management|notifications]
```

### PASSO 2: Carregar Contexto (OBRIGATÓRIO)

Leia os arquivos **NESTA ORDEM**:

```yaml
1. PROGRESSO:
   - docs/IMPLEMENTATION_PROGRESS.md  # Estado atual
   
2. FRONTEIRAS DO PROJETO (CRÍTICO!):
   - _base/{projeto}/BOUNDARIES.md    # O que EU posso/não posso fazer
   
3. ESPECIFICAÇÃO TÉCNICA:
   - _base/{projeto}/projectmap.yaml  # Estrutura e features
   
4. PADRÕES DE CÓDIGO:
   - _base/IMPLEMENTATION_PATTERNS.md # Como implementar
   
5. SE TIVER DÚVIDA SOBRE RESPONSABILIDADES:
   - _base/WHICH_PROJECT_DOES_WHAT.md # Quem faz o quê
   - _base/BOUNDARIES_GUIDE.md        # Guia geral de fronteiras
```

### PASSO 3: Identificar Próxima Tarefa

Do arquivo `IMPLEMENTATION_PROGRESS.md`, extraia:
- Qual é a **FASE ATUAL**?
- Qual é o **PROGRESSO** (%)?
- Qual é a **PRÓXIMA TAREFA**? ← **SEU FOCO**
- Há **ERROS BLOQUEANTES**?

---

## 🚨 REGRAS ABSOLUTAS (VIOLAÇÃO = FALHA)

### Regra 1: Comunicação APENAS via Kafka

```rust
// ✅ CORRETO
kafka.send("signals.buy", signal).await;

// ❌ PROIBIDO - Chamar outro serviço via HTTP
http_client.post("http://crypto-trader/api/order").await;

// ❌ PROIBIDO - Acessar banco de outro projeto
db.query("SELECT * FROM management.positions").await;

// ❌ PROIBIDO - Importar código de outro projeto
use crypto_trader::domain::Order;
```

### Regra 2: Cada Projeto = Uma Responsabilidade

```yaml
crypto-listener:
  ✅ PERMITIDO: Conectar Binance WebSocket, construir candles, publicar preços
  ❌ PROIBIDO:  Calcular indicadores, gerar sinais, executar ordens

crypto-signals:
  ✅ PERMITIDO: Calcular RSI/MACD/EMA, gerar sinais BUY/SELL
  ❌ PROIBIDO:  Executar ordens, enviar notificações, conectar WebSocket

crypto-trader:
  ✅ PERMITIDO: Executar ordens na Binance, gerenciar stops de UMA ordem
  ❌ PROIBIDO:  Calcular indicadores, gerenciar portfolio, enviar notificações

crypto-management:
  ✅ PERMITIDO: Gerenciar posições globais, calcular P&L, risk management
  ❌ PROIBIDO:  Executar ordens diretamente, calcular indicadores

crypto-notifications:
  ✅ PERMITIDO: Formatar mensagens, enviar Telegram/Discord/Email
  ❌ PROIBIDO:  Decidir QUANDO notificar (apenas COMO), executar qualquer lógica

crypto-webhook:
  ✅ PERMITIDO: Receber HTTP, validar, normalizar, publicar no Kafka
  ❌ PROIBIDO:  Executar ordens, analisar sinais, melhorar dados
```

### Regra 3: Quando em Dúvida, PARE e Consulte

```
❓ "Devo implementar X neste projeto?"

ANTES de implementar:
1. Leia _base/{projeto}/BOUNDARIES.md
2. Verifique se X está nas "responsibilities"
3. Confirme que X NÃO está na lista "❌ PROIBIDO"
4. Se X pertence a outro projeto → NÃO IMPLEMENTE

Se violação detectada, responda:
"❌ VIOLAÇÃO DE FRONTEIRA: [X] pertence a [outro_projeto], não a [projeto_atual]"
```

---

## 🔄 CICLO DE IMPLEMENTAÇÃO

```
┌─────────────────────────────────────────────────────────────┐
│  1. CARREGAR CONTEXTO                                       │
│     - Ler IMPLEMENTATION_PROGRESS.md                        │
│     - Ler BOUNDARIES.md do projeto                          │
│     - Ler projectmap.yaml                                   │
└───────────────────────────┬─────────────────────────────────┘
                            ▼
┌─────────────────────────────────────────────────────────────┐
│  2. IDENTIFICAR PRÓXIMA TAREFA                              │
│     - Fase atual e progresso                                │
│     - Próxima tarefa pendente                               │
│     - Erros bloqueantes (resolver PRIMEIRO)                 │
└───────────────────────────┬─────────────────────────────────┘
                            ▼
┌─────────────────────────────────────────────────────────────┐
│  3. VALIDAR FRONTEIRAS (CRÍTICO!)                           │
│     - Tarefa pertence a este projeto?                       │
│     - Não está na lista de proibições?                      │
│     - Comunicação é via Kafka?                              │
└───────────────────────────┬─────────────────────────────────┘
                            ▼
┌─────────────────────────────────────────────────────────────┐
│  4. PLANEJAR IMPLEMENTAÇÃO                                  │
│     - Arquivos a criar/modificar                            │
│     - Padrões a seguir (IMPLEMENTATION_PATTERNS.md)         │
│     - Dependências necessárias                              │
└───────────────────────────┬─────────────────────────────────┘
                            ▼
┌─────────────────────────────────────────────────────────────┐
│  5. IMPLEMENTAR                                             │
│     - Seguir padrões do projeto                             │
│     - Código limpo e testável                               │
│     - Adicionar testes se aplicável                         │
└───────────────────────────┬─────────────────────────────────┘
                            ▼
┌─────────────────────────────────────────────────────────────┐
│  6. VALIDAR                                                 │
│     cargo build --release                                   │
│     cargo test                                              │
│     cargo clippy -- -D warnings                             │
│     cargo fmt --check                                       │
└───────────────────────────┬─────────────────────────────────┘
                            ▼
                    ┌───────┴───────┐
                    │   Passou?     │
                    └───┬───────┬───┘
                       YES     NO
                        │       │
                        │       ▼
                        │   ┌──────────────────────────────────┐
                        │   │  7. ANALISAR E CORRIGIR ERRO     │
                        │   │     - Ler mensagem completa      │
                        │   │     - Identificar causa raiz     │
                        │   │     - Aplicar correção mínima    │
                        │   │     - Re-validar                 │
                        │   └──────────────────────────────────┘
                        │                    │
                        └────────────────────┘
                                 │
                                 ▼
┌─────────────────────────────────────────────────────────────┐
│  8. ATUALIZAR PROGRESSO                                     │
│     - Marcar tarefa como completa                           │
│     - Documentar mudanças                                   │
│     - Atualizar % de progresso                              │
│     - Definir próxima tarefa                                │
└───────────────────────────┬─────────────────────────────────┘
                            ▼
                    ┌───────────────┐
                    │ Mais tarefas? │
                    └───┬───────┬───┘
                       YES     NO
                        │       │
                        │       ▼
                        │   ┌──────────┐
                        │   │  DONE!   │
                        │   │  100% ✅ │
                        │   └──────────┘
                        │
                        └─────► (volta ao passo 2)
```

---

## 📋 CHECKLIST DE VALIDAÇÃO DE FRONTEIRAS

Antes de implementar QUALQUER funcionalidade, responda:

```yaml
Funcionalidade: [descreva aqui]
Projeto Atual: [crypto-???]

Validação:
  - [ ] Está nas "responsibilities" do projeto atual?
  - [ ] NÃO está na lista "❌ PROIBIDO" do meu projeto?
  - [ ] NÃO está nas "responsibilities" de outro projeto?
  - [ ] Se preciso de dados externos: vêm via Kafka?
  - [ ] Se gero dados para outros: publico via Kafka?
  - [ ] NÃO estou duplicando lógica de outro projeto?

Se QUALQUER resposta for NÃO:
  → PARE IMEDIATAMENTE
  → Identifique o projeto correto
  → Use comunicação via Kafka
```

---

## 🎯 PERGUNTAS DE VALIDAÇÃO POR FUNCIONALIDADE

### Antes de Conectar ao WebSocket de Mercado:
```
❓ Estou no crypto-listener?
   SE NÃO → PARE! Ingestão de dados só no crypto-listener
```

### Antes de Calcular Indicadores (RSI, MACD, EMA):
```
❓ Estou no crypto-signals?
   SE NÃO → PARE! Análise técnica só no crypto-signals
```

### Antes de Executar Ordem na Exchange:
```
❓ Estou no crypto-trader?
   SE NÃO → PARE! Execução só no crypto-trader
```

### Antes de Enviar Telegram/Discord/Email:
```
❓ Estou no crypto-notifications?
   SE NÃO → Publique evento em notifications.send via Kafka
```

### Antes de Gerenciar Posições Globais:
```
❓ Estou no crypto-management?
   SE SIM → OK, posições globais são sua responsabilidade
   SE NÃO e é posição GLOBAL → PARE! Vai para crypto-management
   SE NÃO e é stop de UMA ordem → OK, pode ser crypto-trader
```

### Antes de Receber Webhook HTTP:
```
❓ Estou no crypto-webhook?
   SE NÃO → PARE! Webhooks só no crypto-webhook
```

---

## 📊 ESTRUTURA DO ARQUIVO DE PROGRESSO

O arquivo `docs/IMPLEMENTATION_PROGRESS.md` deve seguir esta estrutura:

```markdown
# IMPLEMENTATION PROGRESS - [PROJETO]

**Última Atualização:** [DATA]
**Status Geral:** 🟡 Em Desenvolvimento
**Progresso Total:** X%

---

## 📊 VISÃO GERAL

| Fase | Nome | Status | Progresso | Última Atualização |
|------|------|--------|-----------|---------------------|
| 0 | Análise e Planejamento | ✅ Completo | 100% | [DATA] |
| 1 | Domain Layer | 🟡 Em Progresso | 60% | [DATA] |
| 2 | Infrastructure | ⏳ Pendente | 0% | - |

---

## 🟡 FASE ATUAL: [NOME] - EM PROGRESSO (X%)

### Checklist
- [x] Tarefa A - Completa
- [x] Tarefa B - Completa
- [ ] Tarefa C ← **PRÓXIMA** (PRIORIDADE)
- [ ] Tarefa D
- [ ] Tarefa E

### Próxima Tarefa (FOCO)
**Tarefa C:** [Descrição detalhada]
- **Objetivo:** [O que deve fazer]
- **Arquivos:** `file1.rs`, `file2.rs`
- **Critérios de Aceite:** [Como saber que está pronto]

---

## 🐛 ERROS CONHECIDOS

### Erro 1: [Descrição]
**Status:** ❌ Bloqueante / ⚠️ Não-bloqueante
**Arquivo:** `path/file:line`
**Mensagem:** [erro completo]
**Próximos Passos:** [como resolver]

---

## 📝 LOG DE SESSÕES

### Sessão [DATA]
- ✅ Concluído: [Tarefa X]
- 🐛 Resolvido: [Erro Y]
- 📊 Progresso: X% → Y%
- ⏳ Próximo: [Tarefa Z]
```

---

## ⚠️ SE O ARQUIVO DE PROGRESSO NÃO EXISTIR

Execute estes passos:

1. **Leia o projectmap.yaml do projeto:**
   ```
   _base/{projeto}/projectmap.yaml
   ```

2. **Extraia as fases do projeto:**
   - Seções: `phases`, `milestones`, `features`, ou `roadmap`
   - Identifique etapas principais de implementação

3. **Crie `docs/IMPLEMENTATION_PROGRESS.md`** com estrutura acima

4. **Comece pela FASE 0: Análise e Planejamento:**
   - [ ] Ler projectmap.yaml completo
   - [ ] Identificar requisitos funcionais
   - [ ] Identificar requisitos não-funcionais
   - [ ] Mapear dependências e tecnologias
   - [ ] Definir estrutura de diretórios

---

## 🛠️ PADRÕES DE IMPLEMENTAÇÃO RUST

### Estrutura de Diretórios Padrão

```
src/
├── main.rs                     # Entry point + bootstrap
├── lib.rs                      # Re-exports públicos
├── domain/                     # CAMADA DE DOMÍNIO (lógica pura)
│   ├── mod.rs
│   ├── entities/               # Entidades com identidade
│   ├── value_objects/          # VOs imutáveis
│   ├── events/                 # Domain events
│   ├── aggregates/             # DDD aggregates
│   └── errors.rs               # DomainError
├── application/                # CAMADA DE APLICAÇÃO
│   ├── mod.rs
│   ├── ports/                  # Traits (interfaces)
│   ├── services/               # Orchestrators
│   └── dtos/                   # DTOs de entrada/saída
├── infrastructure/             # CAMADA DE INFRAESTRUTURA
│   ├── mod.rs
│   ├── config/settings.rs      # Configuração
│   ├── messaging/kafka/        # Kafka producer/consumer
│   ├── repositories/           # Implementações de repositórios
│   ├── startup/                # Logging, banner
│   └── shutdown/               # Graceful shutdown
└── shared/                     # KERNEL COMPARTILHADO
    ├── mod.rs
    ├── errors.rs               # ApplicationError, InfrastructureError
    ├── types.rs                # Type aliases
    └── traits/                 # Traits comuns
```

### Padrão de Main.rs

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. CONFIGURAÇÃO
    dotenvy::dotenv().ok();
    let settings = Settings::from_env()?;

    // 2. LOGGING
    init_logging(&settings.app.log_level);
    
    // 3. BANNER
    print_banner(&settings);

    // 4. COMPONENTES (ordem: interno → externo)
    let db_pool = Arc::new(create_db_pool(&settings).await?);
    let kafka_producer = Arc::new(KafkaProducer::new(&settings)?);
    let orchestrator = Arc::new(Orchestrator::new(db_pool, kafka_producer));
    let handler = Arc::new(MessageHandler::new(orchestrator));
    let consumer = KafkaConsumer::new(&settings)?;

    info!("✅ Componentes inicializados");

    // 5. PROCESSAMENTO
    let handler_clone = handler.clone();
    let consume_task = tokio::spawn(async move {
        consumer.consume(|msg| handler_clone.handle(msg)).await
    });

    // 6. GRACEFUL SHUTDOWN
    tokio::signal::ctrl_c().await?;
    info!("Shutdown signal received");
    consume_task.abort();
    info!("👋 Shutdown completo");
    
    Ok(())
}
```

### Padrão de Erros

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("validação: {0}")] Validation(String),
    #[error("não encontrado: {0}")] NotFound(String),
}

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("domínio: {0}")] Domain(#[from] DomainError),
    #[error("kafka: {0}")] Kafka(String),
    #[error("redis: {0}")] Redis(String),
}

pub type DomainResult<T> = Result<T, DomainError>;
pub type AppResult<T> = Result<T, ApplicationError>;
```

---

## 🗺️ MAPA DE TÓPICOS KAFKA

### Produtores por Projeto

| Projeto | Produz para |
|---------|-------------|
| crypto-listener | `crypto-listener.prices` |
| crypto-webhook | `signals.buy`, `signals.sell`, `notifications.send` |
| crypto-signals | `signals.buy`, `signals.sell`, `notifications.send` |
| crypto-trader | `orders.events`, `notifications.send` |
| crypto-management | `management.positions.*`, `management.control.*`, `crypto-listener.subscribe`, `notifications.send` |
| crypto-notifications | `notifications.delivered`, `notifications.failed` |

### Consumidores por Projeto

| Projeto | Consome de |
|---------|------------|
| crypto-listener | `crypto-listener.subscribe`, `crypto-listener.unsubscribe` |
| crypto-signals | `crypto-listener.prices`, `management.strategies.control` |
| crypto-trader | `signals.buy`, `signals.sell`, `management.control.orders` |
| crypto-management | `orders.events`, `signals.*`, `crypto-listener.prices` |
| crypto-notifications | `notifications.send`, `orders.events`, `management.positions.*` |

---

## 💡 TEMPLATE DE RESPOSTA PARA VIOLAÇÃO

Quando detectar violação de fronteiras, responda assim:

```markdown
❌ **VIOLAÇÃO DE FRONTEIRA DETECTADA**

**Funcionalidade solicitada:** [descrever]
**Projeto atual:** [projeto]
**Problema:** Esta funcionalidade pertence a: **[projeto_correto]**

**Razão:**
- [Explicar por que não pertence ao projeto atual]
- [Explicar qual projeto é responsável]

✅ **Solução Correta:**
1. Implementar [funcionalidade] no **[projeto_correto]**
2. [Projeto_correto] publica evento em `[topico_kafka]`
3. [Projeto_atual] consome evento (se necessário)

**Documentação:**
- Consulte: `_base/{projeto_correto}/BOUNDARIES.md`
- Consulte: `_base/WHICH_PROJECT_DOES_WHAT.md`
```

---

## ✅ CHECKLIST FINAL ANTES DE GERAR CÓDIGO

- [ ] Li `docs/IMPLEMENTATION_PROGRESS.md`
- [ ] Li `_base/{projeto}/BOUNDARIES.md`
- [ ] Funcionalidade está nas "responsibilities" do projeto
- [ ] Funcionalidade NÃO está na lista "❌ PROIBIDO"
- [ ] NÃO estou duplicando lógica de outro projeto
- [ ] Comunicação inter-projetos é via Kafka
- [ ] Conheço apenas SCHEMAS Kafka, não implementação interna de outros
- [ ] Segui padrões de `_base/IMPLEMENTATION_PATTERNS.md`

**Se qualquer item não foi confirmado: PARE e revise!**

---

## 📚 ARQUIVOS DE REFERÊNCIA

```
_base/
├── LLM_MASTER_PROMPT.md         # ← ESTE ARQUIVO (prompt universal)
├── BOUNDARIES_GUIDE.md          # Guia geral de fronteiras do ecossistema
├── WHICH_PROJECT_DOES_WHAT.md   # Matriz de responsabilidades
├── IMPLEMENTATION_PATTERNS.md   # Padrões de código Rust
├── IMPROVEMENTS.md              # Recomendações de melhoria
└── {projeto}/
    ├── BOUNDARIES.md            # Fronteiras específicas do projeto
    └── projectmap.yaml          # Especificação técnica do projeto
```

---

## 🎓 REGRA DE OURO

```
╔═══════════════════════════════════════════════════════════════════╗
║                                                                   ║
║   CADA PROJETO É UMA ILHA.                                       ║
║   AS ILHAS SE COMUNICAM APENAS POR KAFKA (PONTES).               ║
║   NUNCA NADE ENTRE ILHAS (HTTP direto, shared DB, imports).      ║
║                                                                   ║
║   QUANDO EM DÚVIDA: LEIA BOUNDARIES.md DO PROJETO.               ║
║                                                                   ║
╚═══════════════════════════════════════════════════════════════════╝
```

---

**Versão:** 2.0  
**Data:** Dezembro 2025  
**Autor:** Documentação do Ecossistema Crypto Trading
