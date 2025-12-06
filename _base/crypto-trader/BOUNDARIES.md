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
  - Tópico Kafka: crypto.signals.buy / crypto.signals.sell
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
  - Publique evento em crypto.trader.orders
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
  - crypto-webhook normaliza e publica em crypto.signals.buy/sell
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
  - Publica em crypto.management.control.mode
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
    kafka_producer.send("crypto.trader.orders", event).await;
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
- ✅ `crypto.signals.buy` (de crypto-signals, crypto-webhook)
- ✅ `crypto.signals.sell` (de crypto-signals, crypto-webhook)
- ✅ `crypto.management.control.risk` (de crypto-management)
- ✅ `crypto.management.control.mode` (de crypto-management)

### PRODUZ (via Kafka):
- ✅ `crypto.trader.orders` (consumido por crypto-management, crypto-notifications)

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

