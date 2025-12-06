# 🔄 MIGRATION PROMPT - crypto-trader

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

**IMPORTANTE:** Agora existem tópicos SEPARADOS para sinais internos e externos!

---

## 🔀 MUDANÇAS PARA ESTE PROJETO

### Tópicos PRODUZIDOS (você envia)

| Antigo | Novo |
|--------|------|
| `orders.events` | `crypto.trader.orders` |
| `notifications.send` | `crypto.notifications.send` |

### Tópicos CONSUMIDOS (você recebe)

| Antigo | Novo | Origem |
|--------|------|--------|
| `signals.buy` | `crypto.signals.buy` | crypto-signals (interno) |
| `signals.sell` | `crypto.signals.sell` | crypto-signals (interno) |
| (novo) | `crypto.webhook.signals.buy` | crypto-webhook (externo) |
| (novo) | `crypto.webhook.signals.sell` | crypto-webhook (externo) |
| `management.control.risk` | `crypto.management.control.risk` | crypto-management |
| `management.control.mode` | `crypto.management.control.mode` | crypto-management |

---

## 📋 CHECKLIST DE MIGRAÇÃO

### Código - Producers
- [ ] Atualizar producer `orders.events` → `crypto.trader.orders`
- [ ] Atualizar producer `notifications.send` → `crypto.notifications.send`

### Código - Consumers
- [ ] Atualizar consumer `signals.buy` → `crypto.signals.buy`
- [ ] Atualizar consumer `signals.sell` → `crypto.signals.sell`
- [ ] **ADICIONAR** consumer `crypto.webhook.signals.buy`
- [ ] **ADICIONAR** consumer `crypto.webhook.signals.sell`
- [ ] Atualizar consumer de risco → `crypto.management.control.risk`
- [ ] Atualizar consumer de modo → `crypto.management.control.mode`

### Configuração
- [ ] Atualizar `Settings` struct
- [ ] Atualizar `.env.example`:
  ```env
  # Entrada - Sinais Internos
  KAFKA_TOPIC_SIGNALS_BUY=crypto.signals.buy
  KAFKA_TOPIC_SIGNALS_SELL=crypto.signals.sell
  
  # Entrada - Sinais Externos (Webhook)
  KAFKA_TOPIC_WEBHOOK_SIGNALS_BUY=crypto.webhook.signals.buy
  KAFKA_TOPIC_WEBHOOK_SIGNALS_SELL=crypto.webhook.signals.sell
  
  # Entrada - Controle
  KAFKA_TOPIC_CONTROL_RISK=crypto.management.control.risk
  KAFKA_TOPIC_CONTROL_MODE=crypto.management.control.mode
  
  # Saída
  KAFKA_TOPIC_ORDERS=crypto.trader.orders
  KAFKA_TOPIC_NOTIFICATIONS=crypto.notifications.send
  ```

### Documentação
- [ ] Atualizar `projectmap.yaml`
- [ ] Atualizar `README.md`

### Testes
- [ ] Atualizar testes de integração
- [ ] Adicionar testes para novos tópicos de webhook

---

## 📁 ARQUIVOS A MODIFICAR

```
crypto-trader/
├── src/
│   ├── infrastructure/
│   │   ├── messaging/
│   │   │   └── kafka/
│   │   │       ├── signal_consumer.rs   # 4 tópicos de sinais agora
│   │   │       ├── control_consumer.rs  # risk + mode
│   │   │       └── order_producer.rs    # crypto.trader.orders
│   │   └── config/
│   │       └── settings.rs
├── .env.example
└── docs/
    └── projectmap.yaml
```

---

## ⚠️ ATENÇÃO: SEPARAÇÃO DE SINAIS

Agora crypto-trader DEVE consumir de 4 tópicos de sinais:

| Tópico | Origem | Tratamento |
|--------|--------|------------|
| `crypto.signals.buy` | Análise interna | Confidence calculada |
| `crypto.signals.sell` | Análise interna | Confidence calculada |
| `crypto.webhook.signals.buy` | TradingView, etc. | Confidence fixa (0.8) |
| `crypto.webhook.signals.sell` | TradingView, etc. | Confidence fixa (0.8) |

**Opções de implementação:**

1. **Consumer único com regex:**
   ```rust
   consumer.subscribe(&["crypto.signals.*", "crypto.webhook.signals.*"]);
   ```

2. **Consumers separados:**
   ```rust
   // Permite tratamento diferenciado
   internal_consumer.subscribe(&["crypto.signals.buy", "crypto.signals.sell"]);
   external_consumer.subscribe(&["crypto.webhook.signals.buy", "crypto.webhook.signals.sell"]);
   ```

---

## ✅ VALIDAÇÃO

1. **Receber sinal interno:**
   ```bash
   echo '{"symbol":"BTCUSDT","strategy":"RSI_DIVERGENCE","confidence":0.85}' | \
     kafka-console-producer --topic crypto.signals.buy --bootstrap-server localhost:9092
   ```

2. **Receber sinal externo:**
   ```bash
   echo '{"symbol":"BTCUSDT","strategy":"EXTERNAL_TRADINGVIEW","confidence":0.8}' | \
     kafka-console-producer --topic crypto.webhook.signals.buy --bootstrap-server localhost:9092
   ```

3. **Verificar ordens publicadas:**
   ```bash
   kafka-console-consumer --topic crypto.trader.orders --bootstrap-server localhost:9092
   ```

4. **Testar controle de risco:**
   ```bash
   echo '{"action":"HALT_TRADING","reason":"Max drawdown reached"}' | \
     kafka-console-producer --topic crypto.management.control.risk --bootstrap-server localhost:9092
   ```

---

## 📚 REFERÊNCIAS

- Guia completo: `_base/KAFKA_TOPICS_MIGRATION.md`
- Boundaries do projeto: `_base/crypto-trader/BOUNDARIES.md`
- Mapa do projeto: `_base/crypto-trader/projectmap.yaml`
