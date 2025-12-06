# 🔄 MIGRATION PROMPT - crypto-webhook

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

**IMPORTANTE:** crypto-webhook agora tem tópicos PRÓPRIOS para sinais, diferenciando de crypto-signals!

---

## 🔀 MUDANÇAS PARA ESTE PROJETO

### Tópicos PRODUZIDOS (você envia)

| Antigo | Novo | Nota |
|--------|------|------|
| `signals.buy` | `crypto.webhook.signals.buy` | ⚠️ Tópico próprio! |
| `signals.sell` | `crypto.webhook.signals.sell` | ⚠️ Tópico próprio! |
| `notifications.send` | `crypto.notifications.send` | Prefixo adicionado |

### Tópicos CONSUMIDOS (você recebe)

**Nenhum** - crypto-webhook é apenas produtor (recebe HTTP, publica Kafka)

---

## 📋 CHECKLIST DE MIGRAÇÃO

### Código
- [ ] Atualizar producer topics em `infrastructure/messaging/kafka/producer.rs`
- [ ] Alterar de `signals.buy` para `crypto.webhook.signals.buy`
- [ ] Alterar de `signals.sell` para `crypto.webhook.signals.sell`
- [ ] Alterar de `notifications.send` para `crypto.notifications.send`

### Configuração
- [ ] Atualizar `Settings` struct com novos nomes
- [ ] Atualizar `.env.example`:
  ```env
  KAFKA_TOPIC_SIGNALS_BUY=crypto.webhook.signals.buy
  KAFKA_TOPIC_SIGNALS_SELL=crypto.webhook.signals.sell
  KAFKA_TOPIC_NOTIFICATIONS=crypto.notifications.send
  ```

### Documentação
- [ ] Atualizar `projectmap.yaml` (kafka_topics section)
- [ ] Atualizar `README.md` do projeto

### Testes
- [ ] Atualizar testes de integração
- [ ] Atualizar mocks de Kafka

---

## 📁 ARQUIVOS A MODIFICAR

```
crypto-webhook/
├── src/
│   ├── infrastructure/
│   │   ├── messaging/
│   │   │   └── kafka/
│   │   │       └── producer.rs    # Todos os tópicos de saída
│   │   └── config/
│   │       └── settings.rs        # Constantes de tópicos
├── .env.example
└── docs/
    └── projectmap.yaml
```

---

## ⚠️ ATENÇÃO: DIFERENCIAÇÃO DE SINAIS

Agora há separação clara entre sinais internos e externos:

| Origem | Tópico |
|--------|--------|
| crypto-signals (análise interna) | `crypto.signals.buy` / `crypto.signals.sell` |
| crypto-webhook (TradingView, etc) | `crypto.webhook.signals.buy` / `crypto.webhook.signals.sell` |

Isso permite que crypto-trader e crypto-management:
- Tratem sinais de forma diferente (prioridade, validação)
- Apliquem regras específicas por origem
- Tenham métricas separadas

---

## ✅ VALIDAÇÃO

Após migração, verificar:

1. **Enviar webhook de teste:**
   ```bash
   curl -X POST http://localhost:8080/webhook/tradingview \
     -H "Content-Type: application/json" \
     -d '{"action":"buy","symbol":"BTCUSDT","price":45000}'
   ```

2. **Verificar tópico correto:**
   ```bash
   kafka-console-consumer --topic crypto.webhook.signals.buy --bootstrap-server localhost:9092
   ```

3. **Confirmar que NÃO vai para crypto.signals.buy:**
   ```bash
   # Este NÃO deve receber a mensagem do webhook
   kafka-console-consumer --topic crypto.signals.buy --bootstrap-server localhost:9092
   ```

---

## 📚 REFERÊNCIAS

- Guia completo: `_base/KAFKA_TOPICS_MIGRATION.md`
- Boundaries do projeto: `_base/crypto-webhook/BOUNDARIES.md`
- Mapa do projeto: `_base/crypto-webhook/projectmap.yaml`
