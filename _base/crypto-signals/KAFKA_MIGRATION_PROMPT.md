# 🔄 MIGRATION PROMPT - crypto-signals

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
| `signals.buy` | `crypto.signals.buy` |
| `signals.sell` | `crypto.signals.sell` |
| `notifications.send` | `crypto.notifications.send` |

### Tópicos CONSUMIDOS (você recebe)

| Antigo | Novo |
|--------|------|
| `crypto-listener.prices` / `crypto-listener-prices` | `crypto.listener.prices` |
| `management.control.strategy` | `crypto.management.control.strategies` |

---

## 📋 CHECKLIST DE MIGRAÇÃO

### Código - Producers
- [ ] Atualizar producer `signals.buy` → `crypto.signals.buy`
- [ ] Atualizar producer `signals.sell` → `crypto.signals.sell`
- [ ] Atualizar producer `notifications.send` → `crypto.notifications.send`

### Código - Consumers
- [ ] Atualizar consumer de preços → `crypto.listener.prices`
- [ ] Atualizar consumer de controle → `crypto.management.control.strategies`

### Configuração
- [ ] Atualizar `Settings` struct
- [ ] Atualizar `.env.example`:
  ```env
  # Entrada
  KAFKA_TOPIC_PRICES=crypto.listener.prices
  KAFKA_TOPIC_CONTROL=crypto.management.control.strategies
  
  # Saída
  KAFKA_TOPIC_SIGNALS_BUY=crypto.signals.buy
  KAFKA_TOPIC_SIGNALS_SELL=crypto.signals.sell
  KAFKA_TOPIC_NOTIFICATIONS=crypto.notifications.send
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
crypto-signals/
├── src/
│   ├── infrastructure/
│   │   ├── messaging/
│   │   │   └── kafka/
│   │   │       ├── price_consumer.rs    # crypto.listener.prices
│   │   │       ├── control_consumer.rs  # crypto.management.control.strategies
│   │   │       └── signal_producer.rs   # crypto.signals.buy/sell
│   │   └── config/
│   │       └── settings.rs
├── .env.example
└── docs/
    └── projectmap.yaml
```

---

## ⚠️ ATENÇÃO: CONTROLE DE ESTRATÉGIAS

O tópico de controle mudou de:
- `management.control.strategy` → `crypto.management.control.strategies`

Este tópico recebe comandos do crypto-management para:
- ENABLE: Ativar estratégia
- DISABLE: Desativar estratégia
- UPDATE_PARAMS: Atualizar parâmetros

---

## ✅ VALIDAÇÃO

1. **Consumer de preços funciona:**
   ```bash
   # Enviar preço de teste
   echo '{"symbol":"BTCUSDT","price":45000.0,"timestamp":1699999999999}' | \
     kafka-console-producer --topic crypto.listener.prices --bootstrap-server localhost:9092
   
   # Verificar logs do crypto-signals
   docker logs crypto-signals 2>&1 | grep -i "btcusdt"
   ```

2. **Producer de sinais funciona:**
   ```bash
   # Verificar sinais gerados
   kafka-console-consumer --topic crypto.signals.buy --bootstrap-server localhost:9092
   ```

3. **Consumer de controle funciona:**
   ```bash
   # Enviar comando de controle
   echo '{"action":"DISABLE","strategy":"RSI_DIVERGENCE","symbols":["ALL"]}' | \
     kafka-console-producer --topic crypto.management.control.strategies --bootstrap-server localhost:9092
   ```

---

## 📚 REFERÊNCIAS

- Guia completo: `_base/KAFKA_TOPICS_MIGRATION.md`
- Boundaries do projeto: `_base/crypto-signals/BOUNDARIES.md`
- Mapa do projeto: `_base/crypto-signals/projectmap.yaml`
