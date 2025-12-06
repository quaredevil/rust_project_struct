# 🔄 MIGRATION PROMPT - crypto-listener

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
| `crypto-listener-prices` | `crypto.listener.prices` |

### Tópicos CONSUMIDOS (você recebe)

| Antigo | Novo |
|--------|------|
| `crypto-listener-subscription-commands` | `crypto.listener.subscribe` |
| `crypto-listener-unsubscription-commands` | `crypto.listener.unsubscribe` |

---

## 📋 CHECKLIST DE MIGRAÇÃO

### Código
- [ ] Atualizar producer topic em `infrastructure/messaging/kafka/producer.rs`
- [ ] Atualizar consumer topics em `infrastructure/messaging/kafka/consumer.rs`
- [ ] Atualizar constantes de tópicos (se existirem)

### Configuração
- [ ] Atualizar `Settings` struct com novos nomes
- [ ] Atualizar `.env.example`:
  ```env
  KAFKA_TOPIC_PRICES=crypto.listener.prices
  KAFKA_TOPIC_SUBSCRIBE=crypto.listener.subscribe
  KAFKA_TOPIC_UNSUBSCRIBE=crypto.listener.unsubscribe
  ```

### Documentação
- [ ] Atualizar `projectmap.yaml` (kafka_topics section)
- [ ] Atualizar `README.md` do projeto

### Testes
- [ ] Atualizar testes de integração que usam tópicos
- [ ] Atualizar mocks/stubs de Kafka

---

## 📁 ARQUIVOS A MODIFICAR

```
crypto-listener/
├── src/
│   ├── infrastructure/
│   │   ├── messaging/
│   │   │   └── kafka/
│   │   │       ├── producer.rs    # Tópico: crypto.listener.prices
│   │   │       └── consumer.rs    # Tópicos: crypto.listener.subscribe/unsubscribe
│   │   └── config/
│   │       └── settings.rs        # Constantes de tópicos
├── .env.example                    # Variáveis de ambiente
└── docs/
    └── projectmap.yaml            # Documentação de tópicos
```

---

## ✅ VALIDAÇÃO

Após migração, verificar:

1. **Producer funciona:**
   ```bash
   # Verificar mensagens no novo tópico
   kafka-console-consumer --topic crypto.listener.prices --bootstrap-server localhost:9092
   ```

2. **Consumer funciona:**
   ```bash
   # Enviar comando de subscribe
   echo '{"symbol":"BTCUSDT","requestId":"test-1"}' | kafka-console-producer --topic crypto.listener.subscribe --bootstrap-server localhost:9092
   ```

3. **Logs sem erro:**
   ```bash
   # Verificar logs do serviço
   docker logs crypto-listener 2>&1 | grep -i "kafka\|topic"
   ```

---

## 📚 REFERÊNCIAS

- Guia completo: `_base/KAFKA_TOPICS_MIGRATION.md`
- Boundaries do projeto: `_base/crypto-listener/BOUNDARIES.md`
- Mapa do projeto: `_base/crypto-listener/projectmap.yaml`
