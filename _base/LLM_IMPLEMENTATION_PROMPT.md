# 🎯 PROMPT PARA LLM: Implementação de Projeto do Ecossistema Crypto Trading

## 🌐 VISÃO GERAL DO ECOSSISTEMA

Este ecossistema consiste em **6 microserviços Rust** que se comunicam **EXCLUSIVAMENTE via Kafka**.

| Projeto | Mantra | Função Principal |
|---------|--------|------------------|
| **crypto-listener** | "EU ESCUTO O MERCADO" | Ingestão de dados via Binance WebSocket |
| **crypto-webhook** | "EU RECEBO, VALIDO E NORMALIZO" | Gateway HTTP para webhooks externos |
| **crypto-signals** | "EU ANALISO MERCADO. EU GERO SINAIS" | Análise técnica e geração de sinais |
| **crypto-trader** | "EU EXECUTO ORDENS" | Execução de ordens na exchange |
| **crypto-management** | "EU ORQUESTRO E COORDENO" | Cérebro central - posições e risco |
| **crypto-notifications** | "EU NOTIFICO" | Distribuição de alertas multi-canal |

---

## 📋 Instruções Obrigatórias

Antes de implementar QUALQUER código, você DEVE:

### 1. Identificar o Projeto
```
Projeto atual: [crypto-listener | crypto-trader | crypto-signals | crypto-notifications | crypto-webhook | crypto-management]
```

### 2. Ler Documentação de Fronteiras
**ORDEM DE LEITURA OBRIGATÓRIA:**

1. `BOUNDARIES_{projeto_atual}.md` - Fronteiras específicas do projeto
2. `WHICH_PROJECT_DOES_WHAT.md` - Guia rápido de decisão
3. `BOUNDARIES_GUIDE.md` - Guia geral do ecossistema
4. `{projeto}_projectmap.yaml` - Estrutura técnica

### 3. Validar Escopo
Antes de cada funcionalidade, responda:

```yaml
Funcionalidade: [descreva]

Checklist de Validação:
  - [ ] Está nas "responsibilities" do projeto atual?
  - [ ] NÃO está na lista "❌ PROIBIDO" do meu projeto?
  - [ ] NÃO está nas "responsibilities" de outro projeto?
  - [ ] Se preciso de dados de outro projeto: vêm via Kafka?
  - [ ] Se gero dados para outro projeto: publico via Kafka?
  
Se QUALQUER resposta for NÃO ou SIM incorreto:
  → PARE
  → Identifique o projeto correto
  → Use comunicação via Kafka
```

---

## 🚨 REGRAS ABSOLUTAS

### Regra 1: Comunicação APENAS via Kafka
```yaml
✅ PERMITIDO:
  - Publicar eventos em tópicos Kafka
  - Consumir eventos de tópicos Kafka
  - Conhecer SCHEMAS dos eventos (formato JSON/Avro)

❌ PROIBIDO:
  - Chamar APIs REST de outros microserviços
  - Acessar banco de dados de outros projetos
  - Importar código de outros projetos
  - Shared memory ou cache compartilhado
  - gRPC entre microserviços
```

### Regra 2: Responsabilidade Única
```yaml
SE você está implementando:
  - Ingestão de dados de mercado → DEVE ser crypto-listener
  - Análise técnica (RSI, MACD) → DEVE ser crypto-signals
  - Execução de ordens → DEVE ser crypto-trader
  - Envio de notificações → DEVE ser crypto-notifications
  - Recepção de webhooks → DEVE ser crypto-webhook
  - Gestão de posições → DEVE ser crypto-management

SE estiver implementando NO PROJETO ERRADO:
  → PARE imediatamente
  → Explique ao usuário o erro
  → Sugira o projeto correto
```

### Regra 3: Sem Duplicação de Lógica
```yaml
ANTES de implementar qualquer lógica, pergunte:
  "Essa lógica já existe em outro projeto?"
  
  SE SIM:
    → NÃO reimplemente
    → Consuma dados via Kafka
    → Publique comando via Kafka se necessário
```

---

## 🎯 Padrões de Implementação por Projeto

### crypto-listener: "Eu Escuto o Mercado"
```rust
// ✅ PERMITIDO
- Conectar: Binance WebSocket (klines, trades, ticker)
- Construir: Candles a partir de trades
- Publicar: crypto-listener.prices
- Consumir: crypto-listener.subscribe, crypto-listener.unsubscribe
- Persistir: Dados históricos no TimescaleDB

// ❌ PROIBIDO
- Calcular indicadores técnicos (RSI, MACD) → crypto-signals
- Gerar sinais de trading → crypto-signals
- Executar ordens → crypto-trader
- Gerenciar posições → crypto-management
- Receber webhooks HTTP → crypto-webhook
- Enviar notificações diretas → crypto-notifications
```

### crypto-signals: "Eu Analiso e Sinalizo"
```rust
// ✅ PERMITIDO
- Consumir: crypto-listener.prices
- Calcular: RSI, MACD, médias móveis, etc.
- Gerar: Sinais BUY/SELL
- Publicar: signals.buy, signals.sell

// ❌ PROIBIDO
- Executar ordens na exchange
- Enviar notificações diretas (Telegram/Discord)
- Gerenciar posições
- Receber webhooks HTTP
- Conectar ao WebSocket de mercado → crypto-listener
```

### crypto-trader: "Eu Executo Ordens"
```rust
// ✅ PERMITIDO
- Consumir: signals.buy, signals.sell
- Executar: Ordens na exchange (Binance API)
- Gerenciar: Stop loss de UMA ordem específica
- Publicar: orders.events

// ❌ PROIBIDO
- Calcular indicadores técnicos
- Gerar sinais de trading
- Enviar notificações diretas
- Gerenciar portfolio completo
- Receber webhooks HTTP
```

### crypto-notifications: "Eu Formato e Envio"
```rust
// ✅ PERMITIDO
- Consumir: orders.events, signals.*, management.positions.*
- Formatar: Mensagens para cada canal
- Enviar: Telegram, Discord, Email
- Publicar: notifications.delivered, notifications.failed

// ❌ PROIBIDO
- Executar ordens
- Gerar sinais
- Calcular P&L
- Decidir SE deve notificar (apenas COMO e POR ONDE)
- Analisar mercado
```

### crypto-webhook: "Eu Recebo e Normalizo"
```rust
// ✅ PERMITIDO
- Receber: HTTP POST de webhooks externos
- Validar: HMAC, tokens, schemas
- Normalizar: Payloads para formato padrão
- Publicar: signals.buy, signals.sell

// ❌ PROIBIDO
- Executar ordens
- Analisar ou "melhorar" sinais
- Enviar notificações
- Gerar sinais próprios (além de normalizar)
- Validar risco ou posições
```

### crypto-management: "Eu Orquestro e Coordeno"
```rust
// ✅ PERMITIDO
- Consumir: orders.events, signals.*, crypto-listener.prices
- Gerenciar: Posições globais, portfolio
- Calcular: P&L total, exposição, drawdown
- Detectar: Trades manuais (auto-discovery)
- Controlar: Estratégias, modo de operação
- Publicar: management.positions.*, management.control.*

// ❌ PROIBIDO
- Executar ordens diretamente na exchange
- Calcular indicadores técnicos
- Implementar estratégias de análise
- Enviar notificações diretas
- Receber webhooks HTTP
- Conectar ao WebSocket de mercado → crypto-listener
```

---

## 🔍 Perguntas de Validação

### Antes de Conectar ao WebSocket de Mercado:
```
❓ Estou no crypto-listener?
   SE NÃO → PARE! Ingestão de dados só no crypto-listener
   
❓ Estou conectando ao Binance WebSocket?
   SE SIM e NÃO for crypto-listener → PARE!
```

### Antes de Implementar Análise Técnica:
```
❓ Estou no crypto-signals?
   SE NÃO → PARE! Análise técnica só no crypto-signals
   
❓ Estou calculando RSI, MACD, médias, etc.?
   SE SIM e NÃO for crypto-signals → PARE!
```

### Antes de Executar Ordem:
```
❓ Estou no crypto-trader?
   SE NÃO → PARE! Execução só no crypto-trader
   
❓ Estou chamando Binance API para create_order?
   SE SIM e NÃO for crypto-trader → PARE!
```

### Antes de Enviar Notificação:
```
❓ Estou no crypto-notifications?
   SE NÃO → Publique evento no Kafka
   
❓ Estou implementando Telegram/Discord client?
   SE SIM e NÃO for crypto-notifications → PARE!
```

### Antes de Receber Webhook:
```
❓ Estou no crypto-webhook?
   SE NÃO → PARE! Webhooks só no crypto-webhook
   
❓ Estou expondo endpoint HTTP POST?
   SE SIM e NÃO for crypto-webhook → PARE!
```

### Antes de Gerenciar Posições:
```
❓ É posição GLOBAL (portfolio)?
   SE SIM → DEVE ser crypto-management
   
❓ É stop de UMA ordem específica?
   SE SIM → PODE ser crypto-trader
```

---

## 💡 Template de Resposta ao Usuário

Quando detectar violação de fronteiras:

```
❌ VIOLAÇÃO DE FRONTEIRA DETECTADA

Funcionalidade solicitada: [descrever]
Projeto atual: [projeto]
Problema: Esta funcionalidade pertence a: [projeto_correto]

Razão:
- [Explicar por que não pertence ao projeto atual]
- [Explicar qual projeto é responsável]

✅ Solução Correta:
1. Implementar [funcionalidade] no [projeto_correto]
2. [Projeto_correto] publica evento em [topico_kafka]
3. [Projeto_atual] consome evento (se necessário)

Exemplo de código correto:
[Mostrar exemplo via Kafka]

Documentação:
- Consulte: BOUNDARIES_[projeto_correto].md
- Consulte: WHICH_PROJECT_DOES_WHAT.md
```

---

## 🎓 Exemplos de Implementação Correta

### Exemplo 1: Análise → Sinal → Execução → Notificação
```rust
// crypto-signals
async fn generate_signal() {
    let rsi = calculate_rsi(&candles).await;
    if rsi < 30 {
        let signal = Signal { symbol: "BTCUSDT", side: Buy, ... };
        kafka.send("signals.buy", signal).await; // ✅
    }
}

// crypto-trader
async fn handle_signal(signal: Signal) {
    let order = execute_order(signal).await;
    kafka.send("orders.events", OrderFilled { ... }).await; // ✅
}

// crypto-notifications
async fn handle_order_event(event: OrderFilled) {
    telegram.send(format_message(event)).await; // ✅
}
```

### Exemplo 2: Webhook → Normalização → Execução
```rust
// crypto-webhook
#[post("/webhook/tradingview")]
async fn receive_webhook(payload: Payload) {
    let signal = normalize(payload);
    kafka.send("signals.buy", signal).await; // ✅
}

// crypto-trader (consome automaticamente)
// crypto-notifications (notifica automaticamente)
```

---

## 🚫 Exemplos de Implementação ERRADA

### ❌ Exemplo Errado 1: Tudo no mesmo lugar
```rust
// NO crypto-trader (ERRADO!)
async fn process_signal() {
    let rsi = calculate_rsi(); // ❌ Isso é crypto-signals!
    if rsi < 30 {
        let order = execute_order().await; // ✅ OK aqui
        telegram.send("Ordem executada").await; // ❌ Isso é crypto-notifications!
    }
}
```

### ❌ Exemplo Errado 2: Chamada direta entre serviços
```rust
// NO crypto-management (ERRADO!)
async fn close_position() {
    // ❌ Chamando API REST de outro serviço
    let response = trader_api.execute_order().await;
    
    // ✅ CORRETO seria:
    kafka.send("management.control.close", CloseCommand { ... }).await;
}
```

---

## 📚 Ordem de Consulta

Sempre que implementar, consulte nesta ordem:

1. **BOUNDARIES_{projeto_atual}.md** - "O que EU posso/não posso fazer?"
2. **WHICH_PROJECT_DOES_WHAT.md** - "ONDE implemento esta funcionalidade?"
3. **{projeto}_projectmap.yaml** - "COMO estruturo o código?"
4. **BOUNDARIES_GUIDE.md** - "Exemplos gerais do ecossistema"

---

## ✅ Checklist Final

Antes de gerar código, confirme:

- [ ] Li BOUNDARIES_{projeto_atual}.md
- [ ] Funcionalidade está nas "responsibilities" do projeto
- [ ] Funcionalidade NÃO está na lista "❌ PROIBIDO"
- [ ] NÃO estou duplicando lógica de outro projeto
- [ ] Comunicação inter-projetos é via Kafka
- [ ] Conheço apenas SCHEMAS, não implementação interna de outros projetos

---

**Se qualquer item não foi confirmado: PARE e revise!**

---

**Versão:** 1.0.0  
**Data:** 2025-10-21
# 🎯 GUIA RÁPIDO: QUAL PROJETO IMPLEMENTA O QUÊ?

## ⚡ Referência Rápida para Decisões

Use este guia quando estiver em dúvida sobre ONDE implementar uma funcionalidade.

---

## 📊 Decisão por Funcionalidade

### 🔍 "Preciso analisar preços e gerar sinais"
→ **crypto-signals**
- Calcular indicadores (RSI, MACD, etc.)
- Construir candles
- Executar estratégias de análise
- Gerar sinais BUY/SELL

### 🤖 "Preciso executar uma ordem na exchange"
→ **crypto-trader**
- Criar ordem (market, limit, stop)
- Monitorar execução
- Gerenciar stop loss de UMA ordem
- Retry de falhas

### 📱 "Preciso notificar usuários"
→ **crypto-notifications**
- Enviar Telegram/Discord/Email
- Formatar mensagens
- Gerenciar rate limits de envio
- Agrupar notificações

### 🌐 "Preciso receber webhook do TradingView"
→ **crypto-webhook**
- Expor endpoint HTTP
- Validar HMAC/token
- Normalizar payload
- Publicar sinal no Kafka

### 🧠 "Preciso gerenciar posições ou controlar o sistema"
→ **crypto-management**
- Rastrear posições globais
- Calcular P&L de portfolio
- Auto-discovery de trades manuais
- Aplicar risk management central
- Enable/disable estratégias
- Controlar modo PAPER/LIVE

---

## 🚦 Árvore de Decisão

```
┌─ Envolve análise técnica? ──────────────────────┐
│  (RSI, MACD, indicadores)                       │
│  SIM → crypto-signals                           │
│  NÃO → Continue                                 │
└─────────────────────────────────────────────────┘
           │
           ▼
┌─ Envolve executar ordem na exchange? ───────────┐
│  (create_order, cancel_order)                   │
│  SIM → crypto-trader                            │
│  NÃO → Continue                                 │
└─────────────────────────────────────────────────┘
           │
           ▼
┌─ Envolve enviar mensagens para usuários? ───────┐
│  (Telegram, Discord, Email)                     │
│  SIM → crypto-notifications                     │
│  NÃO → Continue                                 │
└─────────────────────────────────────────────────┘
           │
           ▼
┌─ Envolve receber HTTP de sistemas externos? ────┐
│  (Webhooks, TradingView)                        │
│  SIM → crypto-webhook                           │
│  NÃO → Continue                                 │
└─────────────────────────────────────────────────┘
           │
           ▼
┌─ Envolve gerenciar posições ou controle? ───────┐
│  (Portfolio, P&L, estratégias, risco)           │
│  SIM → crypto-management                        │
│  NÃO → Reavalie o escopo                        │
└─────────────────────────────────────────────────┘
```

---

## 🎲 Casos de Uso Comuns

### Caso 1: "Detectei que RSI está abaixo de 30"
```
❓ Onde implemento?
→ crypto-signals (análise técnica)

✅ Fluxo correto:
1. crypto-signals calcula RSI
2. crypto-signals gera sinal BUY
3. crypto-signals publica em signals.buy
4. crypto-trader consome e executa
5. crypto-trader publica orders.events
6. crypto-notifications consome e notifica
```

### Caso 2: "Recebi webhook do TradingView"
```
❓ Onde implemento?
→ crypto-webhook (recepção HTTP)

✅ Fluxo correto:
1. crypto-webhook recebe POST /webhook/tradingview
2. crypto-webhook valida HMAC
3. crypto-webhook normaliza payload
4. crypto-webhook publica em signals.buy
5. crypto-trader consome e executa
```

### Caso 3: "Ordem foi executada, preciso avisar usuário"
```
❓ Onde implemento notificação?
→ crypto-notifications

❌ ERRADO:
// No crypto-trader
telegram.send("Ordem executada!").await;

✅ CORRETO:
// No crypto-trader
kafka.send("orders.events", OrderFilledEvent { ... }).await;

// No crypto-notifications (automaticamente)
fn handle_order_filled(event: OrderFilledEvent) {
    telegram.send(format_message(event)).await;
}
```

### Caso 4: "Preciso calcular P&L total do usuário"
```
❓ Onde implemento?
→ crypto-management (gestão de portfolio)

✅ Fluxo correto:
1. crypto-management consome orders.events
2. crypto-management atualiza posições
3. crypto-management calcula P&L total
4. crypto-management publica management.positions.updated
5. crypto-notifications notifica usuário com P&L
```

### Caso 5: "Preciso gerenciar trailing stop"
```
❓ Onde implemento?
→ crypto-trader (gerenciamento de ordem)

⚠️ IMPORTANTE:
- Trailing stop de UMA ordem → crypto-trader
- Gestão de múltiplas posições → crypto-management

✅ Exemplo correto:
// crypto-trader
async fn manage_trailing_stop(order_id: OrderId) {
    // Monitora ESTA ordem específica
    // Ajusta stop dinamicamente
}
```

### Caso 6: "Detectei trade manual na exchange"
```
❓ Onde implemento auto-discovery?
→ crypto-management (orquestração)

✅ Fluxo correto:
1. crypto-management escuta User Data Stream (Binance WebSocket)
2. crypto-management detecta ordem não conhecida
3. crypto-management cria posição
4. crypto-management publica management.positions.opened
5. crypto-management solicita crypto-listener.subscribe
```

---

## 🔄 Padrões de Comunicação

### Padrão 1: Consumidor Único
```
Producer              Topic              Consumer
─────────────────────────────────────────────────
crypto-signals   →   signals.buy    →   crypto-trader
crypto-webhook   →   signals.sell   →   crypto-trader
```

### Padrão 2: Broadcast (Múltiplos Consumidores)
```
Producer              Topic              Consumers
──────────────────────────────────────────────────────────
crypto-trader    →   orders.events  →   crypto-management
                                     →   crypto-notifications
```

### Padrão 3: Comando/Controle
```
Producer                Topic                    Consumer
──────────────────────────────────────────────────────────
crypto-management  →  management.control.*  →  crypto-trader
                                            →  crypto-signals
```

---

## ⚠️ Anti-Padrões Comuns

### ❌ Anti-Padrão 1: "Vou fazer tudo aqui"
```rust
// NÃO FAÇA ISSO no crypto-trader
async fn process_signal(signal: Signal) {
    // ❌ Analisando indicadores (crypto-signals)
    let rsi = calculate_rsi();
    
    // ❌ Decidindo se executa (crypto-management)
    if portfolio_exposure < max_exposure {
        // ❌ Executando ordem (OK, mas...)
        let order = execute(signal).await;
        
        // ❌ Notificando diretamente (crypto-notifications)
        telegram.send("Ordem executada").await;
    }
}

// ✅ CORRETO: Apenas execute e publique
async fn process_signal(signal: Signal) {
    let order = execute(signal).await;
    kafka.send("orders.events", order).await;
    // Outros serviços fazem o resto
}
```

### ❌ Anti-Padrão 2: "Vou buscar dados diretamente"
```rust
// NÃO FAÇA ISSO no crypto-notifications
async fn enrich_notification(event: OrderEvent) {
    // ❌ Chamando API de outro serviço
    let position = management_api.get_position().await;
    // ❌ Calculando P&L (crypto-management)
    let pnl = calculate_pnl(position);
    
    send_notification(format!("P&L: {}", pnl)).await;
}

// ✅ CORRETO: Dados vêm no evento
async fn send_notification(event: PositionClosedEvent) {
    // ✅ P&L já vem calculado
    send(format!("P&L: {}", event.pnl)).await;
}
```

### ❌ Anti-Padrão 3: "Vou implementar 'só um pouquinho'"
```rust
// NÃO FAÇA ISSO no crypto-webhook
async fn process_webhook(payload: Payload) {
    let signal = normalize(payload);
    
    // ❌ "Só vou melhorar o stop loss"
    if signal.stop_loss.is_none() {
        signal.stop_loss = calculate_stop(); // NÃO!
    }
    
    // ❌ "Só vou validar se faz sentido"
    if market_looks_good() { // NÃO!
        publish(signal).await;
    }
}

// ✅ CORRETO: Apenas normalize e publique
async fn process_webhook(payload: Payload) {
    let signal = normalize(payload); // Apenas estrutura
    publish(signal).await; // Outros validam
}
```

---

## 📝 Checklist de 30 Segundos

Antes de implementar, responda:

1. [ ] Esta funcionalidade está na lista "responsibilities" do projeto?
2. [ ] NÃO está na lista "O QUE NÃO FAZ" de outro projeto?
3. [ ] Estou usando APENAS Kafka para comunicação inter-projetos?
4. [ ] Se preciso de dados de outro domínio, eles vêm via Kafka?
5. [ ] NÃO estou duplicando lógica que existe em outro projeto?

**Se respondeu NÃO para qualquer pergunta → PARE e revise!**

---

## 🎓 Regra de Ouro

```
CADA PROJETO É UMA ILHA.
AS ILHAS SE COMUNICAM APENAS POR KAFKA (pontes).
NUNCA NADO ENTRE ILHAS (HTTP direto, shared DB, etc.).
```

---

## 📞 Quando em Dúvida

1. Consulte: `BOUNDARIES_{projeto}.md`
2. Consulte: `WHICH_PROJECT_DOES_WHAT.md`
3. Consulte: `BOUNDARIES_GUIDE.md`
4. Consulte: Este arquivo
5. Pergunte: "Isso está nas responsibilities do meu projeto?"
6. Se NÃO: Encontre o projeto correto e use Kafka

---

**Última Atualização:** 2025-01-20  
**Versão:** 1.1.0

