# 🚨 FRONTEIRAS ESTRITAS - CRYPTO-MANAGEMENT

## ⚠️ Este projeto É:
- ✅ O **CÉREBRO** do sistema (orquestrador central)
- ✅ Um **GERENCIADOR** de posições globais
- ✅ Um **CONTROLADOR** de estratégias e modos
- ✅ Um **APLICADOR** de risk management central
- ✅ Um **DETECTOR** de trades manuais (auto-discovery)

## ❌ Este projeto NÃO É:

### 1. NÃO é um Executor de Ordens
```yaml
❌ PROIBIDO:
  - Executar ordens diretamente na exchange
  - Conectar com Binance API para criar/cancelar ordens
  - Gerenciar stops de ordens específicas
  - Monitorar execução de ordens individuais
  
✅ SOLUÇÃO:
  - Publique comandos em Kafka
  - crypto-trader consome e executa
  - Você COORDENA, não EXECUTA
```

### 2. NÃO é um Gerador de Sinais
```yaml
❌ PROIBIDO:
  - Calcular indicadores técnicos
  - Analisar mercado
  - Gerar sinais de BUY/SELL
  - Implementar estratégias de trading
  
✅ SOLUÇÃO:
  - crypto-signals gera sinais
  - Você CONTROLA estratégias (enable/disable)
  - Mas NÃO implementa as estratégias
```

### 3. NÃO é um Sistema de Notificações
```yaml
❌ PROIBIDO:
  - Enviar mensagens para Telegram/Discord/Email
  - Formatar mensagens para usuários
  - Implementar clients de notificação
  - Gerenciar templates de mensagens
  
✅ SOLUÇÃO:
  - Publique eventos no Kafka
  - crypto-notifications consome e notifica
  - Inclua todos os dados necessários no evento
```

### 4. NÃO é um Receptor de Webhooks
```yaml
❌ PROIBIDO:
  - Expor endpoints HTTP para receber webhooks
  - Validar assinaturas de webhooks externos
  - Normalizar payloads do TradingView
  
✅ SOLUÇÃO:
  - crypto-webhook recebe webhooks
  - Você pode expor API REST de CONTROLE (não webhooks)
  - APIs: GET status, POST config, etc.
```

### 5. NÃO Gerencia Stops de Ordens Específicas
```yaml
❌ PROIBIDO:
  - Monitorar stop loss de cada ordem
  - Disparar trailing stops
  - Ajustar stops dinamicamente por ordem
  
✅ SOLUÇÃO:
  - crypto-trader gerencia stops de ordens
  - Você gerencia posições (agregado de ordens)
  - Monitora RESULTADO, não execução
```

---

## 📋 Checklist Antes de Implementar

Pergunte-se:

### ❓ Estou executando ordens diretamente?
- Se SIM → **PARE!** Isso é crypto-trader
- **OK:** Publicar comando para crypto-trader executar

### ❓ Estou calculando indicadores ou gerando sinais?
- Se SIM → **PARE!** Isso é crypto-signals
- **OK:** Enable/disable estratégias, não implementá-las

### ❓ Estou enviando notificações diretas?
- Se SIM → **PARE!** Isso é crypto-notifications
- **Solução:** Publique evento com dados completos

### ❓ Estou recebendo webhooks externos?
- Se SIM → **PARE!** Isso é crypto-webhook
- **OK:** Expor API REST de controle/consulta

### ❓ Estou gerenciando stop loss de ordem individual?
- Se SIM → **PARE!** Isso é crypto-trader
- **OK:** Gerenciar posição (agregado de múltiplas ordens)

---

## ✅ O QUE VOCÊ PODE/DEVE FAZER

### 1. Gerenciar Posições Globais
```rust
// ✅ CORRETO
async fn track_position(order_filled: OrderFilledEvent) {
    let position = positions_store.get_or_create(&order_filled.symbol).await;
    
    match order_filled.side {
        Buy => position.add_entry(order_filled),
        Sell => position.add_exit(order_filled),
    }
    
    // Calcular P&L
    let pnl = calculate_pnl(&position);
    
    // Atualizar estado
    positions_store.update(position).await;
    
    // Publicar evento
    kafka.send("management.positions.updated", PositionUpdatedEvent {
        position_id: position.id,
        unrealized_pnl: pnl,
        // ...
    }).await;
}
```

### 2. Auto-Discovery de Trades Manuais
```rust
// ✅ CORRETO
async fn detect_manual_trade(user_data: UserDataStreamEvent) {
    // Trade não veio do crypto-trader
    if !known_orders.contains(&user_data.order_id) {
        // Nova posição detectada
        let position = create_position_from_manual_trade(user_data);
        
        // Publicar evento
        kafka.send("management.positions.opened", PositionOpenedEvent {
            position_id: position.id,
            source: "manual",
            // ...
        }).await;
        
        // Solicitar subscrição de preços
        kafka.send("crypto-listener.subscribe", SubscribeEvent {
            symbol: position.symbol,
        }).await;
    }
}
```

### 3. Calcular P&L de Portfolio
```rust
// ✅ CORRETO
async fn calculate_portfolio_pnl() -> PortfolioPnL {
    let open_positions = positions_store.get_open().await;
    let closed_positions = positions_store.get_closed_today().await;
    
    let unrealized_pnl = open_positions
        .iter()
        .map(|p| p.unrealized_pnl)
        .sum();
    
    let realized_pnl = closed_positions
        .iter()
        .map(|p| p.realized_pnl)
        .sum();
    
    PortfolioPnL {
        unrealized: unrealized_pnl,
        realized: realized_pnl,
        total: unrealized_pnl + realized_pnl,
    }
}
```

### 4. Aplicar Risk Management Central
```rust
// ✅ CORRETO
async fn validate_risk(signal: SignalEvent) -> Result<(), RiskViolation> {
    let portfolio = get_portfolio_state().await;
    let risk_limits = load_risk_limits().await;
    
    // Validar exposição
    let new_exposure = portfolio.exposure + signal.quantity * signal.price;
    if new_exposure > risk_limits.max_total_exposure {
        return Err(RiskViolation::ExposureExceeded);
    }
    
    // Validar drawdown
    if portfolio.drawdown > risk_limits.max_drawdown {
        return Err(RiskViolation::DrawdownExceeded);
    }
    
    Ok(())
}

// ❌ ERRADO - Executar ordem após validar
async fn validate_and_execute(signal: Signal) {
    validate_risk(&signal).await?;
    // ❌ NÃO FAÇA ISSO!
    exchange_client.create_order(signal).await; // ERRADO!
    
    // ✅ CORRETO: Publicar aprovação
    kafka.send("signals.approved", signal).await;
}
```

### 5. Controlar Estratégias
```rust
// ✅ CORRETO
async fn enable_strategy(strategy: String, symbols: Vec<String>) {
    // Atualizar estado local
    strategy_store.enable(&strategy, &symbols).await;
    
    // Publicar comando
    kafka.send("management.control.strategy", StrategyControlEvent {
        action: "ENABLE",
        strategy,
        symbols,
    }).await;
}

// ❌ ERRADO - Implementar a estratégia
async fn execute_rsi_strategy() {
    // NÃO FAÇA ISSO!
    let rsi = calculate_rsi(&prices); // Isso é crypto-signals
    if rsi < 30 {
        // ...
    }
}
```

### 6. Controlar Modo de Operação
```rust
// ✅ CORRETO
async fn change_mode(mode: OperationMode) {
    // Atualizar estado
    system_state.set_mode(mode).await;
    
    // Publicar comando
    kafka.send("management.control.mode", ModeControlEvent {
        mode,
        timestamp: Utc::now(),
    }).await;
}
```

### 7. Coordenar Subscrições de Assets
```rust
// ✅ CORRETO
async fn subscribe_to_asset(symbol: String, reason: String) {
    kafka.send("crypto-listener.subscribe", SubscribeEvent {
        symbol,
        source: reason,
        priority: "high",
    }).await;
}

async fn unsubscribe_from_asset(symbol: String) {
    // Verificar se ainda há posições abertas
    if !has_open_positions(&symbol).await {
        kafka.send("crypto-listener.unsubscribe", UnsubscribeEvent {
            symbol,
            reason: "position_closed",
        }).await;
    }
}
```

---

## 🔗 Comunicação com Outros Projetos

### CONSOME (via Kafka):
- ✅ `orders.events` (de crypto-trader)
- ✅ `signals.buy` (de crypto-signals, crypto-webhook)
- ✅ `signals.sell` (de crypto-signals, crypto-webhook)
- ✅ `crypto-listener.prices` (para P&L não realizado)

### PRODUZ (via Kafka):
- ✅ `management.positions.opened` (para crypto-notifications)
- ✅ `management.positions.closed` (para crypto-notifications)
- ✅ `management.positions.updated` (para crypto-notifications)
- ✅ `management.control.strategy` (para crypto-signals, crypto-trader)
- ✅ `management.control.risk` (para crypto-trader)
- ✅ `management.control.mode` (para crypto-trader)
- ✅ `crypto-listener.subscribe` (para crypto-listener)
- ✅ `crypto-listener.unsubscribe` (para crypto-listener)

### PROIBIDO:
- ❌ Chamar Exchange API diretamente para executar ordens
- ❌ Implementar análise técnica
- ❌ Enviar notificações diretas (Telegram/Discord)

---

## 🎯 Mantra do crypto-management

```
EU ORQUESTRO E COORDENO.
EU GERENCIO POSIÇÕES, NÃO ORDENS (crypto-trader gerencia ordens).
EU CONTROLO ESTRATÉGIAS, NÃO AS IMPLEMENTO (crypto-signals implementa).
EU PUBLICO COMANDOS, NÃO EXECUTO DIRETAMENTE.
EU SOU O CÉREBRO, NÃO OS BRAÇOS.
```

---

## 💡 Padrão de Implementação Correto

### Fluxo de Orquestração:
```rust
// Exemplo: Coordenar abertura de posição
async fn orchestrate_position_opening(order_filled: OrderFilledEvent) {
    // 1. Criar/atualizar posição
    let position = create_or_update_position(order_filled).await;
    
    // 2. Publicar evento de posição
    kafka.send("management.positions.opened", PositionOpenedEvent {
        position_id: position.id,
        symbol: position.symbol.clone(),
        // ...
    }).await;
    
    // 3. Solicitar subscrição de preços
    kafka.send("crypto-listener.subscribe", SubscribeEvent {
        symbol: position.symbol,
        source: "position_opened",
    }).await;
    
    // ✅ PARE AQUI! Você coordenou, não executou.
    // crypto-notifications envia alertas
    // crypto-listener começa a enviar preços
    // crypto-trader monitora stops (se houver)
}

// Exemplo: Coordenar fechamento de posição
async fn orchestrate_position_closing(order_filled: OrderFilledEvent) {
    // 1. Atualizar posição
    let position = close_position(order_filled).await;
    
    // 2. Calcular P&L
    let pnl = calculate_pnl(&position);
    
    // 3. Publicar evento de fechamento
    kafka.send("management.positions.closed", PositionClosedEvent {
        position_id: position.id,
        pnl: pnl.total,
        pnl_percent: pnl.percent,
        // ...
    }).await;
    
    // 4. Cancelar subscrição (se não houver outras posições)
    if !has_other_positions(&position.symbol).await {
        kafka.send("crypto-listener.unsubscribe", UnsubscribeEvent {
            symbol: position.symbol,
        }).await;
    }
}
```

---

## ⚠️ Armadilhas Comuns

### ❌ ERRADO: "Executar Diretamente"
```rust
// NÃO FAÇA ISSO!
async fn close_position_directly(position_id: Uuid) {
    let position = get_position(position_id).await;
    
    // ❌ Executando ordem diretamente
    exchange_client.create_order(Order {
        symbol: position.symbol,
        side: Sell,
        quantity: position.quantity,
    }).await;
    
    // ERRADO! Publique comando para crypto-trader
}
```

### ❌ ERRADO: "Implementar Estratégia"
```rust
// NÃO FAÇA ISSO!
async fn check_if_should_close_position(position: &Position) {
    // ❌ Analisando mercado
    let current_price = get_price(&position.symbol).await;
    let rsi = calculate_rsi(&prices); // ERRADO!
    
    if rsi > 70 {
        // ❌ Gerando sinal
        close_position(position).await;
    }
    
    // Isso é trabalho do crypto-signals!
}
```

### ✅ CORRETO: "Coordenar via Eventos"
```rust
async fn handle_risk_violation(portfolio: &Portfolio) {
    // ✅ Publicar comando de parada emergencial
    kafka.send("management.control.risk", RiskControlEvent {
        action: "HALT_TRADING",
        reason: "Max drawdown exceeded",
    }).await;
    
    // ✅ Publicar alerta
    kafka.send("management.alerts", AlertEvent {
        severity: "critical",
        message: "Trading halted due to risk violation",
    }).await;
    
    // Outros serviços reagem aos comandos
}
```

---

## 🎓 Seu Papel como Orquestrador

Você é o maestro, não o músico:
- ✅ Você COORDENA quem faz o que
- ✅ Você MANTÉM o estado global
- ✅ Você APLICA regras globais (risco, limites)
- ✅ Você DETECTA anomalias (trades manuais)
- ❌ Você NÃO executa as ações diretamente

**Se você está chamando Exchange API ou implementando estratégias, PARE!**

