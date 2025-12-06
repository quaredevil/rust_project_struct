# 🎯 WHICH PROJECT DOES WHAT?

## Guia Completo de Responsabilidades do Ecossistema Crypto Trading

> **MANTRA ABSOLUTO**: Cada projeto é uma ilha. As ilhas se comunicam APENAS por Kafka (pontes). NUNCA nadar entre ilhas (HTTP direto, shared DB, etc.).

---

## 📊 Visão Geral dos Projetos

| Projeto | Mantra | Função Principal |
|---------|--------|------------------|
| **crypto-listener** | "EU ESCUTO O MERCADO" | Ingere dados de mercado via WebSocket |
| **crypto-webhook** | "EU RECEBO, VALIDO E NORMALIZO" | Gateway HTTP para webhooks externos |
| **crypto-signals** | "EU ANALISO MERCADO. EU GERO SINAIS" | Análise técnica e geração de sinais |
| **crypto-trader** | "EU EXECUTO ORDENS" | Execução de ordens na exchange |
| **crypto-management** | "EU ORQUESTRO E COORDENO" | Cérebro central - posições e risco |
| **crypto-notifications** | "EU NOTIFICO" | Distribuição de alertas multi-canal |

---

## 🔄 Fluxo de Dados Completo

```
                                    ┌─────────────────────┐
                                    │   Binance WebSocket │
                                    └─────────┬───────────┘
                                              │
                                              ▼
┌─────────────────────┐            ┌─────────────────────┐
│   TradingView/      │            │   crypto-listener   │
│   Webhooks          │            │   "EU ESCUTO"       │
└─────────┬───────────┘            └─────────┬───────────┘
          │                                  │
          ▼                                  │
┌─────────────────────┐                      │
│   crypto-webhook    │                      │
│   "EU NORMALIZO"    │                      │
└─────────┬───────────┘                      │
          │                                  │
          │ signals.buy/sell                 │ crypto-listener.prices
          │                                  │
          ▼                                  ▼
┌────────────────────────────────────────────────────────────┐
│                         KAFKA                               │
├────────────────────────────────────────────────────────────┤
│  signals.buy  │  signals.sell  │  orders.events  │  ...    │
└─────────┬────────────┬────────────────┬────────────────────┘
          │            │                │
          ▼            ▼                │
┌─────────────────────┐                 │
│   crypto-signals    │◄────────────────┘ (preços)
│   "EU ANALISO"      │
└─────────┬───────────┘
          │ signals.buy/sell
          ▼
┌─────────────────────┐
│   crypto-trader     │─────────────────┐
│   "EU EXECUTO"      │                 │ orders.events
└─────────┬───────────┘                 │
          │                             ▼
          │ Binance API        ┌─────────────────────┐
          ▼                    │   crypto-management │
    ┌──────────┐               │   "EU ORQUESTRO"    │
    │ Exchange │               └─────────┬───────────┘
    └──────────┘                         │
                                         │ management.positions.*
                                         │ notifications.send
                                         ▼
                               ┌─────────────────────┐
                               │crypto-notifications │
                               │   "EU NOTIFICO"     │
                               └─────────┬───────────┘
                                         │
                              ┌──────────┼──────────┐
                              ▼          ▼          ▼
                         Telegram    Discord     Email
```

---

## 📋 Matriz Detalhada de Responsabilidades

### 1. crypto-listener - "EU ESCUTO O MERCADO"

| Aspecto | Descrição |
|---------|-----------|
| **Função** | Ingestão de dados de mercado em tempo real |
| **Entrada** | Binance WebSocket (klines, trades, ticker) |
| **Saída** | Tópico `crypto.listener.prices` |
| **Persistência** | TimescaleDB (dados históricos) |

#### ✅ RESPONSABILIDADES:
- Conectar ao Binance WebSocket
- Construir candles a partir de trades
- Publicar preços no Kafka
- Gerenciar reconexão automática
- Responder a comandos de subscribe/unsubscribe

#### ❌ NÃO FAZ:
- Calcular indicadores técnicos (→ crypto-signals)
- Gerar sinais de trading (→ crypto-signals)
- Executar ordens (→ crypto-trader)
- Receber webhooks HTTP (→ crypto-webhook)
- Gerenciar posições (→ crypto-management)
- Enviar notificações (→ crypto-notifications)

#### Tópicos Kafka:
| Tipo | Tópico | Descrição |
|------|--------|-----------|
| **PRODUZ** | `crypto.listener.prices` | Preços e candles em tempo real |
| **CONSOME** | `crypto.listener.subscribe` | Comandos de inscrição em símbolos |
| **CONSOME** | `crypto.listener.unsubscribe` | Comandos de cancelamento |

---

### 2. crypto-webhook - "EU RECEBO, VALIDO E NORMALIZO"

| Aspecto | Descrição |
|---------|-----------|
| **Função** | Gateway HTTP para integrações externas |
| **Entrada** | HTTP POST (TradingView, Alertatron, etc.) |
| **Saída** | Tópicos `crypto.webhook.signals.buy`, `crypto.webhook.signals.sell` |
| **Persistência** | PostgreSQL (logs de requisições) |

#### ✅ RESPONSABILIDADES:
- Expor endpoints HTTP para webhooks
- Validar HMAC/tokens de autenticação
- Validar schemas de payload
- Normalizar dados para formato padrão
- Publicar sinais no Kafka
- Gerenciar idempotência

#### ❌ NÃO FAZ:
- Executar ordens (→ crypto-trader)
- Analisar ou melhorar sinais (→ crypto-signals)
- Calcular stop loss/take profit (→ crypto-signals ou crypto-trader)
- Enviar notificações (→ crypto-notifications)
- Gerenciar posições (→ crypto-management)
- Escutar WebSocket de mercado (→ crypto-listener)

#### Tópicos Kafka:
| Tipo | Tópico | Descrição |
|------|--------|-----------|
| **PRODUZ** | `crypto.webhook.signals.buy` | Sinais de compra normalizados |
| **PRODUZ** | `crypto.webhook.signals.sell` | Sinais de venda normalizados |
| **PRODUZ** | `crypto.notifications.send` | Notificações sobre webhooks recebidos |

---

### 3. crypto-signals - "EU ANALISO MERCADO. EU GERO SINAIS"

| Aspecto | Descrição |
|---------|-----------|
| **Função** | Análise técnica e geração de sinais |
| **Entrada** | Tópico `crypto.listener.prices` |
| **Saída** | Tópicos `crypto.signals.buy`, `crypto.signals.sell` |
| **Persistência** | PostgreSQL (histórico de sinais) |

#### ✅ RESPONSABILIDADES:
- Calcular indicadores técnicos (RSI, MACD, Bollinger, etc.)
- Construir e manter candles agregados
- Executar estratégias de análise (EMA Cross, RSI Divergence, etc.)
- Gerar sinais BUY/SELL com stop loss e take profit sugeridos
- Publicar sinais no Kafka
- Gerenciar estado das estratégias

#### ❌ NÃO FAZ:
- Executar ordens na exchange (→ crypto-trader)
- Receber webhooks HTTP (→ crypto-webhook)
- Enviar notificações diretas (→ crypto-notifications)
- Gerenciar posições globais (→ crypto-management)
- Gerenciar stops de ordens ativas (→ crypto-trader)
- Conectar ao WebSocket de mercado (→ crypto-listener)

#### Tópicos Kafka:
| Tipo | Tópico | Descrição |
|------|--------|-----------|
| **CONSOME** | `crypto.listener.prices` | Preços em tempo real |
| **CONSOME** | `crypto.management.control.strategies` | Comandos de controle |
| **PRODUZ** | `crypto.signals.buy` | Sinais de compra gerados |
| **PRODUZ** | `crypto.signals.sell` | Sinais de venda gerados |
| **PRODUZ** | `crypto.notifications.send` | Alertas sobre sinais |

---

### 4. crypto-trader - "EU EXECUTO ORDENS"

| Aspecto | Descrição |
|---------|-----------|
| **Função** | Execução e gerenciamento de ordens |
| **Entrada** | Tópicos `crypto.signals.*`, `crypto.webhook.signals.*` |
| **Saída** | Tópico `crypto.trader.orders` |
| **Persistência** | PostgreSQL (ordens locais) |

#### ✅ RESPONSABILIDADES:
- Consumir sinais via Kafka
- Executar ordens na Binance API
- Gerenciar stops de ordens específicas (trailing, breakeven)
- Implementar retry com backoff exponencial
- Publicar eventos de ordens no Kafka
- Validar saldo e limites antes de executar

#### ❌ NÃO FAZ:
- Calcular indicadores técnicos (→ crypto-signals)
- Gerar sinais de trading (→ crypto-signals)
- Enviar notificações diretas (→ crypto-notifications)
- Gerenciar posições globais/portfolio (→ crypto-management)
- Calcular P&L total (→ crypto-management)
- Receber webhooks HTTP (→ crypto-webhook)
- Decidir risco do portfolio (→ crypto-management)

#### Tópicos Kafka:
| Tipo | Tópico | Descrição |
|------|--------|-----------|
| **CONSOME** | `crypto.signals.buy` | Sinais de compra (internos) |
| **CONSOME** | `crypto.signals.sell` | Sinais de venda (internos) |
| **CONSOME** | `crypto.webhook.signals.buy` | Sinais de compra (externos) |
| **CONSOME** | `crypto.webhook.signals.sell` | Sinais de venda (externos) |
| **CONSOME** | `crypto.management.control.risk` | Comandos de risco |
| **CONSOME** | `crypto.management.control.mode` | Comandos de modo |
| **PRODUZ** | `crypto.trader.orders` | Eventos de ordens (filled, cancelled, etc.) |
| **PRODUZ** | `crypto.notifications.send` | Alertas sobre execuções |

---

### 5. crypto-management - "EU ORQUESTRO E COORDENO"

| Aspecto | Descrição |
|---------|-----------|
| **Função** | Cérebro central - orquestração e controle |
| **Entrada** | Múltiplos tópicos de eventos |
| **Saída** | Comandos de controle e eventos de posição |
| **Persistência** | PostgreSQL (posições, portfolio) |

#### ✅ RESPONSABILIDADES:
- Gerenciar posições globais do portfolio
- Calcular P&L total, drawdown, exposição
- Auto-discovery de trades manuais (User Data Stream)
- Aplicar risk management central (max exposure, max loss)
- Controlar estratégias (enable/disable)
- Controlar modo de operação (PAPER/LIVE)
- Orquestrar subscriptions do crypto-listener
- Emitir alertas de risco

#### ❌ NÃO FAZ:
- Executar ordens diretamente na exchange (→ crypto-trader)
- Calcular indicadores técnicos (→ crypto-signals)
- Gerar sinais de trading (→ crypto-signals)
- Enviar notificações diretas (→ crypto-notifications)
- Receber webhooks HTTP (→ crypto-webhook)
- Conectar ao WebSocket de preços (→ crypto-listener)
- Gerenciar stops de ordens específicas (→ crypto-trader)

#### Tópicos Kafka:
| Tipo | Tópico | Descrição |
|------|--------|-----------|
| **CONSOME** | `crypto.trader.orders` | Eventos de ordens |
| **CONSOME** | `crypto.signals.buy` | Sinais de compra (tracking) |
| **CONSOME** | `crypto.signals.sell` | Sinais de venda (tracking) |
| **CONSOME** | `crypto.webhook.signals.buy` | Sinais externos (tracking) |
| **CONSOME** | `crypto.webhook.signals.sell` | Sinais externos (tracking) |
| **CONSOME** | `crypto.listener.prices` | Preços (cálculo P&L) |
| **PRODUZ** | `crypto.management.positions.opened` | Posição aberta |
| **PRODUZ** | `crypto.management.positions.closed` | Posição fechada |
| **PRODUZ** | `crypto.management.positions.updated` | Posição atualizada |
| **PRODUZ** | `crypto.management.control.strategies` | Comandos para signals |
| **PRODUZ** | `crypto.management.control.risk` | Comandos de risco para trader |
| **PRODUZ** | `crypto.management.control.mode` | Comandos de modo para trader |
| **PRODUZ** | `crypto.listener.subscribe` | Comandos para listener |
| **PRODUZ** | `crypto.listener.unsubscribe` | Comandos para listener |
| **PRODUZ** | `crypto.notifications.send` | Alertas de risco |

---

### 6. crypto-notifications - "EU NOTIFICO"

| Aspecto | Descrição |
|---------|-----------|
| **Função** | Distribuição de notificações multi-canal |
| **Entrada** | Tópico `crypto.notifications.send` + eventos de broadcast |
| **Saída** | Telegram, Discord, Email |
| **Persistência** | PostgreSQL (histórico), Redis (throttling) |

#### ✅ RESPONSABILIDADES:
- Consumir eventos de notificação via Kafka
- Formatar mensagens para cada canal
- Enviar via Telegram, Discord, Email
- Gerenciar rate limiting e throttling
- Agrupar notificações similares
- Registrar entrega/falha

#### ❌ NÃO FAZ:
- Executar ordens (→ crypto-trader)
- Gerar sinais de trading (→ crypto-signals)
- Calcular indicadores (→ crypto-signals)
- Gerenciar posições (→ crypto-management)
- Decidir QUANDO notificar (vem no evento)
- Receber webhooks HTTP (→ crypto-webhook)
- Analisar mercado (→ crypto-signals)

#### Tópicos Kafka:
| Tipo | Tópico | Descrição |
|------|--------|-----------|
| **CONSOME** | `crypto.notifications.send` | Pedidos de notificação |
| **CONSOME** | `crypto.trader.orders` | Eventos de ordens (broadcast) |
| **CONSOME** | `crypto.management.positions.*` | Eventos de posição (broadcast) |
| **PRODUZ** | `crypto.notifications.delivered` | Confirmação de entrega |
| **PRODUZ** | `crypto.notifications.failed` | Falha de entrega |
| **PRODUZ** | `notifications.delivered` | Confirmação de entrega |
| **PRODUZ** | `notifications.failed` | Falha de entrega |

---

## 🚦 Árvore de Decisão Rápida

### Onde Implemento Esta Funcionalidade?

```
┌─ Preciso conectar ao WebSocket de mercado? ─────────────────┐
│  SIM → crypto-listener                                      │
│  NÃO → Continue                                             │
└─────────────────────────────────────────────────────────────┘
           │
           ▼
┌─ Preciso receber HTTP de sistemas externos? ────────────────┐
│  (Webhooks, TradingView)                                    │
│  SIM → crypto-webhook                                       │
│  NÃO → Continue                                             │
└─────────────────────────────────────────────────────────────┘
           │
           ▼
┌─ Envolve análise técnica ou geração de sinais? ─────────────┐
│  (RSI, MACD, indicadores, estratégias)                      │
│  SIM → crypto-signals                                       │
│  NÃO → Continue                                             │
└─────────────────────────────────────────────────────────────┘
           │
           ▼
┌─ Envolve executar ordem na exchange? ───────────────────────┐
│  (create_order, cancel_order, trailing stop de UMA ordem)   │
│  SIM → crypto-trader                                        │
│  NÃO → Continue                                             │
└─────────────────────────────────────────────────────────────┘
           │
           ▼
┌─ Envolve gerenciar posições globais ou controle? ───────────┐
│  (Portfolio, P&L total, risco, auto-discovery)              │
│  SIM → crypto-management                                    │
│  NÃO → Continue                                             │
└─────────────────────────────────────────────────────────────┘
           │
           ▼
┌─ Envolve enviar mensagens para usuários? ───────────────────┐
│  (Telegram, Discord, Email)                                 │
│  SIM → crypto-notifications                                 │
│  NÃO → Reavalie o escopo                                    │
└─────────────────────────────────────────────────────────────┘
```

---

## 📨 Mapa Completo de Tópicos Kafka

> **PADRÃO:** `crypto.{projeto}.{recurso}.{ação}` (sempre usar `.` como separador)  
> **Guia de Migração:** `_base/KAFKA_TOPICS_MIGRATION.md`

### Tópicos de Dados de Mercado (crypto.listener.*)
| Tópico | Tipo | Produtor | Consumidores |
|--------|------|----------|--------------|
| `crypto.listener.prices` | Evento | crypto-listener | crypto-signals, crypto-management |
| `crypto.listener.subscribe` | Comando | crypto-management | crypto-listener |
| `crypto.listener.unsubscribe` | Comando | crypto-management | crypto-listener |

### Tópicos de Sinais Internos (crypto.signals.*)
| Tópico | Tipo | Produtor | Consumidores |
|--------|------|----------|--------------|
| `crypto.signals.buy` | Evento | crypto-signals | crypto-trader, crypto-management |
| `crypto.signals.sell` | Evento | crypto-signals | crypto-trader, crypto-management |

### Tópicos de Sinais Externos (crypto.webhook.*)
| Tópico | Tipo | Produtor | Consumidores |
|--------|------|----------|--------------|
| `crypto.webhook.signals.buy` | Evento | crypto-webhook | crypto-trader, crypto-management |
| `crypto.webhook.signals.sell` | Evento | crypto-webhook | crypto-trader, crypto-management |

### Tópicos de Ordens (crypto.trader.*)
| Tópico | Tipo | Produtor | Consumidores |
|--------|------|----------|--------------|
| `crypto.trader.orders` | Evento | crypto-trader | crypto-management, crypto-notifications |

### Tópicos de Gestão (crypto.management.*)
| Tópico | Tipo | Produtor | Consumidores |
|--------|------|----------|--------------|
| `crypto.management.positions.opened` | Evento | crypto-management | crypto-notifications |
| `crypto.management.positions.closed` | Evento | crypto-management | crypto-notifications |
| `crypto.management.positions.updated` | Evento | crypto-management | crypto-notifications |
| `crypto.management.control.strategies` | Comando | crypto-management | crypto-signals |
| `crypto.management.control.risk` | Comando | crypto-management | crypto-trader |
| `crypto.management.control.mode` | Comando | crypto-management | crypto-trader |

### Tópicos de Notificações (crypto.notifications.*)
| Tópico | Tipo | Produtores | Consumidor |
|--------|------|------------|------------|
| `crypto.notifications.send` | Comando | TODOS | crypto-notifications |
| `crypto.notifications.delivered` | Evento | crypto-notifications | (monitoring) |
| `crypto.notifications.failed` | Evento | crypto-notifications | crypto-management |

---

## ⚡ Casos de Uso Comuns

### Caso 1: Sinal Interno Gera Trade

```
1. crypto-listener → publica preço em crypto.listener.prices
2. crypto-signals  → consome preço, calcula RSI, gera sinal
3. crypto-signals  → publica sinal em crypto.signals.buy
4. crypto-trader   → consome sinal, executa ordem
5. crypto-trader   → publica evento em crypto.trader.orders
6. crypto-management → atualiza posição
7. crypto-notifications → envia alerta
```

### Caso 2: Webhook Externo Gera Trade

```
1. TradingView     → envia POST para crypto-webhook
2. crypto-webhook  → valida, normaliza
3. crypto-webhook  → publica sinal em crypto.webhook.signals.buy
4. crypto-trader   → consome sinal, executa ordem
5. crypto-trader   → publica evento em crypto.trader.orders
6. crypto-management → atualiza posição
7. crypto-notifications → envia alerta
```

### Caso 3: Trade Manual Detectado

```
1. Usuário         → executa ordem manualmente na Binance
2. crypto-management → detecta via User Data Stream
3. crypto-management → cria posição
4. crypto-management → publica em crypto.management.positions.opened
5. crypto-management → publica crypto.listener.subscribe
6. crypto-listener → começa a monitorar símbolo
7. crypto-notifications → envia alerta
```

### Caso 4: Risco Atinge Limite

```
1. crypto-management → detecta drawdown > max permitido
2. crypto-management → publica crypto.management.control.risk (HALT_TRADING)
3. crypto-trader     → para de processar sinais
4. crypto-management → publica crypto.management.control.strategies (DISABLE)
5. crypto-signals    → desabilita geração de sinais
6. crypto-notifications → envia alerta urgente
```

---

## ⚠️ Anti-Padrões a Evitar

### ❌ Implementar Lógica no Projeto Errado

```rust
// NO crypto-trader (ERRADO!)
async fn process_signal(signal: Signal) {
    let rsi = calculate_rsi();  // ❌ Isso é crypto-signals!
    if rsi < 30 {
        execute_order().await;
        telegram.send("OK").await;  // ❌ Isso é crypto-notifications!
    }
}
```

### ❌ Chamar Outro Projeto Diretamente

```rust
// NO crypto-management (ERRADO!)
async fn close_position() {
    trader_api.execute_order().await;  // ❌ NUNCA HTTP entre projetos!
    
    // ✅ CORRETO:
    kafka.send("crypto.management.control.risk", RiskControl {
        action: HaltTrading,
        reason: "Max drawdown exceeded",
    }).await;
}
```

### ❌ Acessar Banco de Outro Projeto

```rust
// NO crypto-notifications (ERRADO!)
async fn enrich_notification() {
    let position = db.query("SELECT * FROM management.positions").await;  // ❌
    
    // ✅ CORRETO: Dados vêm no evento Kafka
}
```

---

## 📝 Checklist de Validação

Antes de implementar qualquer funcionalidade:

- [ ] Li o BOUNDARIES do projeto atual
- [ ] A funcionalidade está na lista de responsabilidades
- [ ] A funcionalidade NÃO está na lista de proibições
- [ ] NÃO estou duplicando lógica de outro projeto
- [ ] Comunicação inter-projetos é APENAS via Kafka
- [ ] Conheço apenas SCHEMAS Kafka, não implementação interna

---

## 🔗 Documentação Relacionada

### Documentação por Projeto (em subpastas)
- `crypto-listener/BOUNDARIES.md` - Fronteiras do listener
- `crypto-listener/projectmap.yaml` - Mapa do listener
- `crypto-webhook/BOUNDARIES.md` - Fronteiras do webhook
- `crypto-webhook/projectmap.yaml` - Mapa do webhook
- `crypto-signals/BOUNDARIES.md` - Fronteiras do signals
- `crypto-signals/projectmap.yaml` - Mapa do signals
- `crypto-trader/BOUNDARIES.md` - Fronteiras do trader
- `crypto-trader/projectmap.yaml` - Mapa do trader
- `crypto-management/BOUNDARIES.md` - Fronteiras do management
- `crypto-management/projectmap.yaml` - Mapa do management
- `crypto-notifications/BOUNDARIES.md` - Fronteiras do notifications
- `crypto-notifications/projectmap.yaml` - Mapa do notifications

### Documentação Geral (na raiz _base/)
- `BOUNDARIES_GUIDE.md` - Guia geral de fronteiras
- `LLM_IMPLEMENTATION_PROMPT.md` - Instruções para LLM
- `IMPLEMENTATION_PATTERNS.md` - Padrões de código
- `WHICH_PROJECT_DOES_WHAT.md` - Este arquivo

---

**Versão:** 1.0.0  
**Data:** 2025-01-20  
**Autor:** Documentação do Ecossistema Crypto Trading
