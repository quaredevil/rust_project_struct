# 🚨 GUIA DE FRONTEIRAS ESTRITAS DO ECOSSISTEMA

## ⚠️ LEIA ANTES DE IMPLEMENTAR QUALQUER PROJETO

Este documento define as **fronteiras estritas** entre os 6 projetos do ecossistema de crypto trading. O problema que estamos resolvendo é a **EVASÃO DE FRONTEIRAS** - quando um LLM implementa funcionalidades de outro projeto dentro do projeto atual.

---

## 🌐 Visão Geral do Ecossistema

| Projeto | Mantra | Função |
|---------|--------|--------|
| **crypto-listener** | "EU ESCUTO O MERCADO" | Ingestão de dados via WebSocket |
| **crypto-webhook** | "EU RECEBO, VALIDO E NORMALIZO" | Gateway HTTP para webhooks |
| **crypto-signals** | "EU ANALISO MERCADO. EU GERO SINAIS" | Análise técnica e sinais |
| **crypto-trader** | "EU EXECUTO ORDENS" | Execução na exchange |
| **crypto-management** | "EU ORQUESTRO E COORDENO" | Cérebro central |
| **crypto-notifications** | "EU NOTIFICO" | Alertas multi-canal |

---

## 🎯 Problema Identificado

**Sintoma:** Durante a implementação, o LLM tenta implementar funcionalidades que pertencem a OUTROS projetos.

**Exemplos:**
- Implementar análise técnica dentro do `crypto-trader` (deveria estar no `crypto-signals`)
- Implementar envio de notificações dentro do `crypto-management` (deveria estar no `crypto-notifications`)
- Implementar recepção de webhooks dentro do `crypto-signals` (deveria estar no `crypto-webhook`)
- Implementar gerenciamento de posições globais dentro do `crypto-trader` (deveria estar no `crypto-management`)
- Implementar conexão WebSocket de mercado dentro do `crypto-signals` (deveria estar no `crypto-listener`)

---

## 🔒 Princípios Fundamentais

### 1. **Comunicação APENAS via Kafka**
```
✅ CORRETO: Projeto A publica evento → Kafka → Projeto B consome
❌ ERRADO: Projeto A chama REST API do Projeto B
❌ ERRADO: Projeto A acessa banco de dados do Projeto B
❌ ERRADO: Projeto A importa código do Projeto B
```

### 2. **Contratos são Schemas Kafka**
```
✅ CORRETO: Conhecer o SCHEMA do evento Kafka (formato JSON)
❌ ERRADO: Conhecer a IMPLEMENTAÇÃO interna do outro projeto
❌ ERRADO: Conhecer as camadas (domain, application) do outro projeto
```

### 3. **Responsabilidade Única**
```
✅ CORRETO: Cada projeto tem UMA responsabilidade clara
❌ ERRADO: "Já que estou aqui, vou adicionar essa funcionalidade também"
```

### 4. **Sem Duplicação de Lógica**
```
✅ CORRETO: Se a lógica já existe em outro projeto, CONSUMA via Kafka
❌ ERRADO: Reimplementar a mesma lógica "porque é mais fácil"
```

---

## 📊 Matriz de Responsabilidades

| Funcionalidade | Projeto Responsável | Outros Projetos |
|----------------|---------------------|-----------------|
| **Ingestão de Dados de Mercado** | crypto-listener | ❌ NÃO implementam |
| **Conexão WebSocket Binance** | crypto-listener | ❌ NÃO implementam |
| **Análise Técnica** | crypto-signals | ❌ NÃO implementam |
| **Geração de Sinais** | crypto-signals, crypto-webhook | ❌ NÃO implementam |
| **Execução de Ordens** | crypto-trader | ❌ NÃO implementam |
| **Gerenciamento de Stops** | crypto-trader | ❌ NÃO implementam |
| **Notificações Multi-Canal** | crypto-notifications | ❌ NÃO implementam |
| **Recepção de Webhooks** | crypto-webhook | ❌ NÃO implementam |
| **Gerenciamento de Posições** | crypto-management | ❌ NÃO implementam |
| **Auto-Discovery** | crypto-management | ❌ NÃO implementam |
| **Risk Management Central** | crypto-management | ❌ NÃO implementam |
| **Controle de Estratégias** | crypto-management | ❌ NÃO implementam |

---

## 🚫 PROIBIÇÕES POR PROJETO

### crypto-listener (Ingestor de Dados de Mercado)

#### ✅ PODE FAZER:
- Conectar ao Binance WebSocket
- Receber trades e klines em tempo real
- Construir candles a partir de trades
- Publicar preços no Kafka
- Responder a comandos de subscribe/unsubscribe

#### ❌ NÃO PODE FAZER:
```yaml
❌ Calcular indicadores técnicos (RSI, MACD, EMA, etc.)
   → Responsabilidade: crypto-signals
   
❌ Gerar sinais de trading (BUY/SELL)
   → Responsabilidade: crypto-signals
   
❌ Executar ordens na exchange
   → Responsabilidade: crypto-trader
   
❌ Gerenciar posições ou calcular P&L
   → Responsabilidade: crypto-management
   
❌ Receber webhooks HTTP
   → Responsabilidade: crypto-webhook
   
❌ Enviar notificações (Telegram, Discord, Email)
   → Responsabilidade: crypto-notifications
```

---

### crypto-trader (Executor de Ordens)

#### ✅ PODE FAZER:
- Consumir sinais via Kafka
- Executar ordens na exchange
- Gerenciar stops locais (de uma ordem específica)
- Publicar eventos de ordens

#### ❌ NÃO PODE FAZER:
```yaml
❌ Analisar indicadores técnicos (RSI, MACD, etc.)
   → Responsabilidade: crypto-signals
   
❌ Gerar sinais de trading
   → Responsabilidade: crypto-signals
   
❌ Enviar notificações (Telegram, Discord, Email)
   → Responsabilidade: crypto-notifications
   → Solução: Publique evento no Kafka
   
❌ Calcular P&L de portfolio
   → Responsabilidade: crypto-management
   
❌ Detectar trades manuais (auto-discovery)
   → Responsabilidade: crypto-management
   
❌ Gerenciar posições globais
   → Responsabilidade: crypto-management
   
❌ Receber webhooks do TradingView
   → Responsabilidade: crypto-webhook
   
❌ Calcular médias móveis
   → Responsabilidade: crypto-signals
   
❌ Controlar estratégias (enable/disable)
   → Responsabilidade: crypto-management
   
❌ Conectar ao WebSocket de mercado
   → Responsabilidade: crypto-listener
```

---

### crypto-signals (Analisador e Gerador de Sinais)

#### ✅ PODE FAZER:
- Consumir preços em tempo real
- Calcular indicadores técnicos
- Executar estratégias de análise
- Gerar sinais de BUY/SELL
- Publicar sinais no Kafka

#### ❌ NÃO PODE FAZER:
```yaml
❌ Executar ordens na exchange
   → Responsabilidade: crypto-trader
   → Solução: Publique sinal, crypto-trader executa
   
❌ Enviar notificações diretas
   → Responsabilidade: crypto-notifications
   → Solução: Publique evento, crypto-notifications notifica
   
❌ Gerenciar posições
   → Responsabilidade: crypto-management
   
❌ Receber webhooks externos
   → Responsabilidade: crypto-webhook
   → Solução: Consuma sinais já normalizados
   
❌ Calcular P&L de portfolio
   → Responsabilidade: crypto-management
   
❌ Gerenciar stops (stop loss, take profit)
   → Responsabilidade: crypto-trader
   → Solução: Sugira no sinal, crypto-trader aplica
   
❌ Conectar ao WebSocket de mercado
   → Responsabilidade: crypto-listener
   → Solução: Consuma preços do Kafka
```

---

### crypto-notifications (Central de Notificações)

#### ✅ PODE FAZER:
- Consumir eventos de todos os projetos
- Formatar mensagens
- Enviar via múltiplos canais (Telegram, Discord, Email)
- Gerenciar rate limits de envio
- Agrupar notificações

#### ❌ NÃO PODE FAZER:
```yaml
❌ Executar ordens
   → Responsabilidade: crypto-trader
   
❌ Gerar sinais de trading
   → Responsabilidade: crypto-signals
   
❌ Analisar mercado
   → Responsabilidade: crypto-signals
   
❌ Gerenciar posições
   → Responsabilidade: crypto-management
   
❌ Receber webhooks do TradingView
   → Responsabilidade: crypto-webhook
   
❌ Decidir QUANDO notificar
   → Decisão: Já vem do evento publicado
   → Papel: APENAS executar o envio
```

---

### crypto-webhook (Integração com Sistemas Externos)

#### ✅ PODE FAZER:
- Expor endpoints HTTP para webhooks
- Validar assinaturas e autenticação
- Normalizar payloads de diferentes fontes
- Publicar sinais normalizados no Kafka

#### ❌ NÃO PODE FAZER:
```yaml
❌ Executar ordens
   → Responsabilidade: crypto-trader
   → Solução: Publique sinal, crypto-trader executa
   
❌ Analisar dados técnicos
   → Responsabilidade: crypto-signals
   → Papel: APENAS normalizar e encaminhar
   
❌ Enviar notificações
   → Responsabilidade: crypto-notifications
   
❌ Gerenciar posições
   → Responsabilidade: crypto-management
   
❌ Calcular indicadores
   → Responsabilidade: crypto-signals
   → Papel: APENAS receber e publicar
```

---

### crypto-management (Gestão e Orquestração)

#### ✅ PODE FAZER:
- Gerenciar posições globais
- Calcular P&L de portfolio
- Auto-discovery de trades manuais
- Aplicar risk management central
- Controlar estratégias (enable/disable)
- Controlar modo de operação (PAPER/LIVE)

#### ❌ NÃO PODE FAZER:
```yaml
❌ Executar ordens diretamente
   → Responsabilidade: crypto-trader
   → Solução: Publique comando, crypto-trader executa
   
❌ Gerar sinais de trading
   → Responsabilidade: crypto-signals
   
❌ Calcular indicadores técnicos
   → Responsabilidade: crypto-signals
   
❌ Enviar notificações diretas (Telegram, Email)
   → Responsabilidade: crypto-notifications
   → Solução: Publique evento, crypto-notifications envia
   
❌ Receber webhooks do TradingView
   → Responsabilidade: crypto-webhook
   
❌ Gerenciar stops de ordens específicas
   → Responsabilidade: crypto-trader
   → Papel: APENAS monitora resultado
```

---

## 🎓 Exemplos Práticos

### ❌ ERRADO: Evasão de Fronteiras

```rust
// crypto-trader tentando gerar sinais (ERRADO!)
impl TradingService {
    async fn execute_order(&self) {
        // ❌ ERRADO: Calculando RSI aqui
        let rsi = calculate_rsi(&prices);
        if rsi < 30 {
            // ❌ ERRADO: Gerando sinal aqui
            self.buy().await;
        }
    }
}

// crypto-signals tentando executar ordem (ERRADO!)
impl SignalService {
    async fn generate_signal(&self) {
        let signal = analyze_market();
        // ❌ ERRADO: Executando ordem aqui
        exchange_client.create_order().await;
    }
}

// crypto-trader tentando enviar notificação (ERRADO!)
impl OrderService {
    async fn fill_order(&self) {
        // ❌ ERRADO: Enviando Telegram diretamente
        telegram_client.send_message("Order filled").await;
    }
}
```

### ✅ CORRETO: Respeitando Fronteiras

```rust
// crypto-signals: APENAS gera e publica sinal
impl SignalService {
    async fn generate_signal(&self) {
        let signal = analyze_market();
        // ✅ CORRETO: Publica no Kafka
        kafka_producer.send("signals.buy", signal).await;
    }
}

// crypto-trader: CONSOME sinal e executa
impl OrderService {
    async fn process_signal(&self, signal: Signal) {
        // ✅ CORRETO: Apenas executa ordem
        let order = self.create_order(signal).await;
        // ✅ CORRETO: Publica evento no Kafka
        kafka_producer.send("orders.events", order_filled).await;
    }
}

// crypto-notifications: CONSOME evento e notifica
impl NotificationService {
    async fn handle_order_event(&self, event: OrderEvent) {
        // ✅ CORRETO: Apenas formata e envia
        let message = format_message(event);
        telegram_client.send(message).await;
    }
}
```

---

## 🔍 Checklist de Implementação

Antes de implementar qualquer funcionalidade, pergunte:

### 1. Essa funcionalidade pertence a este projeto?
```
❓ Estou implementando análise técnica?
   → Se SIM e não é crypto-signals: ❌ PARE!
   
❓ Estou executando ordens em exchange?
   → Se SIM e não é crypto-trader: ❌ PARE!
   
❓ Estou enviando Telegram/Discord/Email?
   → Se SIM e não é crypto-notifications: ❌ PARE!
```

### 2. Estou duplicando lógica de outro projeto?
```
❓ Essa lógica já existe em outro projeto?
   → Se SIM: CONSUMA via Kafka, não reimplemente!
```

### 3. Estou chamando outro projeto diretamente?
```
❓ Estou fazendo HTTP request para outro microserviço?
   → ❌ ERRADO: Use Kafka
   
❓ Estou acessando o banco de outro projeto?
   → ❌ ERRADO: Use Kafka
   
❓ Estou importando código de outro projeto?
   → ❌ ERRADO: Use Kafka
```

### 4. Minha implementação conhece detalhes internos de outro projeto?
```
❓ Conheço as camadas (domain, application) de outro projeto?
   → ❌ ERRADO: Você só deve conhecer schemas Kafka
   
❓ Conheço as tabelas de banco de outro projeto?
   → ❌ ERRADO: Cada projeto tem seu próprio banco
```

---

## 🛠️ Como Corrigir Evasões de Fronteira

### Passo 1: Identifique a Violação
```
"Estou implementando X dentro do projeto Y"
→ X pertence ao projeto Y ou a outro projeto?
```

### Passo 2: Mova para o Projeto Correto
```
Se X pertence a outro projeto Z:
1. Remova implementação de X do projeto Y
2. Implemente X no projeto Z
3. Projeto Z publica evento no Kafka
4. Projeto Y consome evento via Kafka
```

### Passo 3: Defina o Contrato Kafka
```
1. Defina schema do evento em Avro/JSON Schema
2. Documente no projectmap de ambos os projetos
3. Produtor publica evento
4. Consumidor processa evento
```

---

## 📝 Template de Validação

Use este template ao implementar qualquer funcionalidade:

```markdown
## Funcionalidade: [Nome da Funcionalidade]

### Projeto Atual: [Nome do Projeto]

### Checklist de Fronteiras:
- [ ] Esta funcionalidade está listada nas "responsibilities" do projeto?
- [ ] Esta funcionalidade NÃO está na lista "O QUE NÃO FAZ"?
- [ ] NÃO estou duplicando lógica de outro projeto?
- [ ] NÃO estou chamando APIs REST de outros projetos?
- [ ] NÃO estou acessando banco de dados de outros projetos?
- [ ] Se preciso de dados de outro projeto, estou consumindo via Kafka?
- [ ] Se gero dados para outros projetos, estou publicando via Kafka?

### Dependências Externas (via Kafka):
- Consome de: [tópicos Kafka]
- Produz para: [tópicos Kafka]

### Justificativa:
[Por que esta funcionalidade pertence a este projeto específico]
```

---

## 🎯 Resumo Executivo

### Para o LLM:
```
1. LEIA o projectmap do projeto atual
2. IDENTIFIQUE as "responsibilities"
3. VERIFIQUE se a funcionalidade está nas responsibilities
4. Se NÃO estiver: NÃO IMPLEMENTE
5. Se precisar de dados: CONSUMA via Kafka
6. Se gerar dados: PUBLIQUE via Kafka
7. NUNCA chame outros projetos via HTTP/DB direto
```

### Para Humanos:
- Cada projeto é uma ilha comunicada por pontes (Kafka)
- Se você está implementando algo que parece "interessante mas não é minha responsabilidade", PARE
- Quando em dúvida, consulte a matriz de responsabilidades
- Sempre prefira consumir via Kafka do que reimplementar

---

**Última Atualização:** 2025-01-20
**Versão:** 1.1.0

