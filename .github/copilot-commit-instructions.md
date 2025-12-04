# Git Commit Instructions - Core Banking Ledger

## Commit Message Format

Use the **Conventional Commits** format with project-specific prefixes:

```
<type>(<scope>): <subject>

[optional body]

[optional footer]
```

## Types (Commit Types)

### Core Types
- **feat**: New functionality or feature
- **fix**: Bug fix
- **perf**: Performance improvement
- **refactor**: Code refactoring without changing behavior
- **test**: Addition or modification of tests
- **docs**: Documentation changes
- **chore**: Maintenance tasks, build, CI/CD
- **style**: Formatting, imports, etc (no logic change)

### Domain-Specific Types
- **domain**: Changes in domain layer (entities, value objects)
- **infra**: Changes in infrastructure layer (database, kafka, http)
- **migration**: Changes related to wallet migration
- **adr**: Creation or update of Architecture Decision Records

## Scopes (Contexts)

### Layer Scopes
- **domain**: Domain layer
- **application**: Application layer
- **infrastructure**: Infrastructure layer
- **cmd**: Entrypoints (API or worker)

### Module Scopes
- **account**: Account aggregate
- **transaction**: Transaction aggregate
- **balance**: Balance operations
- **kafka**: Kafka producer/consumer
- **database**: Database operations
- **repository**: Repository implementations
- **handler**: HTTP handlers
- **service**: Application services
- **worker**: Background worker
- **api**: API server

### Technical Scopes
- **test**: Testing utilities
- **benchmark**: Performance benchmarks
- **ci**: CI/CD pipeline
- **docker**: Docker and docker-compose
- **k8s**: Kubernetes deployments
- **monitoring**: Observability (logs, metrics, traces)
- **migration**: Database migrations (Flyway)

## Subject (Title)

### Rules
- Use **imperative** mood: "add" not "added" or "adding"
- First letter **lowercase**
- Maximum **72 characters**
- No period at the end
- Be specific and descriptive

### Examples
```
✅ feat(transaction): add transaction size limit validation
✅ fix(balance): fix race condition in balance updates
✅ perf(repository): optimize batch account lookup query
✅ refactor(domain): simplify authorization service
✅ test(transaction): add tests for unbalanced transactions
```

## Body (Body)

### When to Include
- Explain **what** changed and **why** (not how - that's in the code)
- Context for technical decisions
- Breaking changes
- References to ADRs or issues

### Format
- Blank line after subject
- Wrap at **72 characters**
- Use lists with `-` or `*`

### Example
```
feat(kafka): add retry logic for event publishing

Implements exponential backoff for failures when publishing
transaction events to Kafka. This reduces event loss during
temporary cluster instabilities.

- Retry up to 3 times with 2^n seconds backoff
- Log each attempt with correlation ID
- Metrics for retry counter

Refs: ADR-0005 (Transaction Events)
```

## Footer (Footer)

### Breaking Changes
```
BREAKING CHANGE: remove deprecated `wallet_id` field from API

The wallet_id field has been completely removed from the balance
API response. Use account_id instead.

Migration: update clients to use account_id
```

### References
```
Refs: #123, #456
Closes: #789
Fixes: JIRA-1234
See also: ADR-0007
```

## Commit Message Examples

### Feature Addition
```
feat(transaction): implement batch transaction processing

Adds support for processing multiple transactions in batch,
optimizing balance updates to reduce lock contention.

- Group updates by account_id
- Use sorted updates to prevent deadlocks
- Implement update_multi_debit_balance function in PostgreSQL

Performance: reduces processing time by 40% for batches
with 100+ transactions.

Refs: ADR-0001 (Balances Table Strategy)
```

### Bug Fix
```
fix(balance): fix optimistic locking in concurrent updates

Adds retry logic when version conflict is detected during
balance updates. Previously returned 500 error to client.

- Retry up to 3 times with jitter
- Log conflicts for monitoring
- Return 409 Conflict if all attempts fail

Fixes: #456
```

### Performance Improvement
```
perf(repository): optimize FindAccountsByIDs with IN clause

Replaces multiple individual queries with a single query using
IN clause. Reduces latency from 50ms to 5ms in batches of 20 accounts.

Benchmark results:
- FindAccountsByIDs/10_accounts: 2.5ms → 1.2ms
- FindAccountsByIDs/100_accounts: 45ms → 8ms

Refs: #789
```

### Refactoring
```
refactor(domain): extract authorization rules to dedicated service

Moves authorization logic from Transaction to AuthorizationService,
improving testability and separation of concerns.

- Creates AuthorizationService with specific methods
- Removes validation methods from Transaction
- Updates tests to test service in isolation

No behavior changes.
```

### Test Addition
```
test(transaction): add integration tests for transaction flow

Implements integration tests using testcontainers to validate
complete transaction processing flow.

Tested scenarios:
- Balanced transaction with 2 operations
- Transaction with insufficient balance
- Transaction with unbalanced operations
- Transaction processing idempotency

Coverage: domain/transaction 78% → 92%
```

### Database Migration
```
migration: add index on balances.updated_at

Creates BTREE index on balances.updated_at to optimize queries
for balance change history.

Migration: V020__index_balances_updated_at.sql

Impact: improves audit trail query performance by 10x
```

### Documentation
```
docs(adr): document sorted updates strategy

Creates ADR-0001 documenting decision to use sorted updates to
prevent deadlocks in batch balance updates.

Trade-offs:
- PRO: eliminates deadlocks in concurrent transactions
- PRO: better throughput under high concurrency
- CON: additional complexity in application code
```

### Infrastructure
```
infra(kafka): configure consumer group for transaction events

Adds Kafka consumer configuration in worker to process
transaction events published by API.

- Consumer group: ledger-transaction-processor
- Topic: wallet.operation.event
- Auto-commit: false (manual commit after processing)
- Max poll records: 100

Refs: ADR-0005
```

### Chore/Maintenance
```
chore(deps): update security dependencies

Updates packages with reported vulnerabilities:
- github.com/gin-gonic/gin v1.7.0 → v1.8.1
- golang.org/x/crypto v0.0.0 → v0.1.0

CVE fixes: CVE-2022-1234, CVE-2022-5678
```

## Multi-Module Commits

For commits affecting multiple modules, use general scope:

```
feat(ledger): implement complete transaction processing flow

Implements end-to-end transaction processing including:
- Domain: Transaction and Operation entities
- Application: TransactionService with authorization
- Infrastructure: TransactionRepository with PostgreSQL
- API: POST /transactions endpoint with validation

Complete integration tested with integration tests.

Refs: #123
```

## Commit Frequency

### When to Commit
- ✅ After completing a logical unit of work
- ✅ Before making significant changes
- ✅ When all tests pass
- ✅ After code review fixes (squash if appropriate)

### Avoid
- ❌ Very large commits with multiple features
- ❌ Commits that break build or tests
- ❌ "WIP" commits on main branch
- ❌ "Fix typo" after "Add feature" (squash these commits)

## Special Cases

### Breaking Changes
```
feat(api): remove legacy wallet endpoint support

BREAKING CHANGE: legacy wallet endpoints have been removed

Removed endpoints:
- GET /wallet/{id}/balance
- POST /wallet/{id}/credit
- POST /wallet/{id}/debit

Migration: use new ledger endpoints:
- GET /organizations/{org}/balance
- POST /organizations/{org}/transactions

Refs: ADR-0007, #migration-timeline
```

### Security Fixes
```
fix(security): fix SQL injection in account query

Replaces string concatenation with parameterized query in
FindAccountsByAlias. Vulnerability reported in security audit.

Severity: HIGH
Affected versions: < v2.5.0

CVE: pending assignment
```

### Hotfixes
```
fix(critical): fix data loss in concurrent balance updates

Adds missing transaction isolation level causing race condition
in balance updates under high concurrency.

Impact: critical - can cause data loss
Priority: P0

Rollout: immediate hotfix to production

Refs: INCIDENT-2025-001
```

## Tips

### Good Practices
- Atomic commits: one logical change per commit
- Subject describes "what", body describes "why"
- Use imperative mood ("add" not "added")
- Reference issues and ADRs when relevant
- Include performance numbers in perf commits
- Include migration impact in breaking changes

### Examples from Real Work
```
✅ feat(balance): implement optimistic locking with version field
✅ fix(worker): fix graceful shutdown losing kafka messages
✅ perf(sort): optimize account ID sorting from O(n²) to O(n log n)
✅ test(integration): add testcontainers for postgres tests
✅ docs(readme): update API examples with new endpoints
✅ refactor(repository): extract common query logic to base repository
✅ chore(ci): add golangci-lint to pipeline
```

```
❌ fix: bug
❌ update code
❌ improvements
❌ WIP
❌ small changes
❌ review fixes
```

## Validation

Before committing, verify:
- [ ] Message follows conventional commits format
- [ ] Correct type and scope
- [ ] Subject in imperative, lowercase, < 72 chars
- [ ] Body explains "why" when necessary
- [ ] Issue/ADR references included
- [ ] Breaking changes documented
- [ ] Tests pass (`make test`)
- [ ] Lint passes (`make lint`)
- [ ] Code formatted (`gofmt`)

