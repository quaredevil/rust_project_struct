# 🔄 MIGRATION PROMPT - crypto-management

> **Data:** Dezembro 2025  
> **Tipo:** Migração de Tópicos Kafka  
> **Status:** 📋 PENDENTE

---

## ⚠️ AVISO IMPORTANTE

**HOUVE MUDANÇA NOS NOMES DOS TÓPICOS KAFKA!**

O ecossistema padronizou todos os tópicos para usar `.` (ponto) como separador, seguindo o padrão:
```
crypto.{projeto}.{recurso}.{ação}
```

**IMPORTANTE:** crypto-management é o projeto que mais interage com Kafka - tem MUITOS tópicos!

---

## 🔀 MUDANÇAS PARA ESTE PROJETO

### Tópicos PRODUZIDOS (você envia)

| Antigo | Novo |
|--------|------|
| `management.positions.opened` | `crypto.management.positions.opened` |
| `management.positions.closed` | `crypto.management.positions.closed` |
| `management.positions.updated` | `crypto.management.positions.updated` |
| `management.control.strategy` | `crypto.management.control.strategies` |
| `management.control.risk` | `crypto.management.control.risk` |
| `management.control.mode` | `crypto.management.control.mode` |
| `crypto-listener.subscribe` / `crypto-listener-subscription-commands` | `crypto.listener.subscribe` |
| `crypto-listener.unsubscribe` / `crypto-listener-unsubscription-commands` | `crypto.listener.unsubscribe` |
| `notifications.send` | `crypto.notifications.send` |

### Tópicos CONSUMIDOS (você recebe)

| Antigo | Novo | Origem |
|--------|------|--------|
| `orders.events` | `crypto.trader.orders` | crypto-trader |
| `signals.buy` | `crypto.signals.buy` | crypto-signals |
| `signals.sell` | `crypto.signals.sell` | crypto-signals |
| (novo) | `crypto.webhook.signals.buy` | crypto-webhook |
| (novo) | `crypto.webhook.signals.sell` | crypto-webhook |
| `crypto-listener.prices` / `crypto-listener-prices` | `crypto.listener.prices` | crypto-listener |

---

## 📋 CHECKLIST DE MIGRAÇÃO

### Código - Producers (9 tópicos!)
- [ ] `management.positions.opened` → `crypto.management.positions.opened`
- [ ] `management.positions.closed` → `crypto.management.positions.closed`
- [ ] `management.positions.updated` → `crypto.management.positions.updated`
- [ ] `management.control.strategy` → `crypto.management.control.strategies`
- [ ] `management.control.risk` → `crypto.management.control.risk`
- [ ] `management.control.mode` → `crypto.management.control.mode`
- [ ] `crypto-listener.subscribe` → `crypto.listener.subscribe`
- [ ] `crypto-listener.unsubscribe` → `crypto.listener.unsubscribe`
- [ ] `notifications.send` → `crypto.notifications.send`

### Código - Consumers (6 tópicos!)
- [ ] `orders.events` → `crypto.trader.orders`
- [ ] `signals.buy` → `crypto.signals.buy`
- [ ] `signals.sell` → `crypto.signals.sell`
- [ ] **ADICIONAR** `crypto.webhook.signals.buy`
- [ ] **ADICIONAR** `crypto.webhook.signals.sell`
- [ ] `crypto-listener.prices` → `crypto.listener.prices`

### Configuração
- [ ] Atualizar `Settings` struct (MUITAS variáveis)
- [ ] Atualizar `.env.example`:
  ```env
  # ===== ENTRADA =====
  # Ordens
  KAFKA_TOPIC_ORDERS_IN=crypto.trader.orders
  
  # Sinais Internos
  KAFKA_TOPIC_SIGNALS_BUY_IN=crypto.signals.buy
  KAFKA_TOPIC_SIGNALS_SELL_IN=crypto.signals.sell
  
  # Sinais Externos
  KAFKA_TOPIC_WEBHOOK_BUY_IN=crypto.webhook.signals.buy
  KAFKA_TOPIC_WEBHOOK_SELL_IN=crypto.webhook.signals.sell
  
  # Preços
  KAFKA_TOPIC_PRICES_IN=crypto.listener.prices
  
  # ===== SAÍDA =====
  # Posições
  KAFKA_TOPIC_POSITIONS_OPENED=crypto.management.positions.opened
  KAFKA_TOPIC_POSITIONS_CLOSED=crypto.management.positions.closed
  KAFKA_TOPIC_POSITIONS_UPDATED=crypto.management.positions.updated
  
  # Controle
  KAFKA_TOPIC_CONTROL_STRATEGIES=crypto.management.control.strategies
  KAFKA_TOPIC_CONTROL_RISK=crypto.management.control.risk
  KAFKA_TOPIC_CONTROL_MODE=crypto.management.control.mode
  
  # Listener Commands
  KAFKA_TOPIC_SUBSCRIBE=crypto.listener.subscribe
  KAFKA_TOPIC_UNSUBSCRIBE=crypto.listener.unsubscribe
  
  # Notificações
  KAFKA_TOPIC_NOTIFICATIONS=crypto.notifications.send
  ```

### Documentação
- [ ] Atualizar `projectmap.yaml`
- [ ] Atualizar `README.md`

### Testes
- [ ] Atualizar testes de integração
- [ ] Adicionar testes para novos tópicos

---

## 📁 ARQUIVOS A MODIFICAR

```
crypto-management/
├── src/
│   ├── infrastructure/
│   │   ├── messaging/
│   │   │   └── kafka/
│   │   │       ├── order_consumer.rs     # crypto.trader.orders
│   │   │       ├── signal_consumer.rs    # crypto.signals.* + crypto.webhook.signals.*
│   │   │       ├── price_consumer.rs     # crypto.listener.prices
│   │   │       ├── position_producer.rs  # crypto.management.positions.*
│   │   │       ├── control_producer.rs   # crypto.management.control.*
│   │   │       └── listener_producer.rs  # crypto.listener.subscribe/unsubscribe
│   │   └── config/
│   │       └── settings.rs
├── .env.example
└── docs/
    └── projectmap.yaml
```

---

## 🧠 LEMBRETES IMPORTANTES (BOUNDARIES)

### VOCÊ É O CÉREBRO - MAS NÃO O EXECUTOR!

Ao publicar comandos:

```rust
// ✅ CORRETO - Publicar comando no Kafka
kafka.send("crypto.management.control.risk", RiskControl {
    action: HaltTrading,
    reason: "Max drawdown exceeded",
}).await;

// ❌ ERRADO - Executar diretamente
binance_api.cancel_all_orders().await; // ISSO É crypto-trader!
```

### AO DETECTAR TRADE MANUAL:

```rust
// 1. Criar posição localmente
let position = create_position(user_data_event);

// 2. Publicar evento de posição aberta
kafka.send("crypto.management.positions.opened", position).await;

// 3. Solicitar subscription de preços
kafka.send("crypto.listener.subscribe", SubscribeCommand {
    symbol: position.symbol,
}).await;

// 4. Notificar
kafka.send("crypto.notifications.send", Notification {
    title: "Manual Trade Detected",
    ...
}).await;
```

---

## ✅ VALIDAÇÃO

1. **Consumer de ordens:**
   ```bash
   echo '{"event_type":"FILLED","symbol":"BTCUSDT","side":"BUY"}' | \
     kafka-console-producer --topic crypto.trader.orders --bootstrap-server localhost:9092
   ```

2. **Producer de posições:**
   ```bash
   kafka-console-consumer --topic crypto.management.positions.opened --bootstrap-server localhost:9092
   ```

3. **Producer de controle:**
   ```bash
   kafka-console-consumer --topic crypto.management.control.strategies --bootstrap-server localhost:9092
   ```

4. **Producer de subscribe:**
   ```bash
   kafka-console-consumer --topic crypto.listener.subscribe --bootstrap-server localhost:9092
   ```

---

## 📚 REFERÊNCIAS

- Guia completo: `_base/KAFKA_TOPICS_MIGRATION.md`
- Boundaries do projeto: `_base/crypto-management/BOUNDARIES.md`
- Mapa do projeto: `_base/crypto-management/projectmap.yaml`
