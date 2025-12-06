# 🎧 BOUNDARIES: crypto-listener

## Mantra do Projeto

> **"EU ESCUTO O MERCADO. EU TRANSMITO DADOS. EU NÃO INTERPRETO."**

---

## 📋 Identidade do Projeto

| Aspecto | Descrição |
|---------|-----------|
| **Nome** | crypto-listener |
| **Função** | Ingestão de dados de mercado em tempo real |
| **Entrada** | Binance WebSocket (klines, trades, ticker, depth) |
| **Saída** | Kafka topic `crypto-listener.prices` |
| **Persistência** | TimescaleDB (dados históricos) |

---

## ✅ RESPONSABILIDADES (O que EU faço)

### 1. Conexão WebSocket
- Conectar ao Binance WebSocket API
- Gerenciar múltiplas streams (klines, trades, ticker)
- Implementar reconexão automática com backoff exponencial
- Manter heartbeat e ping/pong

### 2. Processamento de Dados
- Receber trades em tempo real
- Construir candles a partir de trades agregados
- Normalizar dados para formato padrão interno
- Validar integridade dos dados recebidos

### 3. Publicação no Kafka
- Publicar preços em `crypto-listener.prices`
- Garantir entrega confiável (at-least-once)
- Incluir metadata (timestamp, source, symbol)

### 4. Gestão de Subscriptions
- Consumir comandos de `crypto-listener.subscribe`
- Consumir comandos de `crypto-listener.unsubscribe`
- Adicionar/remover streams dinamicamente
- Reportar status de subscriptions

### 5. Persistência de Dados
- Armazenar dados históricos no TimescaleDB
- Manter candles agregados (1m, 5m, 15m, 1h, 4h, 1d)
- Compressão automática de dados antigos

---

## ❌ PROIBIÇÕES (O que EU NÃO faço)

### ❌ Análise Técnica
```yaml
❌ PROIBIDO: Calcular indicadores técnicos (RSI, MACD, EMA, etc.)
   → Responsabilidade: crypto-signals
   
❌ PROIBIDO: Detectar padrões de preço
   → Responsabilidade: crypto-signals
   
❌ PROIBIDO: Calcular níveis de suporte/resistência
   → Responsabilidade: crypto-signals
```

**Por quê?** Eu apenas transmito dados brutos. A interpretação é trabalho do crypto-signals.

### ❌ Geração de Sinais
```yaml
❌ PROIBIDO: Gerar sinais de BUY/SELL
   → Responsabilidade: crypto-signals
   
❌ PROIBIDO: Decidir quando comprar ou vender
   → Responsabilidade: crypto-signals
   
❌ PROIBIDO: Implementar estratégias de trading
   → Responsabilidade: crypto-signals
```

**Por quê?** Eu não tomo decisões. Eu apenas escuto e repasso.

### ❌ Execução de Ordens
```yaml
❌ PROIBIDO: Criar ordens na exchange
   → Responsabilidade: crypto-trader
   
❌ PROIBIDO: Cancelar ordens
   → Responsabilidade: crypto-trader
   
❌ PROIBIDO: Gerenciar stop loss ou take profit
   → Responsabilidade: crypto-trader
```

**Por quê?** Minha função é apenas capturar dados, não executar trades.

### ❌ Gerenciamento de Posições
```yaml
❌ PROIBIDO: Rastrear posições abertas
   → Responsabilidade: crypto-management
   
❌ PROIBIDO: Calcular P&L
   → Responsabilidade: crypto-management
   
❌ PROIBIDO: Gerenciar portfolio
   → Responsabilidade: crypto-management
   
❌ PROIBIDO: Aplicar regras de risco
   → Responsabilidade: crypto-management
```

**Por quê?** Posições são responsabilidade do cérebro central (management).

### ❌ Recepção de Webhooks
```yaml
❌ PROIBIDO: Expor endpoints HTTP POST
   → Responsabilidade: crypto-webhook
   
❌ PROIBIDO: Receber sinais do TradingView
   → Responsabilidade: crypto-webhook
   
❌ PROIBIDO: Validar tokens de webhooks externos
   → Responsabilidade: crypto-webhook
```

**Por quê?** Eu escuto WebSocket de mercado, não HTTP de sistemas externos.

### ❌ Envio de Notificações
```yaml
❌ PROIBIDO: Enviar mensagens via Telegram
   → Responsabilidade: crypto-notifications
   
❌ PROIBIDO: Enviar mensagens via Discord
   → Responsabilidade: crypto-notifications
   
❌ PROIBIDO: Enviar emails
   → Responsabilidade: crypto-notifications
   
❌ PROIBIDO: Formatar mensagens para usuários
   → Responsabilidade: crypto-notifications
```

**Solução correta:**
```rust
// ✅ Se preciso notificar algo
kafka.send("crypto.notifications.send", NotificationRequest {
    channel: "telegram",
    message: "Listener conectado",
    priority: "low",
}).await;
```

---

## 🔌 Interfaces Kafka

### Tópicos que PRODUZO

| Tópico | Schema | Descrição |
|--------|--------|-----------|
| `crypto-listener.prices` | `crypto_listener_price_update.avsc` | Preços e candles em tempo real |

**Exemplo de evento produzido:**
```json
{
  "symbol": "BTCUSDT",
  "timestamp": 1705782000000,
  "open": 42150.50,
  "high": 42200.00,
  "low": 42100.00,
  "close": 42180.25,
  "volume": 125.5,
  "interval": "1m",
  "is_closed": true,
  "source": "binance_websocket"
}
```

### Tópicos que CONSUMO

| Tópico | Schema | Descrição |
|--------|--------|-----------|
| `crypto-listener.subscribe` | `crypto_listener_subscribe_command.avsc` | Comandos para adicionar símbolos |
| `crypto-listener.unsubscribe` | `crypto_listener_unsubscribe_command.avsc` | Comandos para remover símbolos |

**Exemplo de comando consumido:**
```json
{
  "command_id": "uuid-123",
  "symbol": "ETHUSDT",
  "intervals": ["1m", "5m", "1h"],
  "requested_by": "crypto-management",
  "timestamp": 1705782000000
}
```

---

## 🏗️ Estrutura de Diretórios

```
crypto-listener/
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── domain/
│   │   ├── mod.rs
│   │   ├── entities/
│   │   │   ├── mod.rs
│   │   │   ├── candle.rs           # Entidade Candle
│   │   │   ├── trade.rs            # Entidade Trade
│   │   │   └── subscription.rs     # Entidade Subscription
│   │   ├── value_objects/
│   │   │   ├── mod.rs
│   │   │   ├── symbol.rs           # VO Symbol
│   │   │   ├── interval.rs         # VO Interval (1m, 5m, etc.)
│   │   │   └── price.rs            # VO Price
│   │   └── events/
│   │       ├── mod.rs
│   │       └── price_updated.rs    # Evento de preço
│   ├── application/
│   │   ├── mod.rs
│   │   ├── ports/
│   │   │   ├── mod.rs
│   │   │   ├── market_data_source.rs    # Trait para fonte de dados
│   │   │   ├── price_publisher.rs       # Trait para publicação
│   │   │   └── candle_repository.rs     # Trait para persistência
│   │   ├── services/
│   │   │   ├── mod.rs
│   │   │   ├── candle_builder.rs        # Construção de candles
│   │   │   └── subscription_manager.rs  # Gerenciamento de subs
│   │   └── dtos/
│   │       ├── mod.rs
│   │       ├── price_update_dto.rs
│   │       └── subscription_command_dto.rs
│   ├── infrastructure/
│   │   ├── mod.rs
│   │   ├── config/
│   │   │   ├── mod.rs
│   │   │   └── settings.rs
│   │   ├── websocket/
│   │   │   ├── mod.rs
│   │   │   ├── binance_client.rs        # Implementação Binance
│   │   │   └── reconnection_handler.rs  # Lógica de reconexão
│   │   ├── messaging/
│   │   │   ├── mod.rs
│   │   │   └── kafka/
│   │   │       ├── mod.rs
│   │   │       ├── producer.rs          # Produz preços
│   │   │       └── consumer.rs          # Consome comandos
│   │   ├── repositories/
│   │   │   ├── mod.rs
│   │   │   └── timescale_candle_repository.rs
│   │   ├── startup/
│   │   │   ├── mod.rs
│   │   │   ├── logging.rs
│   │   │   └── banner.rs
│   │   └── shutdown/
│   │       └── mod.rs
│   └── shared/
│       ├── mod.rs
│       ├── errors.rs
│       └── types.rs
├── Cargo.toml
├── .env.example
└── README.md
```

---

## 🎯 Exemplos de Implementação

### ✅ Implementação CORRETA

```rust
// src/infrastructure/websocket/binance_client.rs
impl BinanceWebSocketClient {
    /// Conecta ao WebSocket e emite eventos
    pub async fn connect(&self, symbols: Vec<Symbol>) -> Result<(), Error> {
        let stream = self.create_stream(&symbols).await?;
        
        while let Some(msg) = stream.next().await {
            match msg {
                Ok(trade) => {
                    // ✅ CORRETO: Apenas processa e publica
                    let candle = self.candle_builder.process_trade(trade)?;
                    self.publisher.publish(candle).await?;
                }
                Err(e) => {
                    // ✅ CORRETO: Reconexão automática
                    self.reconnect().await?;
                }
            }
        }
        Ok(())
    }
}

// src/application/services/subscription_manager.rs
impl SubscriptionManager {
    /// Processa comando de subscribe
    pub async fn handle_subscribe(&self, cmd: SubscribeCommand) -> Result<(), Error> {
        // ✅ CORRETO: Adiciona stream
        self.websocket.add_symbol(&cmd.symbol, &cmd.intervals).await?;
        
        // ✅ CORRETO: Notifica via Kafka (não diretamente)
        self.kafka.send("crypto.notifications.send", NotificationRequest {
            message: format!("Agora monitorando {}", cmd.symbol),
            priority: "info",
        }).await?;
        
        Ok(())
    }
}
```

### ❌ Implementação ERRADA

```rust
// ❌ ERRADO: Calculando indicadores (responsabilidade do crypto-signals)
impl BinanceWebSocketClient {
    pub async fn process_price(&self, candle: Candle) -> Result<(), Error> {
        // ❌ NUNCA FAÇA ISSO!
        let rsi = self.calculate_rsi(&candle);  // ERRADO!
        let ema = self.calculate_ema(&candle);  // ERRADO!
        
        if rsi < 30 {  // ERRADO!
            // ❌ Gerando sinal (responsabilidade do crypto-signals)
            self.kafka.send("crypto.signals.buy", signal).await;  // ERRADO!
        }
        
        // ❌ Enviando notificação direta (responsabilidade do crypto-notifications)
        self.telegram.send("RSI baixo detectado").await;  // ERRADO!
    }
}

// ❌ ERRADO: Gerenciando posições (responsabilidade do crypto-management)
impl SubscriptionManager {
    pub async fn handle_price(&self, price: Price) -> Result<(), Error> {
        // ❌ NUNCA FAÇA ISSO!
        let positions = self.db.get_positions().await;  // ERRADO!
        let pnl = self.calculate_pnl(&positions, &price);  // ERRADO!
    }
}
```

---

## 🔍 Perguntas de Validação

Antes de implementar, pergunte-se:

### 1. Estou apenas escutando e transmitindo?
```
✅ SIM → Continue
❌ NÃO → Revise qual projeto deve fazer isso
```

### 2. Estou calculando indicadores ou analisando padrões?
```
❌ SIM → PARE! Isso é responsabilidade do crypto-signals
✅ NÃO → Continue
```

### 3. Estou gerando sinais de compra/venda?
```
❌ SIM → PARE! Isso é responsabilidade do crypto-signals
✅ NÃO → Continue
```

### 4. Estou chamando API da exchange para executar ordens?
```
❌ SIM → PARE! Isso é responsabilidade do crypto-trader
✅ NÃO → Continue
```

### 5. Estou enviando mensagens diretamente (Telegram, etc.)?
```
❌ SIM → PARE! Use Kafka para crypto.notifications.send
✅ NÃO → Continue
```

---

## 📚 Fluxo Típico

```
┌─────────────────────┐
│   Binance WebSocket │
└─────────┬───────────┘
          │ trades, klines
          ▼
┌─────────────────────┐
│   crypto-listener   │
│   ┌───────────────┐ │
│   │ WebSocket     │ │
│   │ Client        │ │
│   └───────┬───────┘ │
│           │         │
│   ┌───────▼───────┐ │
│   │ Candle        │ │
│   │ Builder       │ │
│   └───────┬───────┘ │
│           │         │
│   ┌───────▼───────┐ │
│   │ Kafka         │ │
│   │ Producer      │ │
│   └───────┬───────┘ │
└───────────┼─────────┘
            │
            ▼
┌───────────────────────────┐
│ crypto-listener.prices    │
│ (Kafka Topic)             │
└───────────────────────────┘
            │
    ┌───────┴───────┐
    ▼               ▼
crypto-signals  crypto-management
(análise)       (tracking)
```

---

## ⚠️ Armadilhas Comuns

### 1. "Vou calcular só uma média móvel aqui..."
**NÃO!** Qualquer cálculo de indicador vai para crypto-signals.

### 2. "Vou verificar se o preço bateu no stop..."
**NÃO!** Gerenciamento de stops é do crypto-trader.

### 3. "Vou enviar um alerta quando conectar..."
**OK**, mas via Kafka:
```rust
// ✅ CORRETO
kafka.send("crypto.notifications.send", NotificationRequest { ... }).await;

// ❌ ERRADO
telegram.send("Conectado!").await;
```

### 4. "Vou salvar a posição quando detectar trade..."
**NÃO!** Posições são do crypto-management. Apenas publique o preço.

---

## 📖 Documentação Relacionada

- `WHICH_PROJECT_DOES_WHAT.md` - Visão geral de todos os projetos
- `BOUNDARIES_GUIDE.md` - Guia geral de fronteiras
- `crypto-listener_projectmap.yaml` - Mapa detalhado do projeto
- `LLM_IMPLEMENTATION_PROMPT.md` - Instruções para LLM

---

**Versão:** 1.0.0  
**Data:** 2025-01-20  
**Mantra:** "EU ESCUTO O MERCADO. EU TRANSMITO DADOS. EU NÃO INTERPRETO."
