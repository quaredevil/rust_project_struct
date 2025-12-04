# 🚨 FRONTEIRAS ESTRITAS - CRYPTO-NOTIFICATIONS

## ⚠️ Este projeto É:
- ✅ Um **DISTRIBUIDOR** de notificações
- ✅ Um **FORMATADOR** de mensagens
- ✅ Um **GERENCIADOR** de canais (Telegram, Discord, Email)
- ✅ Um **CONSUMIDOR** de eventos de todos os projetos

## ❌ Este projeto NÃO É:

### 1. NÃO é um Executor de Ordens
```yaml
❌ PROIBIDO:
  - Executar ordens na exchange
  - Conectar com Binance API para criar ordens
  - Gerenciar stops
  - Modificar ordens
  
✅ SOLUÇÃO:
  - Você APENAS notifica sobre ordens já executadas
  - Consome events de orders.events
  - NÃO toma decisões sobre ordens
```

### 2. NÃO é um Gerador de Sinais
```yaml
❌ PROIBIDO:
  - Analisar mercado
  - Calcular indicadores técnicos
  - Gerar sinais de BUY/SELL
  - Decidir quando alertar baseado em análise
  
✅ SOLUÇÃO:
  - Você APENAS notifica quando RECEBE um evento
  - Não analise mercado para decidir se envia
  - Consome eventos já prontos
```

### 3. NÃO é um Gerenciador de Posições
```yaml
❌ PROIBIDO:
  - Calcular P&L de posições
  - Rastrear posições abertas
  - Detectar mudanças de posição
  - Gerenciar portfolio
  
✅ SOLUÇÃO:
  - Consome eventos de posições do crypto-management
  - APENAS formata e envia
  - Os dados já vêm prontos no evento
```

### 4. NÃO Decide QUANDO Notificar
```yaml
❌ PROIBIDO:
  - "Vou analisar se o usuário realmente quer ser notificado"
  - "Vou verificar se o evento é importante antes de notificar"
  - "Vou consultar regras de negócio para decidir se envio"
  
✅ SOLUÇÃO:
  - Se você RECEBEU o evento, é porque DEVE notificar
  - Sua única decisão: qual canal usar (baseado em preferências)
  - Outros projetos já decidiram que o evento é relevante
```

### 5. NÃO é um Receptor de Webhooks
```yaml
❌ PROIBIDO:
  - Expor endpoints HTTP
  - Receber webhooks externos
  - Validar assinaturas
  
✅ SOLUÇÃO:
  - crypto-webhook recebe webhooks
  - Você apenas consome eventos do Kafka
```

### 6. NÃO é um Controlador de Sistema
```yaml
❌ PROIBIDO:
  - Enable/disable estratégias
  - Controlar modo de operação
  - Gerenciar configurações de risco
  - Coordenar outros serviços
  
✅ SOLUÇÃO:
  - crypto-management faz controle
  - Você pode NOTIFICAR sobre mudanças
  - Mas não controla
```

---

## 📋 Checklist Antes de Implementar

Pergunte-se:

### ❓ Estou executando ordens ou gerenciando stops?
- Se SIM → **PARE!** Isso é crypto-trader
- **OK:** Notificar sobre ordens executadas

### ❓ Estou analisando mercado ou gerando sinais?
- Se SIM → **PARE!** Isso é crypto-signals
- **OK:** Notificar sobre sinais gerados

### ❓ Estou calculando P&L ou gerenciando posições?
- Se SIM → **PARE!** Isso é crypto-management
- **OK:** Notificar sobre mudanças de posição (dados vêm prontos)

### ❓ Estou decidindo SE devo notificar baseado em lógica de negócio?
- Se SIM → **PARE!** Evento recebido = deve notificar
- **OK:** Decidir QUAL canal usar (Telegram vs Email)

### ❓ Estou recebendo webhooks HTTP?
- Se SIM → **PARE!** Isso é crypto-webhook

---

## ✅ O QUE VOCÊ PODE/DEVE FAZER

### 1. Consumir Eventos
```rust
// ✅ CORRETO
async fn consume_events() {
    let consumer = KafkaConsumer::new(&[
        "orders.events",
        "signals.buy",
        "signals.sell",
        "management.positions.opened",
        "management.positions.closed",
    ]);
    
    while let Some(event) = consumer.next().await {
        handle_event(event).await;
    }
}
```

### 2. Formatar Mensagens
```rust
// ✅ CORRETO - Formatação para cada canal
async fn format_for_telegram(event: OrderFilledEvent) -> String {
    format!(
        "🎯 *Order Filled*\n\
         Symbol: {}\n\
         Side: {}\n\
         Quantity: {}\n\
         Price: {}",
        event.symbol,
        event.side,
        event.quantity,
        event.price
    )
}

async fn format_for_discord(event: OrderFilledEvent) -> DiscordEmbed {
    DiscordEmbed::new()
        .title("Order Filled")
        .field("Symbol", event.symbol)
        .field("Price", event.price)
        .color(0x00ff00)
}
```

### 3. Enviar via Múltiplos Canais
```rust
// ✅ CORRETO
async fn send_notification(message: FormattedMessage, channels: Vec<Channel>) {
    for channel in channels {
        match channel {
            Channel::Telegram => telegram_client.send(message).await,
            Channel::Discord => discord_client.send(message).await,
            Channel::Email => email_client.send(message).await,
            Channel::Webhook => webhook_client.send(message).await,
        }
    }
}
```

### 4. Gerenciar Rate Limits
```rust
// ✅ CORRETO
async fn send_with_rate_limit(message: Message, channel: Channel) {
    if rate_limiter.check(channel).await {
        channel_client.send(message).await;
    } else {
        // Queue para enviar depois
        queue.push(message).await;
    }
}
```

### 5. Agrupar Notificações
```rust
// ✅ CORRETO - Evitar spam
async fn batch_notifications(events: Vec<Event>) -> BatchedMessage {
    // Agrupar múltiplos eventos similares
    // Ex: "3 ordens executadas" ao invés de 3 mensagens
}
```

### 6. Aplicar Preferências do Usuário
```rust
// ✅ CORRETO
async fn get_channels_for_user(user_id: UserId, event_type: EventType) -> Vec<Channel> {
    let prefs = load_preferences(user_id).await;
    
    match event_type {
        EventType::OrderFilled => prefs.order_channels,
        EventType::SignalGenerated => prefs.signal_channels,
        EventType::PositionClosed => prefs.position_channels,
    }
}

// ❌ ERRADO - Decidir baseado em lógica de negócio
async fn should_notify(event: Event) -> bool {
    // NÃO FAÇA ISSO!
    if event.is_important() { // Quem decide isso?
        return true;
    }
    // Se recebeu evento, DEVE notificar!
}
```

---

## 🔗 Comunicação com Outros Projetos

### CONSOME (via Kafka):
- ✅ `orders.events` (de crypto-trader)
- ✅ `signals.buy` (de crypto-signals, crypto-webhook)
- ✅ `signals.sell` (de crypto-signals, crypto-webhook)
- ✅ `management.positions.opened` (de crypto-management)
- ✅ `management.positions.closed` (de crypto-management)
- ✅ `management.positions.updated` (de crypto-management)

### PRODUZ (via Kafka):
- ✅ `notifications.delivered` (para auditoria)
- ✅ `notifications.failed` (para retry/alertas)

### PROIBIDO:
- ❌ Chamar APIs de outros microserviços
- ❌ Acessar bancos de dados de outros projetos
- ❌ Consultar exchange APIs
- ❌ Tomar decisões de negócio

---

## 🎯 Mantra do crypto-notifications

```
EU NOTIFICO.
EU NÃO DECIDO O QUE NOTIFICAR (eventos já vêm prontos).
EU NÃO EXECUTO ORDENS (isso é crypto-trader).
EU NÃO GERO SINAIS (isso é crypto-signals).
EU NÃO GERENCIO POSIÇÕES (isso é crypto-management).
EU APENAS FORMATO E ENVIO.
```

---

## 💡 Padrão de Implementação Correto

### Fluxo Completo:
```
1. Outro projeto gera evento (ordem executada, posição aberta, etc.)
2. Evento é publicado no Kafka
3. crypto-notifications consome evento
4. crypto-notifications carrega preferências do usuário
5. crypto-notifications formata mensagem para cada canal
6. crypto-notifications aplica rate limiting
7. crypto-notifications envia via canais configurados
8. crypto-notifications publica confirmação (delivered/failed)
```

### Sua Responsabilidade (passos 3-8):
```rust
// Exemplo completo
async fn notification_flow(event: Event) {
    // 1. Identificar usuário
    let user_id = extract_user_id(&event);
    
    // 2. Carregar preferências
    let prefs = load_preferences(user_id).await;
    
    // 3. Determinar canais
    let channels = get_channels_for_event(&event, &prefs);
    
    // 4. Formatar para cada canal
    for channel in channels {
        let message = match channel {
            Telegram => format_telegram(&event),
            Discord => format_discord(&event),
            Email => format_email(&event),
        };
        
        // 5. Aplicar rate limiting
        if rate_limiter.allow(channel, user_id).await {
            // 6. Enviar
            match send(channel, message).await {
                Ok(_) => publish_delivered(event.id).await,
                Err(e) => publish_failed(event.id, e).await,
            }
        } else {
            // Queue para depois
            queue.push(message).await;
        }
    }
    
    // ✅ PARE AQUI! Seu trabalho acabou.
    // Não execute ordens
    // Não gere novos sinais
    // Não calcule P&L
}
```

---

## ⚠️ Armadilhas Comuns

### ❌ ERRADO: "Validação Inteligente"
```rust
// NÃO FAÇA ISSO!
async fn handle_order_event(event: OrderEvent) {
    // ❌ Decidindo se é "importante o suficiente"
    if event.value > 1000.0 {
        send_notification(event).await;
    }
    // Se você recebeu o evento, ENVIE!
}
```

### ❌ ERRADO: "Enriquecimento de Dados"
```rust
// NÃO FAÇA ISSO!
async fn enrich_notification(event: OrderEvent) {
    // ❌ Buscando dados de outros domínios
    let position = crypto_management_api.get_position().await;
    let pnl = calculate_pnl(&position); // ❌ ERRADO!
    
    // Os dados devem VIR COMPLETOS no evento!
}
```

### ✅ CORRETO: "Dados Vêm Prontos"
```rust
// Evento já vem com todos os dados necessários
struct PositionClosedEvent {
    position_id: Uuid,
    symbol: String,
    pnl: f64,           // ✅ Já calculado
    pnl_percent: f64,   // ✅ Já calculado
    duration: String,   // ✅ Já calculado
}

async fn notify_position_closed(event: PositionClosedEvent) {
    // ✅ Apenas formate e envie
    let message = format!(
        "Position closed: {} with P&L: {:.2}%",
        event.symbol, event.pnl_percent
    );
    send(message).await;
}
```

---

**Se você está calculando algo além de formatação, PARE!**

