# 🚨 FRONTEIRAS ESTRITAS - CRYPTO-SIGNALS

## ⚠️ Este projeto É:
- ✅ Um **ANALISADOR** de mercado
- ✅ Um **GERADOR** de sinais de trading
- ✅ Um **CALCULADOR** de indicadores técnicos
- ✅ Um **EXECUTOR** de estratégias de análise

## ❌ Este projeto NÃO É:

### 1. NÃO é um Executor de Ordens
```yaml
❌ PROIBIDO:
  - Executar ordens na exchange (Binance, Coinbase, etc.)
  - Conectar com APIs de exchange para criar ordens
  - Gerenciar saldo de carteira
  - Cancelar ordens
  - Modificar ordens
  - Monitorar status de ordens na exchange
  
✅ SOLUÇÃO:
  - Gere o sinal com TODOS os dados necessários
  - Publique em signals.buy ou signals.sell
  - crypto-trader consome e executa a ordem
```

### 2. NÃO é um Sistema de Notificações
```yaml
❌ PROIBIDO:
  - Enviar alertas para Telegram
  - Enviar emails
  - Enviar mensagens para Discord/Slack
  - Formatar mensagens para usuários
  - Implementar clients de notificação
  
✅ SOLUÇÃO:
  - Publique sinal em signals.buy/sell
  - crypto-notifications consome e notifica automaticamente
  - Inclua metadata suficiente no sinal
```

### 3. NÃO é um Gerenciador de Posições
```yaml
❌ PROIBIDO:
  - Rastrear posições abertas
  - Calcular P&L de posições
  - Detectar trades manuais
  - Gerenciar portfolio
  - Calcular exposição total
  - Controlar drawdown
  
✅ SOLUÇÃO:
  - crypto-management gerencia posições
  - crypto-signals APENAS gera sinais
  - Não se preocupe com posições atuais
```

### 4. NÃO é um Receptor de Webhooks
```yaml
❌ PROIBIDO:
  - Expor endpoints HTTP
  - Receber webhooks do TradingView
  - Validar assinaturas de webhooks
  - Normalizar payloads externos
  
✅ SOLUÇÃO:
  - crypto-webhook recebe webhooks
  - crypto-webhook normaliza e publica em sinais
  - Você pode consumir esses sinais SE necessário
  - Mas NÃO implemente recepção HTTP
```

### 5. NÃO é um Gerenciador de Stops
```yaml
❌ PROIBIDO:
  - Monitorar stop loss em tempo real
  - Disparar stop loss
  - Gerenciar trailing stops
  - Cancelar ordens quando stop é atingido
  
✅ SOLUÇÃO:
  - SUGIRA stop loss no sinal (campo stop_loss)
  - crypto-trader implementa e monitora o stop
  - Você apenas recomenda, não executa
```

### 6. NÃO é um Controlador de Sistema
```yaml
❌ PROIBIDO:
  - Controlar modo de operação (PAPER/LIVE)
  - Enable/disable estratégias globalmente
  - Gerenciar configurações de risco
  - Coordenar outros serviços
  
✅ SOLUÇÃO:
  - crypto-management controla o sistema
  - Você CONSOME management.control.strategy
  - Respeite quando uma estratégia é desabilitada
```

---

## 📋 Checklist Antes de Implementar

Pergunte-se:

### ❓ Estou executando ordens em exchange?
- Se SIM → **PARE!** Isso é crypto-trader
- **Solução:** Publique sinal, deixe crypto-trader executar

### ❓ Estou enviando notificações diretas?
- Se SIM → **PARE!** Isso é crypto-notifications
- **Solução:** Publique evento, crypto-notifications envia

### ❓ Estou gerenciando posições ou calculando P&L?
- Se SIM → **PARE!** Isso é crypto-management
- **OK:** Calcular indicadores técnicos baseados em preços históricos

### ❓ Estou recebendo webhooks HTTP?
- Se SIM → **PARE!** Isso é crypto-webhook

### ❓ Estou disparando stop loss?
- Se SIM → **PARE!** Isso é crypto-trader
- **OK:** SUGERIR stop loss no campo do sinal

---

## ✅ O QUE VOCÊ PODE/DEVE FAZER

### 1. Calcular Indicadores Técnicos
```rust
// ✅ CORRETO
async fn calculate_rsi(candles: &[Candle], period: usize) -> f64 {
    // Implementar cálculo de RSI
    // Retornar valor
}

async fn calculate_macd(candles: &[Candle]) -> MACD {
    // Implementar cálculo de MACD
    // Retornar valores
}
```

### 2. Construir e Gerenciar Candles
```rust
// ✅ CORRETO
async fn build_candle(ticks: Vec<Tick>, timeframe: Timeframe) -> Candle {
    // Agregar ticks em OHLCV
    // Armazenar candles históricos
}
```

### 3. Executar Estratégias de Análise
```rust
// ✅ CORRETO
async fn execute_strategy(candles: &[Candle], strategy: &Strategy) -> Option<Signal> {
    let rsi = calculate_rsi(candles, 14);
    let macd = calculate_macd(candles);
    
    if rsi < 30 && macd.histogram > 0 {
        return Some(Signal {
            symbol: "BTCUSDT",
            side: Buy,
            confidence: 0.85,
            target_price: current_price,
            stop_loss: Some(calculate_stop_loss()),
            take_profit: Some(calculate_take_profit()),
            // ...
        });
    }
    None
}
```

### 4. Gerar e Publicar Sinais
```rust
// ✅ CORRETO
async fn publish_signal(signal: Signal) {
    kafka_producer.send("signals.buy", signal).await;
}

// ❌ ERRADO - Executar ordem após gerar sinal
async fn publish_and_execute(signal: Signal) {
    kafka_producer.send("signals.buy", signal).await;
    // ❌ NÃO FAÇA ISSO!
    exchange_client.create_order(signal).await; // ERRADO!
}
```

### 5. Backtesting de Estratégias
```rust
// ✅ CORRETO
async fn backtest_strategy(
    strategy: &Strategy,
    historical_data: &[Candle]
) -> BacktestResults {
    // Replay candles históricos
    // Gerar sinais
    // Calcular métricas (win rate, profit factor)
    // NÃO executar ordens reais
}
```

### 6. Filtrar Sinais
```rust
// ✅ CORRETO
async fn filter_signal(signal: Signal, filters: &[SignalFilter]) -> Option<Signal> {
    // Aplicar filtros (confidence mínima, volume, etc.)
    // Rejeitar se não passar
    // Retornar sinal aprovado
}
```

---

## 🔗 Comunicação com Outros Projetos

### CONSOME (via Kafka):
- ✅ `crypto-listener.prices` (preços em tempo real)
- ✅ `management.control.strategy` (comandos para estratégias)

### PRODUZ (via Kafka):
- ✅ `signals.buy` (consumido por crypto-trader)
- ✅ `signals.sell` (consumido por crypto-trader)

### PROIBIDO:
- ❌ Chamar APIs de exchange para executar ordens
- ❌ Chamar APIs de notificação (Telegram, Discord)
- ❌ Acessar banco de dados do crypto-management
- ❌ Chamar REST APIs de outros microserviços

---

## 🎯 Mantra do crypto-signals

```
EU ANALISO MERCADO.
EU GERO SINAIS.
EU NÃO EXECUTO ORDENS (isso é crypto-trader).
EU NÃO NOTIFICO USUÁRIOS (isso é crypto-notifications).
EU NÃO GERENCIO POSIÇÕES (isso é crypto-management).
EU APENAS ANALISO E SINALIZO.
```

---

## 💡 Padrão de Implementação Correto

### Fluxo Completo:
```
1. crypto-listener → publica preços
2. crypto-signals → consome preços
3. crypto-signals → constrói candles
4. crypto-signals → calcula indicadores
5. crypto-signals → executa estratégia
6. crypto-signals → gera sinal
7. crypto-signals → publica sinal no Kafka
8. crypto-trader → consome sinal
9. crypto-trader → executa ordem
10. crypto-trader → publica order.events
11. crypto-notifications → consome event
12. crypto-notifications → envia notificação
```

### Sua Responsabilidade (passos 2-7):
```rust
// Exemplo completo
async fn full_signal_flow() {
    // 1. Consumir preços
    let price = consume_price_from_kafka().await;
    
    // 2. Atualizar candle
    let candle = update_candle(price).await;
    
    // 3. Calcular indicadores
    let rsi = calculate_rsi(&candles).await;
    let macd = calculate_macd(&candles).await;
    
    // 4. Executar estratégia
    if let Some(signal) = strategy.evaluate(rsi, macd).await {
        // 5. Filtrar sinal
        if filter.passes(&signal) {
            // 6. Publicar no Kafka
            kafka.send("signals.buy", signal).await;
            // ✅ PARE AQUI! Seu trabalho acabou.
            // crypto-trader executa a ordem
            // crypto-notifications envia alertas
        }
    }
}
```

---

**Se você está implementando execução de ordens ou envio de notificações, PARE!**
# 🚨 FRONTEIRAS ESTRITAS - CRYPTO-TRADER

## ⚠️ Este projeto É:
- ✅ Um **EXECUTOR** de ordens
- ✅ Um **GERENCIADOR** de stops locais (de uma ordem específica)
- ✅ Um **CONSUMIDOR** de sinais via Kafka
- ✅ Um **PUBLICADOR** de eventos de ordens via Kafka

## ❌ Este projeto NÃO É:

### 1. NÃO é um Analisador de Mercado
```yaml
❌ PROIBIDO:
  - Calcular RSI, MACD, Bollinger Bands, ou qualquer indicador técnico
  - Analisar tendências de mercado
  - Identificar padrões de candlestick
  - Calcular médias móveis
  - Executar estratégias de análise técnica
  
✅ SOLUÇÃO:
  - crypto-signals faz análise técnica
  - crypto-trader APENAS CONSOME sinais já gerados
  - Tópico Kafka: signals.buy / signals.sell
```

### 2. NÃO é um Gerador de Sinais
```yaml
❌ PROIBIDO:
  - Gerar sinais de BUY/SELL
  - Decidir quando comprar ou vender
  - Implementar estratégias de trading
  - Avaliar condições de mercado
  
✅ SOLUÇÃO:
  - crypto-signals gera sinais
  - crypto-webhook recebe sinais externos
  - crypto-trader APENAS EXECUTA sinais recebidos
```

### 3. NÃO é um Sistema de Notificações
```yaml
❌ PROIBIDO:
  - Enviar mensagens para Telegram
  - Enviar emails
  - Enviar mensagens para Discord
  - Formatar mensagens de notificação
  - Implementar clients de Telegram/Discord/Email/Slack
  - Gerenciar templates de mensagens
  
✅ SOLUÇÃO:
  - Publique evento em orders.events
  - crypto-notifications consome e envia notificações
  - Inclua TODOS os dados necessários no evento
```

### 4. NÃO é um Gerenciador de Posições Globais
```yaml
❌ PROIBIDO:
  - Calcular P&L de portfolio completo
  - Gerenciar múltiplas posições
  - Detectar trades manuais (auto-discovery)
  - Calcular exposição total
  - Gerenciar drawdown de portfolio
  - Reconciliar posições com exchange
  
✅ SOLUÇÃO:
  - crypto-management gerencia posições globais
  - crypto-trader APENAS:
    - Gerencia stops de UMA ordem específica
    - Publica evento quando ordem é executada
    - crypto-management consolida tudo
```

### 5. NÃO é um Receptor de Webhooks
```yaml
❌ PROIBIDO:
  - Expor endpoints HTTP para TradingView
  - Receber webhooks de alertas externos
  - Validar assinaturas de webhooks
  - Normalizar payloads de fontes externas
  - Implementar servidor HTTP para webhooks
  
✅ SOLUÇÃO:
  - crypto-webhook recebe webhooks
  - crypto-webhook normaliza e publica em signals.buy/sell
  - crypto-trader consome sinais já normalizados
```

### 6. NÃO é um Controlador de Estratégias
```yaml
❌ PROIBIDO:
  - Enable/disable estratégias
  - Configurar parâmetros de estratégias
  - Decidir quais estratégias usar
  - Gerenciar modos de operação (PAPER/LIVE)
  
✅ SOLUÇÃO:
  - crypto-management controla estratégias
  - Publica em management.control.mode
  - crypto-trader APENAS RESPEITA o modo recebido
```

---

## 📋 Checklist Antes de Implementar

Pergunte-se:

### ❓ Estou implementando análise técnica?
- Se SIM → **PARE!** Isso é crypto-signals

### ❓ Estou gerando sinais de trading?
- Se SIM → **PARE!** Isso é crypto-signals ou crypto-webhook

### ❓ Estou enviando notificações (Telegram/Email/Discord)?
- Se SIM → **PARE!** Isso é crypto-notifications
- **Solução:** Publique evento no Kafka

### ❓ Estou gerenciando posições de múltiplas ordens?
- Se SIM → **PARE!** Isso é crypto-management
- **OK:** Gerenciar stop de UMA ordem específica

### ❓ Estou recebendo webhooks HTTP?
- Se SIM → **PARE!** Isso é crypto-webhook

### ❓ Estou controlando estratégias ou modo de operação?
- Se SIM → **PARE!** Isso é crypto-management
- **OK:** CONSUMIR e RESPEITAR configurações recebidas

---

## ✅ O QUE VOCÊ PODE/DEVE FAZER

### 1. Consumir Sinais
```rust
// ✅ CORRETO
async fn consume_signal(signal: Signal) {
    // Validar estrutura do sinal
    // Transformar em ordem de exchange
    // Executar ordem
}
```

### 2. Executar Ordens
```rust
// ✅ CORRETO
async fn execute_order(order: Order) {
    // Conectar com Binance API
    // Criar ordem na exchange
    // Monitorar execução
}
```

### 3. Gerenciar Stops Locais
```rust
// ✅ CORRETO - Stop de UMA ordem específica
async fn manage_stop_loss(order_id: OrderId, stop_price: Price) {
    // Monitorar preço
    // Disparar stop quando atingido
    // Cancelar ordem relacionada
}

// ❌ ERRADO - Portfolio inteiro
async fn manage_portfolio_stops() {
    // NÃO FAÇA ISSO!
    // Isso é crypto-management
}
```

### 4. Publicar Eventos
```rust
// ✅ CORRETO
async fn publish_order_filled(order: Order) {
    let event = OrderFilledEvent {
        order_id: order.id,
        symbol: order.symbol,
        quantity: order.filled_quantity,
        price: order.average_price,
        // ... todos os dados necessários
    };
    kafka_producer.send("orders.events", event).await;
}
```

### 5. Validar Risco Localmente
```rust
// ✅ CORRETO - Aplicar limites recebidos
async fn validate_risk(signal: Signal, limits: RiskLimits) {
    if signal.quantity > limits.max_position_size {
        return Err(RiskViolation);
    }
}

// ❌ ERRADO - Calcular limites globais
async fn calculate_portfolio_risk() {
    // NÃO FAÇA ISSO!
    // Isso é crypto-management
}
```

---

## 🔗 Comunicação com Outros Projetos

### CONSOME (via Kafka):
- ✅ `signals.buy` (de crypto-signals, crypto-webhook)
- ✅ `signals.sell` (de crypto-signals, crypto-webhook)
- ✅ `management.control.risk` (de crypto-management)
- ✅ `management.control.mode` (de crypto-management)

### PRODUZ (via Kafka):
- ✅ `orders.events` (consumido por crypto-management, crypto-notifications)

### PROIBIDO:
- ❌ Chamar APIs REST de outros projetos
- ❌ Acessar banco de dados de outros projetos
- ❌ Importar código de outros projetos
- ❌ Conectar diretamente com Redis de outros projetos

---

## 🎯 Mantra do crypto-trader

```
EU EXECUTO ORDENS.
EU NÃO DECIDO QUANDO EXECUTAR (isso é crypto-signals).
EU NÃO NOTIFICO DIRETAMENTE (isso é crypto-notifications).
EU NÃO GERENCIO PORTFOLIO (isso é crypto-management).
EU APENAS EXECUTO E PUBLICO EVENTOS.
```

---

**Se você está implementando algo que não está neste documento, PARE e pergunte!**

