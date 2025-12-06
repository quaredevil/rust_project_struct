# 🔄 MIGRATION PROMPT - crypto-notifications

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

---

## 🔀 MUDANÇAS PARA ESTE PROJETO

### Tópicos PRODUZIDOS (você envia)

| Antigo | Novo |
|--------|------|
| `notifications.delivered` | `crypto.notifications.delivered` |
| `notifications.failed` | `crypto.notifications.failed` |

### Tópicos CONSUMIDOS (você recebe)

| Antigo | Novo | Descrição |
|--------|------|-----------|
| `notifications.send` | `crypto.notifications.send` | Tópico principal |
| `orders.events` | `crypto.trader.orders` | Broadcast (opcional) |
| `management.positions.opened` | `crypto.management.positions.opened` | Broadcast (opcional) |
| `management.positions.closed` | `crypto.management.positions.closed` | Broadcast (opcional) |
| `management.positions.updated` | `crypto.management.positions.updated` | Broadcast (opcional) |

---

## 📋 CHECKLIST DE MIGRAÇÃO

### Código - Producers
- [ ] `notifications.delivered` → `crypto.notifications.delivered`
- [ ] `notifications.failed` → `crypto.notifications.failed`

### Código - Consumers
- [ ] `notifications.send` → `crypto.notifications.send` (PRINCIPAL)
- [ ] `orders.events` → `crypto.trader.orders` (broadcast)
- [ ] `management.positions.*` → `crypto.management.positions.*` (broadcast)

### Configuração
- [ ] Atualizar `Settings` struct
- [ ] Atualizar `.env.example`:
  ```env
  # Entrada
  KAFKA_TOPIC_NOTIFICATIONS=crypto.notifications.send
  KAFKA_TOPIC_ORDERS=crypto.trader.orders
  KAFKA_TOPIC_POSITIONS=crypto.management.positions.*
  
  # Saída
  KAFKA_TOPIC_DELIVERED=crypto.notifications.delivered
  KAFKA_TOPIC_FAILED=crypto.notifications.failed
  ```

### Documentação
- [ ] Atualizar `projectmap.yaml`
- [ ] Atualizar `README.md`

### Testes
- [ ] Atualizar testes de integração
- [ ] Atualizar mocks de Kafka

---

## 📁 ARQUIVOS A MODIFICAR

```
crypto-notifications/
├── src/
│   ├── infrastructure/
│   │   ├── messaging/
│   │   │   └── kafka/
│   │   │       ├── consumer.rs          # Todos os tópicos de entrada
│   │   │       └── producer.rs          # delivered + failed
│   │   └── config/
│   │       └── settings.rs
├── .env.example
└── docs/
    └── projectmap.yaml
```

---

## 💡 SOBRE TÓPICOS DE BROADCAST

crypto-notifications pode **opcionalmente** consumir diretamente de:
- `crypto.trader.orders` - Para notificar sobre ordens em tempo real
- `crypto.management.positions.*` - Para notificar sobre posições

**Ou** pode receber tudo via `crypto.notifications.send` (outros projetos enviam para este tópico).

### Estratégia Recomendada:

```rust
// Opção 1: Apenas tópico principal (mais simples)
consumer.subscribe(&["crypto.notifications.send"]);

// Opção 2: Tópico principal + broadcasts (mais rico)
consumer.subscribe(&[
    "crypto.notifications.send",
    "crypto.trader.orders",
    "crypto.management.positions.opened",
    "crypto.management.positions.closed",
]);
```

**Trade-off:**
- Opção 1: Outros projetos precisam enviar para `crypto.notifications.send`
- Opção 2: crypto-notifications "escuta" diretamente os eventos e decide o que notificar

---

## ✅ VALIDAÇÃO

1. **Consumer principal funciona:**
   ```bash
   echo '{"channel":"telegram","priority":"high","title":"Test","message":"Hello"}' | \
     kafka-console-producer --topic crypto.notifications.send --bootstrap-server localhost:9092
   ```

2. **Verificar Telegram recebeu:**
   - Checar seu chat do Telegram

3. **Producer de delivered funciona:**
   ```bash
   kafka-console-consumer --topic crypto.notifications.delivered --bootstrap-server localhost:9092
   ```

4. **Consumer de broadcast funciona (se implementado):**
   ```bash
   echo '{"event_type":"FILLED","symbol":"BTCUSDT"}' | \
     kafka-console-producer --topic crypto.trader.orders --bootstrap-server localhost:9092
   # Verificar se gerou notificação automática
   ```

---

## 📚 REFERÊNCIAS

- Guia completo: `_base/KAFKA_TOPICS_MIGRATION.md`
- Boundaries do projeto: `_base/crypto-notifications/BOUNDARIES.md`
- Mapa do projeto: `_base/crypto-notifications/projectmap.yaml`
