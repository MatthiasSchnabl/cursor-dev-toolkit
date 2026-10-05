# Observability, Logging & Tracing Engineering Standard

## Contents

- §1: Version Discipline
- §2: Model
- §3: What a Developer Must Be Able to Answer
- §4: tracing Is the Mechanism
- §5: Span Hierarchy
- §6: Span Names
- §7: Structured Fields
- §8: Cardinality
- §9: Request and Correlation IDs
- §10: Axum and tower-http 0.6 Order
- §11: One Instrumentation, Two Formats
- §12: Runtime Filtering
- §13: Levels
- §14: #[instrument]
- §15: Async
- §16: Errors
- §17: SQLx
- §18: External HTTP
- §19: Slow Work
- §20: State Transitions
- §21: Rig and GenAI
- §22: OpenTelemetry
- §23: Configuration
- §24: Secrets and Personal Data
- §25: Cost
- §26: Tests
- §27: Metrics
- §28: Anti-Patterns
- §29: Debugging Checklist
- §30: Definition of Done

> Navigation: use this map to load only the sections required for the current phase. Do not preload the whole file unless the task genuinely requires it.


These instructions are mandatory for logging, tracing, and observability in Rust applications in this repository.

This standard is the canonical cross-cutting source of truth for diagnostic instrumentation. It supplements:

* the Rust Engineering Standard
* the Rust Axum Engineering Standard
* the Rust SQLx + PostgreSQL Engineering Standard
* the Testing & Quality Engineering Standard
* the Security & Supply-Chain Engineering Standard
* the Rust Rig Agentic Engineering Standard

Those standards keep their own architectural rules. They do not restate this standard.

The goal is instrumentation that is:

* fast to debug during development
* correlated in production
* consistent across HTTP, database, external calls, and agent runs
* cheap to operate
* free of secrets and unnecessary sensitive data

Do not add an observability stack because a crate exists.

---

# 1. Version Discipline

This toolkit does not pin application crates.

Before applying version-specific rules, read the target project's resolved versions from `Cargo.lock` or `cargo metadata`. Check at least:

```text
tracing
tracing-subscriber
tower-http
sqlx
rig
opentelemetry
tracing-opentelemetry
```

Do not assume the newest release. Do not copy versions from another repository.

The rules below match current official documentation for `tracing` 0.1, `tower-http` 0.6 request-id middleware, and `sqlx-core` 0.8 `QueryLogger`. If the project resolves a different version, re-check that version's documentation and follow it.

Do not add telemetry crates merely because this standard names them.

---

# 2. Model

A log line is not the unit of diagnosis.

```text
Span   = context, unit of work, duration
Event  = a fact that happened inside that context
Fields = structured data on the span or event
```

Causality matters more than which thread happened to poll the future.

```text
Application / Request / Job / Agent Run
                 │
                 ▼
              Span Tree
                 │
        ┌────────┼─────────┐
        ▼        ▼         ▼
   Use Case    SQL/DB    External Call
        │                  │
        ▼                  ▼
      Events            Child Spans
```

---

# 3. What a Developer Must Be Able to Answer

For a failed critical workflow, telemetry should answer:

```text
Which request, job, or agent run?
Which use case?
Which user or tenant, when that is allowed?
Which relevant entity?
Which database, external, or tool call?
Which decision or state transition?
Where did it fail?
What is the error chain?
How long did the relevant step take?
What happened immediately before?
```

Instrument semantic units of work. Do not instrument every function, getter, or line.

---

# 4. `tracing` Is the Mechanism

Use `tracing` for structured diagnostics.

Do not build a second logging facade or an internal framework on top of `tracing`. A small helper is allowed only when it removes real duplication.

`log` crates may still emit through a `tracing` subscriber. Libraries instrument. Only the application binary installs the global subscriber.

Initialize that subscriber once, early in process startup, before meaningful work. If observability is required to operate the process, an initialization failure must not be swallowed. Do not install multiple global subscribers.

---

# 5. Span Hierarchy

Typical tree:

```text
HTTP request / worker job / agent run
    ↓
application use case
    ↓
repository / external dependency / tool
    ↓
the concrete operation, when it adds information
```

Examples:

```text
http.request
└── article.update
    ├── db.article.load
    ├── db.article.update
    └── event.publish
```

```text
agent.run
├── gen_ai.chat
├── agent.tool
│   └── db.project.lookup
└── agent.output.validate
```

The tree shows the business flow. It does not mirror the call stack.

---

# 6. Span Names

Span names are stable, low-cardinality, and name the operation.

Prefer:

```text
http.request
article.update
db.article.load
external.sap.fetch
agent.run
agent.tool
```

Do not put variable values in the name:

```text
GET /articles/123456
processing Matthias request
function_called_at_line_382
```

Ids, users, and routes with parameters belong in fields.

---

# 7. Structured Fields

Fields are the diagnostic record. The message may explain them. The message must not be the only place the data exists.

```rust
tracing::info!(
    article_id = %article_id,
    rows_affected,
    "article updated"
);
```

Prefer OpenTelemetry semantic conventions when they match the signal, including:

```text
http.request.method
http.route
http.response.status_code
db.system.name
db.operation.name
error.type
gen_ai.operation.name
gen_ai.request.model
gen_ai.usage.input_tokens
gen_ai.usage.output_tokens
```

Application correlation fields may sit beside those conventions:

```text
request_id
run_id
user_id
tenant_id
operation
entity_id
tool.name
tool.call_id
```

`request_id` is not a trace id. When OpenTelemetry export is enabled, `trace_id` and `span_id` come from the trace context. Do not overwrite one with the other.

Do not invent a private taxonomy when a semantic convention already names the fact.

---

# 8. Cardinality

High-cardinality values are valid trace and log fields:

```text
request_id
user_id
entity_id
tool_call_id
```

They are not metric labels by default.

```text
trace/log attribute  = one execution
metric dimension     = an aggregation key
```

A field that identifies one request must not become a label on a counter or histogram.

---

# 9. Request and Correlation IDs

An HTTP service has one stable request id per request.

Honor an incoming id when policy says so. Otherwise generate one.

Put it on the request span, let child spans inherit the context, and propagate it on downstream calls when the protocol has a place for it. A response header is appropriate when clients need to quote the id.

---

# 10. Axum and `tower-http` 0.6 Order

Middleware order is behavior.

`tower-http` 0.6 documents this `ServiceBuilder` order so the request id exists before `TraceLayer` builds the span, and so the id is copied onto the response before that layer records the response:

```rust
ServiceBuilder::new()
    .set_x_request_id(MakeRequestId)
    .layer(TraceLayer::new_for_http())
    .propagate_x_request_id()
```

On `ServiceBuilder`, the first layer is the outermost. `Router::layer` is the reverse: the last `.layer` is the outermost. Do not paste the `ServiceBuilder` sequence into successive `Router::layer` calls without reversing it.

`SetRequestId` does not replace an id that is already present.

Do not record `Authorization`, `Cookie`, raw query strings, bodies, or other sensitive headers on the HTTP span. Prefer `http.route` (the route template) over a raw path that contains ids.

Useful HTTP fields:

```text
http.request.method
http.route
http.response.status_code
request_id
latency, from the span
```

Handler shape, extractor policy, and status mapping stay in the Axum standard.

---

# 11. One Instrumentation, Two Formats

Development and production use the same spans and events.

Development formatting is for a person: compact or pretty output, timestamps, levels, targets where they help, span context, and error chains.

Production formatting is for a machine: JSON and/or OpenTelemetry export.

The choice is subscriber configuration, not a second logging system.

---

# 12. Runtime Filtering

Use `tracing_subscriber::EnvFilter`, or the project's existing equivalent, with `RUST_LOG`.

A developer must be able to raise detail for one target without recompiling:

```text
RUST_LOG=my_app=debug,sqlx::query=debug,tower_http=info
```

The crate name is the project's, not this example.

Require:

* a safe default level
* an environment override
* target and module filters

Do not build a profile framework when `EnvFilter` is enough. Document a few `RUST_LOG` examples for application debug, SQL debug, HTTP debug, and agent/tool debug. Add live filter reload only when operations actually need it.

`DEBUG` and `TRACE` are not production defaults.

---

# 13. Levels

```text
ERROR  The operation failed and a person should look.
WARN   Degraded or unexpected, and the operation may still have succeeded.
INFO   Lifecycle, business, or system facts worth keeping.
DEBUG  Diagnosis during development and incident analysis.
TRACE  Internal detail. Normally off.
```

An expected `NotFound`, validation failure, conflict, or `Forbidden` is not automatically `ERROR`. A missing public resource that returns 404 is not an error log.

Unexpected failures include connection loss, invariant breaks, unexpected provider failures, and serialization defects.

Do not log the same error at every layer. Propagate the typed error, add context where it is new, and log once at the boundary that can act on it.

---

# 14. `#[instrument]`

Use `#[tracing::instrument]` on relevant units:

```text
application use cases
repository operations
external integration boundaries
worker and job handlers
agent harness steps
important tool execution
```

Do not put it on every function.

Do not record every argument through `Debug`. Skip sensitive or bulky inputs and record only safe identifiers:

```rust
#[tracing::instrument(
    skip_all,
    fields(
        article_id = %article_id,
        tenant_id = %tenant_id
    )
)]
```

Never record by default:

```text
passwords
tokens
JWTs
Authorization headers
full request DTOs
customer payloads
LLM prompts
```

---

# 15. Async

`tracing` 0.1: do not hold a `Span::enter` guard across `.await`. The guard follows the future, not the poll, and attributes later work to the wrong span.

For `async fn`, use `#[instrument]`. For a future value, use `Instrument::instrument`.

---

# 16. Errors

Keep the `source()` chain. Do not collapse a `thiserror` chain into a string at the first boundary.

The external response stays safe for users. The internal diagnostic keeps the chain, plus structured fields where they exist:

```text
error.type
error.code
retryable
```

Do not classify failures by matching log text.

---

# 17. SQLx

`sqlx-core` 0.8 `QueryLogger` emits target `sqlx::query`. The event carries a short summary, `db.statement`, `rows_affected`, `rows_returned`, and elapsed time. Slow statements, using the connection's `LogSettings`, add `slow_threshold` and a slow-statement message. That event does not include bind values.

`db.statement` can still be the full SQL text. That is a problem when SQL was built with literals. Confirm the resolved SQLx version before describing argument logging. Do not enable `sqlx::query` at `TRACE` in production until that check is done.

Do not log the same SQL again from repository code. SQLx already traces the query.

Add a semantic repository span when the question is which business operation caused the queries:

```text
db.article.load
db.article.search
db.article.update
```

Those spans complement `sqlx::query`. They do not repeat it.

Slow-query thresholds belong to the connection's `log_slow_statements` settings or an explicit project threshold. Do not invent a universal millisecond cutoff.

For local debugging, raise `sqlx::query` without leaving that target at a noisy level in production.

---

# 18. External HTTP

Outbound calls are their own spans. Useful fields:

```text
peer service name
operation
http.request.method
http.response.status_code
latency
retry count
```

Do not log credentials, URLs that contain secrets, or request and response bodies by default.

---

# 19. Slow Work

Make slow database calls, external HTTP, model calls, tool calls, long use cases, and queue handling visible.

Thresholds come from the system's own latency, not from a copied constant.

---

# 20. State Transitions

Log the transitions that explain a workflow:

```text
job claimed
job started
job completed
job failed
```

Do not log every internal variable.

---

# 21. Rig and GenAI

Agent policy, budgets, and tool authorization stay in the Rig standard.

This standard requires that a production agent run can be correlated when those facts exist:

```text
run_id
agent
model
provider
turn
tool name
tool call id
tool outcome
latency
token usage
retry
finish reason
```

Prefer `gen_ai.*` semantic conventions when OpenTelemetry export is in use. Do not invent parallel names for the same fact.

A tool call is a span under the run:

```text
agent.run
└── agent.tool
    └── db.project.lookup
```

Record safe identifiers. Do not record full tool arguments by default.

Do not record by default:

```text
system prompt
user prompt
retrieved documents
tool payloads
model output
```

Content logging for local debugging must be an explicit switch, marked debug-only, and must not be the production default. Retention and redaction still apply.

---

# 22. OpenTelemetry

OpenTelemetry is not mandatory.

When the project needs distributed traces or a central backend, export from `tracing` through `tracing-opentelemetry` and OTLP. Do not build a second proprietary trace pipeline.

Libraries still must not install the exporter. The binary owns endpoint, service name, and content-capture flags.

---

# 23. Configuration

Keep telemetry configuration typed and central:

```text
log format
base level
filters
OTLP endpoint, when used
service name, when used
content telemetry switch
```

Do not scatter `std::env` reads for these through the codebase.

---

# 24. Secrets and Personal Data

The Security standard owns secret handling. This standard applies it to telemetry.

Never log by default:

```text
passwords
API keys
access tokens
refresh tokens
Authorization
Cookie
session tokens
private keys
credentials
```

Personal and sensitive business data need a diagnostic reason, a policy, and redaction or retention. Check `Debug`, `Display`, and `Serialize` on types that can hold secrets. A redacted implementation beats a convenient dump.

---

# 25. Cost

Structured telemetry is not free. In a hot loop, do not format expensive fields when the event is disabled. Do not micro-optimize cold paths.

---

# 26. Tests

Test telemetry when it is part of the contract:

```text
request id propagation
trace context propagation
redaction
required security or audit events
critical span fields
agent run correlation
```

Do not assert the exact formatted log line. Assert fields and event meaning. Formatting may change.

Do not unit-test every log line.

---

# 27. Metrics

Traces and logs describe one execution. Metrics aggregate rates, latency, and saturation.

Do not turn every trace field into a metric label. High-cardinality ids stay off metric dimensions.

---

# 28. Anti-Patterns

```text
println! or eprintln! as production logging
string-only logs
no request or run context
INFO noise
DEBUG left on in production
Span::enter held across .await
full DTO debug dumps
secrets or personal data
the same error logged at every layer
hand-copied SQL plus bind values
prompts and model output by default
ids inside span names
high-cardinality metric labels
a logging framework wrapped around tracing
a span on every function
INFO inside a hot loop
```

---

# 29. Debugging Checklist

For a production-like failure, check whether telemetry can show:

1. Which request, job, or run.
2. Which use case.
3. Which relevant ids.
4. Which child operation failed.
5. The error chain.
6. Which external, database, or tool operation.
7. The duration.
8. Retries or fallbacks.
9. The correlated events before and after.

If a critical workflow cannot answer these, the instrumentation is insufficient.

---

# 30. Definition of Done

Proportional to the feature's importance:

```text
entry span exists
important use-case spans exist
database, tool, and dependency work is correlated
request or run id exists
failures carry context
error chains survive
local filters can raise detail without a rebuild
production output can be structured
known secret and personal-data leaks are absent
async spans are not entered across .await
critical agent runs are correlated
important slow operations are visible
logs are signal, not noise
```

A small pure helper does not need this list.
