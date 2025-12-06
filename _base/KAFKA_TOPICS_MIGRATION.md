# 🔄 KAFKA TOPICS MIGRATION GUIDE

> **Data:** Dezembro 2025  
> **Versão:** 1.0  
> **Status:** 📋 PENDENTE DE IMPLEMENTAÇÃO

---

## 📋 RESUMO DA MUDANÇA

### Padrão Antigo (DEPRECADO)
```
Mistura de:
- crypto-listener-prices (hífen)
- signals.buy (sem prefixo de projeto)
- orders.events (sem prefixo de projeto)
- management.positions.opened (sem prefixo completo)
```

### Padrão Novo (ADOTADO)
```
crypto.{projeto}.{recurso}.{ação}

Exemplos:
- crypto.listener.prices
- crypto.signals.buy
- crypto.trader.orders
- crypto.management.positions.opened
```

---

## 🗺️ MAPA COMPLETO DE TÓPICOS

### Dados de Mercado (crypto.listener.*)

| Tópico | Tipo | Produtor | Consumidor(es) | Schema |
|--------|------|----------|----------------|--------|
| `crypto.listener.prices` | Evento | crypto-listener | crypto-signals, crypto-management | `crypto.listener.prices.avsc` |
| `crypto.listener.subscribe` | Comando | crypto-management | crypto-listener | `crypto.listener.subscribe.avsc` |
| `crypto.listener.unsubscribe` | Comando | crypto-management | crypto-listener | `crypto.listener.unsubscribe.avsc` |

### Sinais de Trading (crypto.signals.* / crypto.webhook.*)

| Tópico | Tipo | Produtor | Consumidor(es) | Schema |
|--------|------|----------|----------------|--------|
| `crypto.signals.buy` | Evento | crypto-signals | crypto-trader, crypto-management | `crypto.signals.buy.avsc` |
| `crypto.signals.sell` | Evento | crypto-signals | crypto-trader, crypto-management | `crypto.signals.sell.avsc` |
| `crypto.webhook.signals.buy` | Evento | crypto-webhook | crypto-trader, crypto-management | `crypto.signals.buy.avsc` |
| `crypto.webhook.signals.sell` | Evento | crypto-webhook | crypto-trader, crypto-management | `crypto.signals.sell.avsc` |

### Ordens (crypto.trader.*)

| Tópico | Tipo | Produtor | Consumidor(es) | Schema |
|--------|------|----------|----------------|--------|
| `crypto.trader.orders` | Evento | crypto-trader | crypto-management, crypto-notifications | `crypto.trader.orders.avsc` |

### Gestão (crypto.management.*)

| Tópico | Tipo | Produtor | Consumidor(es) | Schema |
|--------|------|----------|----------------|--------|
| `crypto.management.positions.opened` | Evento | crypto-management | crypto-notifications | `crypto.management.positions.avsc` |
| `crypto.management.positions.closed` | Evento | crypto-management | crypto-notifications | `crypto.management.positions.avsc` |
| `crypto.management.positions.updated` | Evento | crypto-management | crypto-notifications | `crypto.management.positions.avsc` |
| `crypto.management.control.strategies` | Comando | crypto-management | crypto-signals | `crypto.management.control.strategies.avsc` |
| `crypto.management.control.risk` | Comando | crypto-management | crypto-trader | `crypto.management.control.risk.avsc` |
| `crypto.management.control.mode` | Comando | crypto-management | crypto-trader | `crypto.management.control.mode.avsc` |

### Notificações (crypto.notifications.*)

| Tópico | Tipo | Produtor | Consumidor(es) | Schema |
|--------|------|----------|----------------|--------|
| `crypto.notifications.send` | Comando | TODOS | crypto-notifications | `crypto.notifications.send.avsc` |
| `crypto.notifications.delivered` | Evento | crypto-notifications | (monitoring) | `crypto.notifications.delivered.avsc` |
| `crypto.notifications.failed` | Evento | crypto-notifications | crypto-management | `crypto.notifications.failed.avsc` |

---

## 🔀 TABELA DE MIGRAÇÃO (DE → PARA)

| # | Tópico Antigo | Tópico Novo |
|---|---------------|-------------|
| 1 | `crypto-listener-prices` | `crypto.listener.prices` |
| 2 | `crypto-listener-subscription-commands` | `crypto.listener.subscribe` |
| 3 | `crypto-listener-unsubscription-commands` | `crypto.listener.unsubscribe` |
| 4 | `signals.buy` | `crypto.signals.buy` |
| 5 | `signals.sell` | `crypto.signals.sell` |
| 6 | (novo) | `crypto.webhook.signals.buy` |
| 7 | (novo) | `crypto.webhook.signals.sell` |
| 8 | `orders.events` | `crypto.trader.orders` |
| 9 | `management.positions.opened` | `crypto.management.positions.opened` |
| 10 | `management.positions.closed` | `crypto.management.positions.closed` |
| 11 | `management.positions.updated` | `crypto.management.positions.updated` |
| 12 | `management.control.strategy` | `crypto.management.control.strategies` |
| 13 | `management.control.risk` | `crypto.management.control.risk` |
| 14 | `management.control.mode` | `crypto.management.control.mode` |
| 15 | `notifications.send` | `crypto.notifications.send` |
| 16 | `notifications.delivered` | `crypto.notifications.delivered` |
| 17 | `notifications.failed` | `crypto.notifications.failed` |

---

## 📁 MIGRAÇÃO DE SCHEMAS AVRO

| # | Arquivo Antigo | Arquivo Novo |
|---|----------------|--------------|
| 1 | `crypto_listener_price_update.avsc` | `crypto.listener.prices.avsc` |
| 2 | `crypto_listener_subscribe_command.avsc` | `crypto.listener.subscribe.avsc` |
| 3 | `crypto_listener_unsubscribe_command.avsc` | `crypto.listener.unsubscribe.avsc` |
| 4 | `crypto_trading_signal_buy.avsc` | `crypto.signals.buy.avsc` |
| 5 | `crypto_trading_signal_sell.avsc` | `crypto.signals.sell.avsc` |
| 6 | `crypto_trading_order_event.avsc` | `crypto.trader.orders.avsc` |
| 7 | `crypto_trading_risk_control.avsc` | `crypto.management.control.risk.avsc` |
| 8 | `crypto_notification.avsc` | `crypto.notifications.send.avsc` |
| 9 | `crypto_notification_delivered.avsc` | `crypto.notifications.delivered.avsc` |
| 10 | `crypto_notification_failed.avsc` | `crypto.notifications.failed.avsc` |
| 11 | `crypto_notification_throttled.avsc` | `crypto.notifications.throttled.avsc` |

---

## 📊 DIAGRAMA DE FLUXO

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│                              KAFKA CLUSTER                                       │
│                                                                                  │
│  ┌─────────────────────────────────────────────────────────────────────────────┐│
│  │                         crypto.listener.*                                    ││
│  │  .prices ◄── crypto-listener                                                ││
│  │  .subscribe ──► crypto-listener (comando de crypto-management)              ││
│  │  .unsubscribe ──► crypto-listener (comando de crypto-management)            ││
│  └─────────────────────────────────────────────────────────────────────────────┘│
│                                    │                                             │
│                                    │ crypto.listener.prices                      │
│                                    ▼                                             │
│  ┌─────────────────────────────────────────────────────────────────────────────┐│
│  │                         crypto.signals.*                                     ││
│  │  .buy ◄── crypto-signals ──► crypto-trader, crypto-management               ││
│  │  .sell ◄── crypto-signals ──► crypto-trader, crypto-management              ││
│  └─────────────────────────────────────────────────────────────────────────────┘│
│                                                                                  │
│  ┌─────────────────────────────────────────────────────────────────────────────┐│
│  │                         crypto.webhook.*                                     ││
│  │  .signals.buy ◄── crypto-webhook ──► crypto-trader, crypto-management       ││
│  │  .signals.sell ◄── crypto-webhook ──► crypto-trader, crypto-management      ││
│  └─────────────────────────────────────────────────────────────────────────────┘│
│                                    │                                             │
│                                    │ crypto.signals.* / crypto.webhook.signals.*│
│                                    ▼                                             │
│  ┌─────────────────────────────────────────────────────────────────────────────┐│
│  │                         crypto.trader.*                                      ││
│  │  .orders ◄── crypto-trader ──► crypto-management, crypto-notifications      ││
│  └─────────────────────────────────────────────────────────────────────────────┘│
│                                    │                                             │
│                                    │ crypto.trader.orders                        │
│                                    ▼                                             │
│  ┌─────────────────────────────────────────────────────────────────────────────┐│
│  │                       crypto.management.*                                    ││
│  │  .positions.opened ◄── crypto-management ──► crypto-notifications           ││
│  │  .positions.closed ◄── crypto-management ──► crypto-notifications           ││
│  │  .positions.updated ◄── crypto-management ──► crypto-notifications          ││
│  │  .control.strategies ◄── crypto-management ──► crypto-signals               ││
│  │  .control.risk ◄── crypto-management ──► crypto-trader                      ││
│  │  .control.mode ◄── crypto-management ──► crypto-trader                      ││
│  └─────────────────────────────────────────────────────────────────────────────┘│
│                                    │                                             │
│                                    ▼                                             │
│  ┌─────────────────────────────────────────────────────────────────────────────┐│
│  │                      crypto.notifications.*                                  ││
│  │  .send ◄── TODOS ──► crypto-notifications                                   ││
│  │  .delivered ◄── crypto-notifications ──► (monitoring)                       ││
│  │  .failed ◄── crypto-notifications ──► crypto-management                     ││
│  └─────────────────────────────────────────────────────────────────────────────┘│
│                                                                                  │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

## ✅ CHECKLIST DE MIGRAÇÃO POR PROJETO

### crypto-listener
- [ ] Atualizar producer para `crypto.listener.prices`
- [ ] Atualizar consumer de `crypto.listener.subscribe`
- [ ] Atualizar consumer de `crypto.listener.unsubscribe`
- [ ] Atualizar variáveis de ambiente
- [ ] Atualizar testes

### crypto-webhook
- [ ] Atualizar producer para `crypto.webhook.signals.buy`
- [ ] Atualizar producer para `crypto.webhook.signals.sell`
- [ ] Atualizar producer para `crypto.notifications.send`
- [ ] Atualizar variáveis de ambiente

### crypto-signals
- [ ] Atualizar consumer de `crypto.listener.prices`
- [ ] Atualizar consumer de `crypto.management.control.strategies`
- [ ] Atualizar producer para `crypto.signals.buy`
- [ ] Atualizar producer para `crypto.signals.sell`
- [ ] Atualizar producer para `crypto.notifications.send`
- [ ] Atualizar variáveis de ambiente

### crypto-trader
- [ ] Atualizar consumer de `crypto.signals.buy`
- [ ] Atualizar consumer de `crypto.signals.sell`
- [ ] Atualizar consumer de `crypto.webhook.signals.buy`
- [ ] Atualizar consumer de `crypto.webhook.signals.sell`
- [ ] Atualizar consumer de `crypto.management.control.risk`
- [ ] Atualizar consumer de `crypto.management.control.mode`
- [ ] Atualizar producer para `crypto.trader.orders`
- [ ] Atualizar producer para `crypto.notifications.send`
- [ ] Atualizar variáveis de ambiente

### crypto-management
- [ ] Atualizar consumer de `crypto.trader.orders`
- [ ] Atualizar consumer de `crypto.signals.buy`
- [ ] Atualizar consumer de `crypto.signals.sell`
- [ ] Atualizar consumer de `crypto.listener.prices`
- [ ] Atualizar producer para `crypto.management.positions.opened`
- [ ] Atualizar producer para `crypto.management.positions.closed`
- [ ] Atualizar producer para `crypto.management.positions.updated`
- [ ] Atualizar producer para `crypto.management.control.strategies`
- [ ] Atualizar producer para `crypto.management.control.risk`
- [ ] Atualizar producer para `crypto.management.control.mode`
- [ ] Atualizar producer para `crypto.listener.subscribe`
- [ ] Atualizar producer para `crypto.listener.unsubscribe`
- [ ] Atualizar producer para `crypto.notifications.send`
- [ ] Atualizar variáveis de ambiente

### crypto-notifications
- [ ] Atualizar consumer de `crypto.notifications.send`
- [ ] Atualizar consumer de `crypto.trader.orders` (broadcast)
- [ ] Atualizar consumer de `crypto.management.positions.*` (broadcast)
- [ ] Atualizar producer para `crypto.notifications.delivered`
- [ ] Atualizar producer para `crypto.notifications.failed`
- [ ] Atualizar variáveis de ambiente

---

## 🚀 ESTRATÉGIA DE DEPLOY

### Opção 1: Big Bang (Mais Simples)
1. Parar todos os serviços
2. Criar novos tópicos no Kafka
3. Deploy de todos os serviços com nova configuração
4. Reiniciar
5. Deletar tópicos antigos após validação

### Opção 2: Gradual (Mais Seguro)
1. Criar novos tópicos (mantendo antigos)
2. Deploy de producers primeiro (produzem em ambos)
3. Deploy de consumers (consomem de novos)
4. Validar fluxo completo
5. Remover produção para tópicos antigos
6. Deletar tópicos antigos

---

## 📝 VARIÁVEIS DE AMBIENTE

### Padrão por Projeto
```env
# crypto-listener
KAFKA_TOPIC_PRICES=crypto.listener.prices
KAFKA_TOPIC_SUBSCRIBE=crypto.listener.subscribe
KAFKA_TOPIC_UNSUBSCRIBE=crypto.listener.unsubscribe

# crypto-signals
KAFKA_TOPIC_PRICES_IN=crypto.listener.prices
KAFKA_TOPIC_CONTROL_IN=crypto.management.control.strategies
KAFKA_TOPIC_SIGNALS_BUY_OUT=crypto.signals.buy
KAFKA_TOPIC_SIGNALS_SELL_OUT=crypto.signals.sell
KAFKA_TOPIC_NOTIFICATIONS_OUT=crypto.notifications.send

# crypto-webhook
KAFKA_TOPIC_SIGNALS_BUY_OUT=crypto.webhook.signals.buy
KAFKA_TOPIC_SIGNALS_SELL_OUT=crypto.webhook.signals.sell
KAFKA_TOPIC_NOTIFICATIONS_OUT=crypto.notifications.send

# crypto-trader
KAFKA_TOPIC_SIGNALS_BUY_IN=crypto.signals.buy
KAFKA_TOPIC_SIGNALS_SELL_IN=crypto.signals.sell
KAFKA_TOPIC_WEBHOOK_SIGNALS_BUY_IN=crypto.webhook.signals.buy
KAFKA_TOPIC_WEBHOOK_SIGNALS_SELL_IN=crypto.webhook.signals.sell
KAFKA_TOPIC_CONTROL_RISK_IN=crypto.management.control.risk
KAFKA_TOPIC_CONTROL_MODE_IN=crypto.management.control.mode
KAFKA_TOPIC_ORDERS_OUT=crypto.trader.orders
KAFKA_TOPIC_NOTIFICATIONS_OUT=crypto.notifications.send

# crypto-management
KAFKA_TOPIC_ORDERS_IN=crypto.trader.orders
KAFKA_TOPIC_SIGNALS_BUY_IN=crypto.signals.buy
KAFKA_TOPIC_SIGNALS_SELL_IN=crypto.signals.sell
KAFKA_TOPIC_PRICES_IN=crypto.listener.prices
KAFKA_TOPIC_POSITIONS_OPENED_OUT=crypto.management.positions.opened
KAFKA_TOPIC_POSITIONS_CLOSED_OUT=crypto.management.positions.closed
KAFKA_TOPIC_POSITIONS_UPDATED_OUT=crypto.management.positions.updated
KAFKA_TOPIC_CONTROL_STRATEGIES_OUT=crypto.management.control.strategies
KAFKA_TOPIC_CONTROL_RISK_OUT=crypto.management.control.risk
KAFKA_TOPIC_CONTROL_MODE_OUT=crypto.management.control.mode
KAFKA_TOPIC_SUBSCRIBE_OUT=crypto.listener.subscribe
KAFKA_TOPIC_UNSUBSCRIBE_OUT=crypto.listener.unsubscribe
KAFKA_TOPIC_NOTIFICATIONS_OUT=crypto.notifications.send

# crypto-notifications
KAFKA_TOPIC_NOTIFICATIONS_IN=crypto.notifications.send
KAFKA_TOPIC_ORDERS_IN=crypto.trader.orders
KAFKA_TOPIC_POSITIONS_IN=crypto.management.positions.*
KAFKA_TOPIC_DELIVERED_OUT=crypto.notifications.delivered
KAFKA_TOPIC_FAILED_OUT=crypto.notifications.failed
```

---

**Versão:** 1.0  
**Autor:** Documentação do Ecossistema  
**Próxima Revisão:** Após implementação
