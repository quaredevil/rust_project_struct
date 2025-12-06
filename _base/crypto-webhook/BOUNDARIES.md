# 🚨 FRONTEIRAS ESTRITAS - CRYPTO-WEBHOOK

## ⚠️ Este projeto É:
- ✅ Um **RECEPTOR** de webhooks HTTP
- ✅ Um **NORMALIZADOR** de payloads heterogêneos
- ✅ Um **VALIDADOR** de autenticidade (HMAC, tokens)
- ✅ Um **PUBLICADOR** de sinais normalizados no Kafka

## ❌ Este projeto NÃO É:

### 1. NÃO é um Executor de Ordens
```yaml
❌ PROIBIDO:
  - Executar ordens na exchange após receber webhook
  - Conectar com Binance API para criar ordens
  - Gerenciar stops
  - Monitorar execução de ordens
  
✅ SOLUÇÃO:
  - Receba webhook
  - Normalize payload
  - Publique sinal em crypto.webhook.signals.buy/sell
  - crypto-trader consome e executa
```

### 2. NÃO é um Analisador de Dados
```yaml
❌ PROIBIDO:
  - Calcular indicadores técnicos nos dados recebidos
  - Analisar se o sinal é bom ou ruim
  - Aplicar lógica de trading
  - Melhorar ou corrigir sinais recebidos
  
✅ SOLUÇÃO:
  - Você é um "CORREIO"
  - Recebe → Valida → Normaliza → Encaminha
  - NÃO analise ou modifique a intenção
```

### 3. NÃO é um Sistema de Notificações
```yaml
❌ PROIBIDO:
  - Enviar Telegram quando webhook é recebido
  - Enviar emails de confirmação
  - Notificar usuários sobre webhooks
  
✅ SOLUÇÃO:
  - Publique sinal normalizado
  - crypto-notifications consome e notifica
  - Outros serviços consomem eventos
```

### 4. NÃO é um Gerenciador de Posições
```yaml
❌ PROIBIDO:
  - Rastrear se há posição aberta antes de aceitar webhook
  - Calcular P&L
  - Verificar exposição
  - Consultar posições atuais
  
✅ SOLUÇÃO:
  - Apenas normalize e publique
  - crypto-management valida posições
  - crypto-trader valida risco
```

### 5. NÃO Aplica Risk Management
```yaml
❌ PROIBIDO:
  - Validar se o sinal viola limites de risco
  - Rejeitar webhooks baseado em exposição
  - Verificar drawdown
  - Aplicar limites de quantidade
  
✅ SOLUÇÃO:
  - Publique sinal normalizado
  - crypto-management e crypto-trader validam risco
  - Sua validação é APENAS: autenticidade, schema, rate limit
```

### 6. NÃO é um Gerador de Sinais
```yaml
❌ PROIBIDO:
  - Criar sinais adicionais baseados no webhook recebido
  - Combinar múltiplos webhooks em um sinal
  - Melhorar sinais com análise técnica
  
✅ SOLUÇÃO:
  - 1 webhook = 1 sinal normalizado (mapeamento 1:1)
  - Não gere sinais próprios
```

---

## 📋 Checklist Antes de Implementar

Pergunte-se:

### ❓ Estou executando ordens após receber webhook?
- Se SIM → **PARE!** Isso é crypto-trader
- **Solução:** Publique sinal, deixe crypto-trader executar

### ❓ Estou analisando ou melhorando o sinal?
- Se SIM → **PARE!** Isso é crypto-signals
- **OK:** APENAS normalizar formato (estrutura, não conteúdo)

### ❓ Estou enviando notificações sobre webhooks?
- Se SIM → **PARE!** Isso é crypto-notifications
- **OK:** Publicar evento de auditoria

### ❓ Estou validando risco ou posições?
- Se SIM → **PARE!** Isso é crypto-management/crypto-trader
- **OK:** Validar autenticidade, schema, rate limit

### ❓ Estou gerando sinais adicionais?
- Se SIM → **PARE!** 1 webhook = 1 sinal

---

## ✅ O QUE VOCÊ PODE/DEVE FAZER

### 1. Receber Webhooks HTTP
```rust
// ✅ CORRETO
#[axum::post("/webhook/tradingview")]
async fn receive_tradingview(
    headers: HeaderMap,
    body: Json<TradingViewPayload>
) -> Result<Json<WebhookResponse>, WebhookError> {
    // Processar webhook
}

#[axum::post("/webhook/discord")]
async fn receive_discord(body: Json<DiscordPayload>) -> Result<...> {
    // Processar webhook
}
```

### 2. Validar Autenticidade
```rust
// ✅ CORRETO - Validar assinatura HMAC
async fn validate_signature(
    payload: &[u8],
    signature: &str,
    secret: &str
) -> Result<(), AuthError> {
    let expected = hmac_sha256(payload, secret);
    if constant_time_eq(signature, &expected) {
        Ok(())
    } else {
        Err(AuthError::InvalidSignature)
    }
}

// ✅ CORRETO - Validar token
async fn validate_token(token: &str) -> Result<SourceId, AuthError> {
    if let Some(source) = token_store.get(token).await {
        Ok(source.id)
    } else {
        Err(AuthError::InvalidToken)
    }
}
```

### 3. Validar Schema
```rust
// ✅ CORRETO
async fn validate_schema(
    payload: &Value,
    source: SourceType
) -> Result<(), ValidationError> {
    let schema = get_schema_for_source(source);
    schema.validate(payload)?;
    Ok(())
}
```

### 4. Normalizar Payload
```rust
// ✅ CORRETO - Normalização é mudança de ESTRUTURA, não de CONTEÚDO
async fn normalize_tradingview(payload: TradingViewPayload) -> Signal {
    Signal {
        symbol: payload.ticker,           // ✅ Mapear campo
        strategy: format!("EXTERNAL_{}", payload.strategy), // ✅ Prefixar
        source: "tradingview".to_string(), // ✅ Adicionar metadata
        confidence: 0.8,                   // ✅ Default por fonte
        target_price: payload.close,       // ✅ Mapear campo
        stop_loss: payload.stop,           // ✅ Mapear campo
        take_profit: payload.target,       // ✅ Mapear campo
        metadata: json!({                  // ✅ Preservar original
            "original_payload": payload,
        }),
        timestamp: Utc::now(),
    }
}

// ❌ ERRADO - Modificar intenção
async fn normalize_and_improve(payload: TradingViewPayload) -> Signal {
    let mut signal = normalize(payload);
    
    // ❌ NÃO FAÇA ISSO! Analisando/melhorando
    if signal.stop_loss.is_none() {
        signal.stop_loss = Some(calculate_stop_loss(&signal)); // ERRADO!
    }
    
    signal
}
```

### 5. Aplicar Rate Limiting
```rust
// ✅ CORRETO
async fn check_rate_limit(source_id: SourceId) -> Result<(), RateLimitError> {
    let limiter = get_rate_limiter(source_id);
    limiter.check().await
}
```

### 6. Prevenir Replay Attacks
```rust
// ✅ CORRETO
async fn check_replay(request_signature: &str) -> Result<(), ReplayError> {
    if replay_cache.exists(request_signature).await {
        return Err(ReplayError::DuplicateRequest);
    }
    
    replay_cache.set(request_signature, 5.minutes()).await;
    Ok(())
}
```

### 7. Publicar Sinal Normalizado
```rust
// ✅ CORRETO
async fn publish_signal(signal: Signal) -> Result<(), KafkaError> {
    let topic = match signal.side {
        Side::Buy => "crypto.webhook.signals.buy",
        Side::Sell => "crypto.webhook.signals.sell",
    };
    
    kafka_producer.send(topic, &signal).await
}
```

---

## 🔗 Comunicação com Outros Projetos

### CONSOME:
- ✅ Nenhum (é ponto de entrada HTTP)

### PRODUZ (via Kafka):
- ✅ `crypto.webhook.signals.buy` (consumido por crypto-trader)
- ✅ `crypto.webhook.signals.sell` (consumido por crypto-trader)
- ✅ `webhooks.audit` (opcional, para auditoria)

### PROIBIDO:
- ❌ Chamar APIs de outros microserviços
- ❌ Chamar Exchange APIs
- ❌ Acessar bancos de dados de outros projetos

---

## 🎯 Mantra do crypto-webhook

```
EU SOU UM PORTEIRO.
EU RECEBO, VALIDO E NORMALIZO.
EU NÃO EXECUTO ORDENS (isso é crypto-trader).
EU NÃO ANALISO SINAIS (isso é crypto-signals).
EU NÃO NOTIFICO (isso é crypto-notifications).
EU APENAS RECEBO E ENCAMINHO.
```

---

## 💡 Padrão de Implementação Correto

### Fluxo Completo:
```
1. TradingView/Sistema externo envia webhook HTTP
2. crypto-webhook recebe requisição
3. crypto-webhook valida autenticidade (HMAC/token)
4. crypto-webhook valida rate limit
5. crypto-webhook previne replay
6. crypto-webhook valida schema
7. crypto-webhook normaliza payload
8. crypto-webhook publica sinal no Kafka
9. crypto-trader consome sinal
10. crypto-trader executa ordem
```

### Sua Responsabilidade (passos 2-8):
```rust
// Exemplo completo
#[axum::post("/webhook/:source")]
async fn handle_webhook(
    Path(source): Path<String>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Json<WebhookResponse>, WebhookError> {
    // 1. Autenticar
    let source_id = authenticate(&headers, &source).await?;
    
    // 2. Rate limit
    check_rate_limit(source_id).await?;
    
    // 3. Replay prevention
    let signature = compute_signature(&body);
    check_replay(&signature).await?;
    
    // 4. Parse payload
    let payload: Value = serde_json::from_slice(&body)?;
    
    // 5. Validar schema
    validate_schema(&payload, &source).await?;
    
    // 6. Normalizar (ESTRUTURA, não CONTEÚDO)
    let signal = normalize_payload(payload, source).await?;
    
    // 7. Publicar no Kafka
    publish_signal(signal).await?;
    
    // ✅ PARE AQUI! Seu trabalho acabou.
    // crypto-trader executa a ordem
    // crypto-notifications envia alertas
    
    Ok(Json(WebhookResponse {
        status: "accepted",
        webhook_id: Uuid::new_v4(),
    }))
}
```

---

## ⚠️ Armadilhas Comuns

### ❌ ERRADO: "Validação Inteligente"
```rust
// NÃO FAÇA ISSO!
async fn process_webhook(payload: TradingViewPayload) {
    let signal = normalize(payload);
    
    // ❌ Validando se é um bom sinal
    if signal.confidence < 0.5 {
        return Err("Signal confidence too low");
    }
    
    // ❌ Verificando posições atuais
    let has_position = check_if_has_position(&signal.symbol).await;
    if has_position {
        return Err("Already has position");
    }
    
    // Você NÃO é juiz! Apenas normalize e encaminhe!
}
```

### ❌ ERRADO: "Enriquecimento de Dados"
```rust
// NÃO FAÇA ISSO!
async fn normalize_with_enrichment(payload: Payload) -> Signal {
    let mut signal = normalize(payload);
    
    // ❌ Buscando dados adicionais
    let current_price = binance_api.get_price(&signal.symbol).await;
    
    // ❌ Calculando stop loss "melhorado"
    signal.stop_loss = Some(current_price * 0.98);
    
    // NÃO! Use os dados que vieram no webhook!
    signal
}
```

### ✅ CORRETO: "Normalização Pura"
```rust
// Apenas mudança de estrutura, preservando conteúdo
async fn normalize_tradingview(tv: TradingViewPayload) -> Signal {
    Signal {
        // ✅ Mapear campos (estrutura)
        symbol: tv.ticker,
        target_price: tv.close,
        stop_loss: tv.stop,  // ✅ Use o que veio, não calcule
        
        // ✅ Adicionar metadata da fonte
        source: "tradingview",
        
        // ✅ Confidence padrão POR FONTE (não por análise)
        confidence: 0.8,
        
        // ✅ Preservar payload original
        metadata: json!({ "original": tv }),
    }
}
```

---

**Se você está analisando ou executando algo além de normalizar, PARE!**
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
  - Consome events de crypto.trader.orders
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
        "crypto.trader.orders",
        "crypto.webhook.signals.buy",
        "crypto.webhook.signals.sell",
        "crypto.management.positions.opened",
        "crypto.management.positions.closed",
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
- ✅ `crypto.trader.orders` (de crypto-trader)
- ✅ `crypto.webhook.signals.buy` (de crypto-signals, crypto-webhook)
- ✅ `crypto.webhook.signals.sell` (de crypto-signals, crypto-webhook)
- ✅ `crypto.management.positions.opened` (de crypto-management)
- ✅ `crypto.management.positions.closed` (de crypto-management)
- ✅ `crypto.management.positions.updated` (de crypto-management)

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

