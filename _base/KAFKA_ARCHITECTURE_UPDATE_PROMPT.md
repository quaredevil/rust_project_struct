# 🔄 KAFKA ARCHITECTURE UPDATE PROMPT

> **Versão:** 1.0.0  
> **Data:** 2025-12-06  
> **Status:** PENDENTE DE IMPLEMENTAÇÃO

---

## 📋 Resumo das Mudanças

Este documento define as alterações arquiteturais no sistema de mensageria Kafka do ecossistema Crypto Trading. As mudanças visam:

1. **Simplificar tópicos de sinais** - Unificar webhook signals com signals internos
2. **Adicionar gestão de saldo** - crypto-management como dono do estado de saldo
3. **Escalar crypto-listener** - Leader Election com Redis para evitar duplicidade

---

## 1️⃣ MUDANÇA: Unificação de Tópicos de Sinais

### Situação Atual (DEPRECADA)

```
Tópicos de sinais SEPARADOS por origem:
├── crypto.signals.buy           (origem: crypto-signals)
├── crypto.signals.sell          (origem: crypto-signals)
├── crypto.webhook.signals.buy   (origem: crypto-webhook)  ← REMOVER
└── crypto.webhook.signals.sell  (origem: crypto-webhook)  ← REMOVER
```

### Situação Nova (IMPLEMENTAR)

```
Tópicos de sinais UNIFICADOS com campo source:
├── crypto.signals.buy           (origem: crypto-signals, crypto-webhook)
└── crypto.signals.sell          (origem: crypto-signals, crypto-webhook)
```

### Alterações Necessárias

#### A. Schema Avro (`schemas/crypto.signals.buy.avsc` e `crypto.signals.sell.avsc`)

Adicionar campo `source` ao schema:

```json
{
  "type": "record",
  "name": "TradingSignal",
  "namespace": "crypto.signals",
  "fields": [
    {"name": "signal_id", "type": "string", "doc": "UUID único do sinal"},
    {"name": "symbol", "type": "string", "doc": "Par de negociação (ex: BTCUSDT)"},
    {"name": "side", "type": {"type": "enum", "name": "Side", "symbols": ["BUY", "SELL"]}},
    {"name": "price", "type": ["null", "double"], "default": null, "doc": "Preço sugerido (null = market)"},
    {"name": "quantity", "type": ["null", "double"], "default": null, "doc": "Quantidade sugerida"},
    {"name": "stop_loss", "type": ["null", "double"], "default": null},
    {"name": "take_profit", "type": ["null", "double"], "default": null},
    {"name": "confidence", "type": ["null", "double"], "default": null, "doc": "Confiança do sinal (0.0-1.0)"},
    {
      "name": "source",
      "type": {
        "type": "enum",
        "name": "SignalSource",
        "symbols": ["INTERNAL", "WEBHOOK", "MANUAL"],
        "doc": "Origem do sinal"
      },
      "doc": "Indica de onde o sinal foi gerado"
    },
    {"name": "source_details", "type": ["null", "string"], "default": null, "doc": "Detalhes da origem (ex: strategy name, webhook provider)"},
    {"name": "strategy", "type": ["null", "string"], "default": null, "doc": "Nome da estratégia (se INTERNAL)"},
    {"name": "webhook_provider", "type": ["null", "string"], "default": null, "doc": "Provider do webhook (se WEBHOOK): tradingview, alertatron, custom"},
    {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
  ]
}
```

#### B. Projeto: crypto-webhook

**Arquivo:** `crypto-webhook/src/infrastructure/messaging/kafka/producer.rs`

```rust
// ANTES:
kafka.send("crypto.webhook.signals.buy", signal).await;

// DEPOIS:
let signal = TradingSignal {
    source: SignalSource::WEBHOOK,
    source_details: Some("tradingview".to_string()),
    webhook_provider: Some(provider_name),
    // ... outros campos
};
kafka.send("crypto.signals.buy", signal).await;
```

**Atualizar:** `crypto-webhook/projectmap.yaml`
```yaml
# REMOVER:
produced:
  - name: crypto.webhook.signals.buy
  - name: crypto.webhook.signals.sell

# ALTERAR PARA:
produced:
  - name: crypto.signals.buy
    description: Sinais de compra normalizados de webhooks externos
    source_field: WEBHOOK
  - name: crypto.signals.sell
    description: Sinais de venda normalizados de webhooks externos
    source_field: WEBHOOK
```

#### C. Projeto: crypto-trader

**Arquivo:** `crypto-trader/src/infrastructure/messaging/kafka/consumer.rs`

```rust
// ANTES: Consumir 4 tópicos
let topics = [
    "crypto.signals.buy",
    "crypto.signals.sell",
    "crypto.webhook.signals.buy",    // REMOVER
    "crypto.webhook.signals.sell",   // REMOVER
];

// DEPOIS: Consumir 2 tópicos
let topics = [
    "crypto.signals.buy",
    "crypto.signals.sell",
];

// Lógica interna diferencia por source:
match signal.source {
    SignalSource::INTERNAL => process_internal_signal(signal),
    SignalSource::WEBHOOK => process_webhook_signal(signal),
    SignalSource::MANUAL => process_manual_signal(signal),
}
```

#### D. Projeto: crypto-management

Mesma alteração que crypto-trader - consumir apenas 2 tópicos.

#### E. Arquivos Schema a DELETAR

```bash
# DELETAR estes arquivos:
rm schemas/crypto.webhook.signals.buy.avsc
rm schemas/crypto.webhook.signals.sell.avsc
```

> **NOTA:** Estes arquivos não existem ainda nos schemas, mas se existirem, deletar.

---

## 2️⃣ MUDANÇA: Gestão de Saldo pela crypto-management

### Responsabilidade

**crypto-management** é a Source of Truth para saldo da carteira.

### Novo Tópico: `crypto.management.balance.updated`

#### A. Criar Schema Avro (`schemas/crypto.management.balance.avsc`)

```json
{
  "type": "record",
  "name": "BalanceUpdated",
  "namespace": "crypto.management.balance",
  "doc": "Evento emitido quando o saldo de um asset é atualizado",
  "fields": [
    {"name": "event_id", "type": "string", "doc": "UUID único do evento"},
    {"name": "asset", "type": "string", "doc": "Símbolo do asset (BTC, USDT, ETH)"},
    {"name": "free", "type": "double", "doc": "Saldo disponível para trading"},
    {"name": "locked", "type": "double", "doc": "Saldo bloqueado em ordens abertas"},
    {"name": "total", "type": "double", "doc": "Saldo total (free + locked)"},
    {"name": "change_amount", "type": "double", "doc": "Variação do saldo (+/-)"},
    {
      "name": "change_reason",
      "type": {
        "type": "enum",
        "name": "BalanceChangeReason",
        "symbols": ["TRADE", "DEPOSIT", "WITHDRAWAL", "FEE", "TRANSFER", "UNKNOWN"],
        "doc": "Motivo da alteração de saldo"
      }
    },
    {"name": "related_order_id", "type": ["null", "string"], "default": null, "doc": "ID da ordem relacionada (se TRADE ou FEE)"},
    {"name": "timestamp", "type": "long", "logicalType": "timestamp-millis"}
  ]
}
```

#### B. Atualizar crypto-management

**Arquivo:** `crypto-management/projectmap.yaml`

Adicionar na seção `produced`:
```yaml
produced:
  # ... tópicos existentes ...
  
  - name: crypto.management.balance.updated
    description: Evento de atualização de saldo de um asset
    consumers: [crypto-notifications]
    trigger: User Data Stream (Binance) ou reconciliação
    
    schema:
      event_id: UUID
      asset: string
      free: number
      locked: number
      total: number
      change_amount: number
      change_reason: enum [TRADE, DEPOSIT, WITHDRAWAL, FEE, TRANSFER, UNKNOWN]
      related_order_id: optional string
      timestamp: ISO8601
```

Adicionar na seção `components`:
```yaml
components:
  # ... componentes existentes ...
  
  balance_manager:
    description: Gerencia estado de saldo da carteira
    source: User Data Stream (Binance WebSocket)
    features:
      - Recebe eventos de balance update em tempo real
      - Mantém estado atual em Redis (cache) e Postgres (persistência)
      - Publica eventos de mudança significativa
      - Suporta reconciliação manual
    state: Redis (real-time) + Postgres (persistent)
    reconciliation:
      - Polling periódico da API Binance (fallback)
      - Comparação com estado local
      - Auto-correção se divergência detectada
```

#### C. Atualizar crypto-notifications

**Arquivo:** `crypto-notifications/projectmap.yaml`

Adicionar na seção `consumed`:
```yaml
consumed:
  # ... tópicos existentes ...
  
  - name: crypto.management.balance.updated
    description: Eventos de atualização de saldo
    producer: crypto-management
    
    use_case: |
      - Notificar depósitos significativos
      - Alertar sobre saques
      - Informar saldo baixo
```

---

## 3️⃣ MUDANÇA: Leader Election para crypto-listener

### Problema

Múltiplas instâncias de crypto-listener conectando ao mesmo WebSocket da Binance geram mensagens duplicadas no Kafka.

### Solução: Leader Election com Redis

Apenas o pod eleito como **líder** conecta ao WebSocket. Os demais ficam em standby para failover.

#### A. Atualizar crypto-listener/projectmap.yaml

Adicionar nova seção `scaling`:
```yaml
scaling:
  strategy: leader_election
  
  leader_election:
    description: |
      Apenas o pod líder conecta ao WebSocket da Binance.
      Demais pods ficam em standby para failover automático.
      
    technology: Redis
    lock_key: "crypto-listener:leader"
    lock_ttl: 30s
    heartbeat_interval: 10s
    
    behavior:
      on_leader_acquired: |
        1. Conectar ao Binance WebSocket
        2. Subscrever nos símbolos ativos
        3. Começar a publicar em crypto.listener.prices
        
      on_leader_lost: |
        1. Desconectar do WebSocket
        2. Entrar em modo standby
        3. Tentar reacquire lock
        
      failover_time: "~30s (tempo do TTL expirar)"
      
  asset_distribution:
    description: |
      Quando um pod é líder, ele gerencia TODOS os assets.
      Escalabilidade horizontal é via failover, não via sharding.
      
    future_enhancement: |
      Para alta escala (1000+ symbols), considerar:
      - Particionamento por hash do symbol
      - Múltiplos grupos de líderes
      - Cada grupo responsável por subset de symbols
```

#### B. Adicionar seção de componentes

```yaml
components:
  # ... componentes existentes ...
  
  leader_elector:
    description: Gerencia eleição de líder via Redis
    technology: Redis (SET NX EX pattern)
    
    implementation: |
      async fn try_become_leader(&self) -> bool {
          // SET key value NX EX 30
          redis.set_nx_ex("crypto-listener:leader", pod_id, 30).await
      }
      
      async fn maintain_leadership(&self) {
          loop {
              // Renovar lock a cada 10s
              redis.expire("crypto-listener:leader", 30).await;
              tokio::time::sleep(Duration::from_secs(10)).await;
          }
      }
      
      async fn on_leadership_lost(&self) {
          websocket.disconnect().await;
          enter_standby_mode().await;
      }
    
    redis_commands:
      acquire: "SET crypto-listener:leader {pod_id} NX EX 30"
      renew: "EXPIRE crypto-listener:leader 30"
      release: "DEL crypto-listener:leader"
      check: "GET crypto-listener:leader"
```

#### C. Adicionar dependência Redis

```yaml
dependencies:
  # ... dependências existentes ...
  
  redis:
    purpose: Leader election e distributed locking
    connection: Via environment variable REDIS_URL
    required: true
```

#### D. Métricas de Monitoramento

```yaml
metrics:
  # ... métricas existentes ...
  
  leader_election:
    - name: crypto_listener_is_leader
      type: gauge
      description: "1 se este pod é líder, 0 caso contrário"
      labels: [pod_id]
      
    - name: crypto_listener_leadership_transitions
      type: counter
      description: "Número de transições de liderança"
      labels: [transition_type]  # acquired, lost, renewed
      
    - name: crypto_listener_standby_duration_seconds
      type: histogram
      description: "Tempo em standby antes de virar líder"
```

---

## 4️⃣ MATRIZ DE TÓPICOS ATUALIZADA

### Tópicos Finais

| Tópico | Tipo | Produtor(es) | Consumidor(es) |
|--------|------|--------------|----------------|
| `crypto.listener.prices` | Evento | crypto-listener | crypto-signals, crypto-management |
| `crypto.listener.subscribe` | Comando | crypto-management | crypto-listener |
| `crypto.listener.unsubscribe` | Comando | crypto-management | crypto-listener |
| `crypto.signals.buy` | Evento | crypto-signals, **crypto-webhook** | crypto-trader, crypto-management |
| `crypto.signals.sell` | Evento | crypto-signals, **crypto-webhook** | crypto-trader, crypto-management |
| `crypto.trader.orders` | Evento | crypto-trader | crypto-management, crypto-notifications |
| `crypto.management.positions.opened` | Evento | crypto-management | crypto-notifications |
| `crypto.management.positions.closed` | Evento | crypto-management | crypto-notifications |
| `crypto.management.positions.updated` | Evento | crypto-management | crypto-notifications |
| `crypto.management.balance.updated` | Evento | crypto-management | crypto-notifications | **NOVO** |
| `crypto.management.control.strategies` | Comando | crypto-management | crypto-signals |
| `crypto.management.control.risk` | Comando | crypto-management | crypto-trader |
| `crypto.management.control.mode` | Comando | crypto-management | crypto-trader |
| `crypto.notifications.send` | Comando | TODOS | crypto-notifications |
| `crypto.notifications.delivered` | Evento | crypto-notifications | (monitoring) |
| `crypto.notifications.failed` | Evento | crypto-notifications | crypto-management |
| `crypto.notifications.throttled` | Evento | crypto-notifications | (monitoring) |

### Tópicos REMOVIDOS

| Tópico | Motivo |
|--------|--------|
| ~~`crypto.webhook.signals.buy`~~ | Unificado com `crypto.signals.buy` |
| ~~`crypto.webhook.signals.sell`~~ | Unificado com `crypto.signals.sell` |

---

## 5️⃣ CHECKLIST DE IMPLEMENTAÇÃO

### Fase 1: Schemas

- [ ] Atualizar `schemas/crypto.signals.buy.avsc` - adicionar campo `source`
- [ ] Atualizar `schemas/crypto.signals.sell.avsc` - adicionar campo `source`
- [ ] Criar `schemas/crypto.management.balance.avsc`
- [ ] Registrar novos schemas no Schema Registry

### Fase 2: crypto-webhook

- [ ] Atualizar producer para publicar em `crypto.signals.buy/sell`
- [ ] Adicionar campo `source: WEBHOOK` nos sinais
- [ ] Atualizar `projectmap.yaml`
- [ ] Atualizar `BOUNDARIES.md`
- [ ] Remover referências a tópicos antigos

### Fase 3: crypto-trader

- [ ] Remover consumo de `crypto.webhook.signals.*`
- [ ] Adicionar lógica de diferenciação por `source`
- [ ] Atualizar `projectmap.yaml`
- [ ] Atualizar `BOUNDARIES.md`

### Fase 4: crypto-management

- [ ] Implementar `BalanceManager`
- [ ] Conectar ao User Data Stream para balance updates
- [ ] Implementar publicação em `crypto.management.balance.updated`
- [ ] Atualizar `projectmap.yaml`
- [ ] Atualizar `BOUNDARIES.md`
- [ ] Remover consumo de `crypto.webhook.signals.*`

### Fase 5: crypto-listener

- [ ] Implementar `LeaderElector` com Redis
- [ ] Adicionar lógica de standby/failover
- [ ] Adicionar métricas de leader election
- [ ] Atualizar `projectmap.yaml`
- [ ] Atualizar `BOUNDARIES.md`
- [ ] Testar cenários de failover

### Fase 6: crypto-notifications

- [ ] Adicionar consumo de `crypto.management.balance.updated`
- [ ] Implementar templates de notificação para balance
- [ ] Atualizar `projectmap.yaml`
- [ ] Atualizar `BOUNDARIES.md`

### Fase 7: Documentação

- [ ] Atualizar `WHICH_PROJECT_DOES_WHAT.md`
- [ ] Atualizar `BOUNDARIES_GUIDE.md`
- [ ] Atualizar diagramas de fluxo

### Fase 8: Testes e Migração

- [ ] Criar testes de integração para novos fluxos
- [ ] Testar leader election em ambiente dev
- [ ] Testar failover com múltiplos pods
- [ ] Planejar migração zero-downtime (dual-write temporário)

---

## 6️⃣ ESTRATÉGIA DE MIGRAÇÃO

### Migração Zero-Downtime para Tópicos de Sinais

```
Fase A (Dual Write):
1. crypto-webhook publica em AMBOS os tópicos:
   - crypto.webhook.signals.buy (legado)
   - crypto.signals.buy (novo, com source: WEBHOOK)
   
2. crypto-trader/management consomem AMBOS

Fase B (Cutover):
1. Verificar que todos consumers processam corretamente
2. crypto-webhook PARA de publicar no legado
3. crypto-trader/management PARAM de consumir legado

Fase C (Cleanup):
1. Deletar tópicos legados do Kafka
2. Remover código de dual-write
3. Atualizar documentação
```

---

## 7️⃣ PROMPTS PARA LLM POR PROJETO

### Prompt: crypto-webhook

```
Você está atualizando o projeto crypto-webhook.

CONTEXTO:
- Os tópicos crypto.webhook.signals.buy e crypto.webhook.signals.sell estão sendo deprecados
- Sinais devem ser publicados diretamente em crypto.signals.buy e crypto.signals.sell
- Usar campo source: WEBHOOK para identificar origem

TAREFAS:
1. Atualizar producer Kafka para publicar em crypto.signals.buy/sell
2. Adicionar campo source = SignalSource::WEBHOOK em todos sinais
3. Adicionar campo webhook_provider com nome do provedor (tradingview, alertatron, etc)
4. Atualizar projectmap.yaml removendo tópicos antigos
5. Atualizar BOUNDARIES.md

ARQUIVOS A MODIFICAR:
- src/infrastructure/messaging/kafka/producer.rs
- projectmap.yaml
- BOUNDARIES.md

SCHEMA DO SINAL:
{
  signal_id: UUID,
  symbol: string,
  side: BUY | SELL,
  source: WEBHOOK,  // NOVO
  webhook_provider: string,  // NOVO
  // ... outros campos existentes
}
```

### Prompt: crypto-trader

```
Você está atualizando o projeto crypto-trader.

CONTEXTO:
- Os tópicos crypto.webhook.signals.* foram unificados com crypto.signals.*
- Sinais agora têm campo source: INTERNAL | WEBHOOK | MANUAL

TAREFAS:
1. Remover consumo dos tópicos crypto.webhook.signals.buy e crypto.webhook.signals.sell
2. Manter consumo apenas de crypto.signals.buy e crypto.signals.sell
3. Adicionar lógica para diferenciar processamento por source se necessário
4. Atualizar projectmap.yaml
5. Atualizar BOUNDARIES.md

ARQUIVOS A MODIFICAR:
- src/infrastructure/messaging/kafka/consumer.rs
- src/application/services/signal_processor.rs (se existir)
- projectmap.yaml
- BOUNDARIES.md

LÓGICA:
match signal.source {
    SignalSource::INTERNAL => // Sinal de crypto-signals
    SignalSource::WEBHOOK => // Sinal de crypto-webhook
    SignalSource::MANUAL => // Sinal manual
}
```

### Prompt: crypto-management

```
Você está atualizando o projeto crypto-management.

CONTEXTO:
1. Os tópicos crypto.webhook.signals.* foram unificados com crypto.signals.*
2. crypto-management agora é responsável por gerenciar saldo da carteira
3. Novo tópico: crypto.management.balance.updated

TAREFAS:
1. Remover consumo dos tópicos crypto.webhook.signals.*
2. Implementar BalanceManager:
   - Conectar ao User Data Stream (Binance) para balance updates
   - Manter estado em Redis (cache) e Postgres (persistência)
   - Publicar eventos de mudança em crypto.management.balance.updated
3. Atualizar projectmap.yaml com novo componente e tópico
4. Atualizar BOUNDARIES.md

ARQUIVOS A MODIFICAR/CRIAR:
- src/domain/aggregates/balance.rs (criar)
- src/domain/entities/wallet_balance.rs (criar)
- src/application/services/balance_manager.rs (criar)
- src/infrastructure/messaging/kafka/consumer.rs
- src/infrastructure/messaging/kafka/producer.rs
- projectmap.yaml
- BOUNDARIES.md

SCHEMA DO EVENTO:
{
  event_id: UUID,
  asset: string (BTC, USDT),
  free: number,
  locked: number,
  total: number,
  change_amount: number,
  change_reason: TRADE | DEPOSIT | WITHDRAWAL | FEE | TRANSFER | UNKNOWN,
  related_order_id: optional string,
  timestamp: ISO8601
}
```

### Prompt: crypto-listener

```
Você está atualizando o projeto crypto-listener.

CONTEXTO:
- Múltiplas instâncias do crypto-listener causam mensagens duplicadas no Kafka
- Implementar Leader Election com Redis para garantir apenas 1 instância ativa

TAREFAS:
1. Implementar LeaderElector:
   - Usar Redis SET NX EX para lock distribuído
   - Lock key: "crypto-listener:leader"
   - TTL: 30 segundos
   - Heartbeat: 10 segundos
2. Lógica de comportamento:
   - Se líder: conectar ao WebSocket, publicar preços
   - Se não líder: standby, tentar acquire periodicamente
3. Implementar failover automático quando líder cai
4. Adicionar métricas de leader election
5. Atualizar projectmap.yaml
6. Atualizar BOUNDARIES.md

ARQUIVOS A MODIFICAR/CRIAR:
- src/infrastructure/distributed/leader_elector.rs (criar)
- src/infrastructure/distributed/mod.rs (criar)
- src/main.rs (integrar leader election)
- src/infrastructure/messaging/websocket/binance_client.rs
- projectmap.yaml
- BOUNDARIES.md

PADRÃO REDIS:
// Acquire lock
SET crypto-listener:leader {pod_id} NX EX 30

// Renew lock (heartbeat)
EXPIRE crypto-listener:leader 30

// Release lock
DEL crypto-listener:leader

// Check leader
GET crypto-listener:leader

MÉTRICAS:
- crypto_listener_is_leader: gauge (0 ou 1)
- crypto_listener_leadership_transitions: counter
- crypto_listener_standby_duration_seconds: histogram
```

### Prompt: crypto-notifications

```
Você está atualizando o projeto crypto-notifications.

CONTEXTO:
- Novo tópico crypto.management.balance.updated para notificar sobre mudanças de saldo

TAREFAS:
1. Adicionar consumo do tópico crypto.management.balance.updated
2. Implementar formatação de notificação para eventos de balance
3. Atualizar projectmap.yaml
4. Atualizar BOUNDARIES.md

ARQUIVOS A MODIFICAR:
- src/infrastructure/messaging/kafka/consumer.rs
- src/application/services/notification_formatter.rs
- src/domain/entities/notification_template.rs (se existir)
- projectmap.yaml
- BOUNDARIES.md

TEMPLATES DE NOTIFICAÇÃO:
- Depósito: "💰 Depósito de {amount} {asset} detectado. Novo saldo: {total}"
- Saque: "📤 Saque de {amount} {asset}. Novo saldo: {total}"
- Trade: "📊 Saldo {asset} atualizado após trade. Free: {free}, Locked: {locked}"
- Saldo Baixo: "⚠️ Alerta: Saldo de {asset} abaixo do mínimo ({total})"
```

---

## 8️⃣ VALIDAÇÃO PÓS-IMPLEMENTAÇÃO

### Testes Funcionais

```bash
# 1. Testar unificação de sinais
curl -X POST http://crypto-webhook/signal -d '{"symbol":"BTCUSDT","side":"BUY"}'
# Verificar que mensagem aparece em crypto.signals.buy com source=WEBHOOK

# 2. Testar balance update
# Fazer trade manual na Binance
# Verificar evento em crypto.management.balance.updated

# 3. Testar leader election
kubectl scale deployment crypto-listener --replicas=3
# Verificar que apenas 1 pod está como líder
# Matar pod líder
# Verificar que outro assume em ~30s
```

### Métricas a Monitorar

```promql
# Verificar que apenas 1 líder existe
sum(crypto_listener_is_leader) == 1

# Tempo de failover
histogram_quantile(0.99, crypto_listener_standby_duration_seconds_bucket)

# Eventos de balance por minuto
rate(crypto_management_balance_events_total[5m])
```

---

**Autor:** Documentação do Ecossistema Crypto Trading  
**Revisão:** Arquitetura v2.0
