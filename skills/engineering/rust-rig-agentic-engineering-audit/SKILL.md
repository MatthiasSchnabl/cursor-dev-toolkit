---
name: rust-rig-agentic-engineering-audit
description: >
  Perform a repository-wide evidence-based engineering audit of agentic systems
  implemented with Rust and Rig against the canonical Rust Rig Agentic
  Engineering Standard. Evaluates agent architecture, harnesses, models, tools,
  ToolContext, hooks, run budgets, structured output, prompts, context, memory,
  retrieval/RAG, MCP, multi-agent orchestration, guardrails, observability,
  deterministic tests and agent evaluations. Produces a persistent audit
  ledger and never fixes findings.
---

# Rust Rig Agentic Engineering Audit

Perform a complete evidence-based engineering audit of agentic functionality implemented with Rust and Rig.

This skill is READ-ONLY with respect to existing production and test source.

It may create exactly one persistent audit report under:

`docs/engineering-audits/rig/`

It MUST NOT fix findings.

The canonical standard is:

`../_standards/rust-rig-agentic-engineering-standard.md`

Read that standard completely before evaluating the repository.

---

# 1. Core Contract

The workflow is:

```text
REPOSITORY DISCOVERY
        ↓
RIG VERSION / FEATURE DISCOVERY
        ↓
AGENT ARCHITECTURE DISCOVERY
        ↓
HARNESS / CAPABILITY MAPPING
        ↓
QUALITY BASELINE
        ↓
PARALLEL SPECIALIST ANALYSIS
        ↓
PARENT EVIDENCE VALIDATION
        ↓
ROOT-CAUSE CORRELATION
        ↓
PERSISTENT AUDIT
```

Never perform:

```text
AUDIT
→ FIX
```

Remediation belongs exclusively to:

```text
/rust-rig-agentic-engineering-fix
```

---

# 2. Audit Output

Create:

```text
docs/engineering-audits/rig/
RIG-AUDIT-YYYYMMDD-HHMMSS-<short-git-sha>.md
```

Example:

```text
RIG-AUDIT-20260919-214500-a92d326.md
```

The report is the persistent ledger used by the fix skill.

---

# 3. Audit Frontmatter

Every report MUST begin with:

```yaml
---
schema_version: 1
audit_id: RIG-AUDIT-20260919-214500-a92d326
domain: rust-rig-agentic
standard: rust-rig-agentic-engineering-standard
standard_version: 1
baseline_commit: <full-git-sha>
baseline_branch: <branch>
baseline_worktree: clean
rig_version: <resolved-version>
created_at: <ISO-8601>
updated_at: <ISO-8601>
audit_status: complete
implementation_status: open
---
```

Allowed:

```text
audit_status:
- complete
- incomplete

implementation_status:
- open
- partial
- resolved
- requires-reaudit
```

If no confirmed actionable findings exist:

```yaml
audit_status: complete
implementation_status: resolved
```

Do not manufacture findings to create work.

---

# 4. Baseline

Before analysis inspect:

```bash
git rev-parse HEAD
git branch --show-current
git status --short
rustc --version
cargo --version
```

Record the worktree state.

Never reset or discard user changes.

---

# 5. Determine Actual Rig Version

Before interpreting Rig APIs determine the repository's actual version and enabled features.

Inspect:

```text
Cargo.toml
workspace Cargo.toml
Cargo.lock
cargo metadata
cargo tree
```

Identify usage of:

```text
rig
rig-core
rig-agent
rig-memory
Rig vector-store companion crates
MCP / rmcp integration
provider-specific Rig features
```

Do not assume the repository uses the newest Rig release.

Do not assume API semantics from the canonical standard when the repository intentionally uses an older Rig version.

Where version-specific behavior matters, verify it against the installed dependency source or authoritative documentation.

---

# 6. Feature Discovery

Determine enabled Rig features.

Identify relevant capabilities such as:

```text
agent runtime
providers
streaming
test-utils
MCP
telemetry
memory
vector-store integrations
```

Do not report absence of an optional capability unless the application requires it.

---

# 7. Repository Instructions

Read applicable:

```text
AGENTS.md
nested AGENTS.md
.cursor/rules/
README.md
CONTRIBUTING.md
architecture documentation
ADRs
agent documentation
prompt documentation
evaluation documentation
Cargo.toml
CI workflows
```

Repository-specific decisions may specialize the standard.

Do not report a violation without considering documented intent.

---

# 8. Read-Only Rule

AUDIT MUST NOT modify:

```text
production source
test source
prompts
fixtures
eval datasets
Cargo.toml
Cargo.lock
CI configuration
agent configuration
snapshots
```

Do not run:

```text
cargo fix
cargo update
automatic prompt rewriting
automatic snapshot acceptance
automatic dependency upgrades
code generators that alter tracked source
```

The only intentional write is the audit report.

---

# 9. Agentic Architecture Discovery

Build an inventory of:

```text
agents
agent builders/factories
AgentRunner / AgentRun usage
completion models
provider clients
model registry/routing
system prompts
tools
PortableTool implementations
contextual Tool implementations
ToolContext
hooks
structured outputs
extractors
conversation memory
retrieval
vector stores
embeddings
reranking
MCP servers/tools
multi-agent relationships
agent-as-tool relationships
agent harnesses
approval mechanisms
budgets
timeouts
telemetry
evaluations
```

Do not infer architecture only from names.

Trace representative execution paths.

---

# 10. Identify Agent Entry Points

Locate where agent execution starts.

Examples:

```text
Axum handler
background job
CLI
worker
scheduled task
application service
```

Trace representative paths:

```text
entry point
→ application use case
→ agent harness
→ Rig agent/runner
→ model/tools
→ result validation
→ side effect/output
```

---

# 11. Establish Quality Baseline

Prefer repository-defined commands.

If no stronger procedure exists, use appropriate equivalents of:

```bash
cargo fmt --check
cargo check
cargo clippy -- -D warnings
cargo test
```

Run relevant agent-specific deterministic tests and existing eval commands where practical.

Record:

```text
command
scope
result
limitations
```

---

# 12. Do Not Use Live Providers Casually

Ordinary audit verification should not incur uncontrolled paid/provider calls.

Use existing deterministic tests first.

Run live-provider evaluations only when:

```text
repository explicitly defines them
credentials/environment are intentionally available
cost is understood
the result materially contributes to the audit
```

Record when live evaluation was not available.

---

# 13. Mandatory Analysis Domains

Systematically evaluate all applicable sections below.

---

# 14. Agent Necessity

For each material agent or agent workflow ask:

```text
Does this problem require model-directed choice?
Could deterministic application code solve it more reliably?
```

Do not report every agent as over-engineering.

Create a finding when agent autonomy materially replaces a deterministic workflow without providing meaningful value and creates concrete reliability/cost/risk.

---

# 15. Architecture Boundary

Verify that Rig remains an orchestration layer.

Inspect whether:

```text
domain logic
authorization
business invariants
persistence semantics
irreversible decisions
```

have leaked into prompts or model reasoning unnecessarily.

Domain/application code should not normally depend directly on Rig runtime types.

---

# 16. Dependency Direction

Look for application/domain code depending on:

```text
Agent
AgentRunner
AgentRun
Message
ToolContext
CompletionModel
provider-specific Rig types
```

when the dependency can reasonably remain inside the agent infrastructure layer.

Do not enforce artificial layering when Rig is itself the product domain.

---

# 17. Agent Construction

Determine whether agents are constructed intentionally in coherent factories/builders.

Look for ad-hoc agent construction scattered across:

```text
handlers
services
jobs
tests
```

with inconsistent:

```text
models
prompts
tools
limits
hooks
output modes
```

Create a finding when this causes behavioral inconsistency or unreviewable policy.

---

# 18. Agent Configuration as Behavior

Inventory material:

```text
system prompts
model IDs
temperature
max output
max turns
tools
retrieval
memory
hooks
output mode
retry policy
```

Determine whether these settings are reviewable and version-controlled.

---

# 19. Provider Isolation

Inspect whether provider-specific types or payloads spread through application code.

Provider-specific functionality is acceptable.

The concern is uncontrolled coupling.

Prefer isolation around:

```text
provider construction
model registry
routing
capability adapter
```

---

# 20. Model Routing

Inspect whether model selection is:

```text
explicit
observable
bounded
capability-aware
```

Look for hidden routing heuristics or uncontrolled fallback chains.

---

# 21. Provider Capabilities

Check assumptions about:

```text
tool calling
structured output
streaming
context size
reasoning support
```

against the actually configured models/providers where material.

Do not assume every Rig model backend behaves identically.

---

# 22. Fallbacks

Inspect fallback behavior.

Check for:

```text
unbounded provider fallback
recursive fallback
silent model substitution
fallback that changes capability guarantees
```

A fallback should be intentional and observable.

---

# 23. Run Bounds

Every autonomous or multi-turn agent run should have an explicit bound.

Inspect:

```text
max_turns
default_max_turns
AgentRun configuration
AgentRunner configuration
custom loops
```

Flag uncontrolled loops.

Do not rely on the prompt telling the model to stop.

---

# 24. Additional Budgets

Where risk/volume justifies it, inspect deterministic limits for:

```text
wall-clock duration
tool calls
external calls
output retries
invalid tool retries
token usage
cost
```

Rig turn limits alone may not bound every downstream resource.

Do not require every possible budget for low-risk one-shot agents.

---

# 25. Custom Agent Loops

Search for custom loops around:

```text
prompt
completion
tool execution
streaming
```

Determine whether they duplicate Rig runtime behavior.

Custom drivers require a concrete architectural reason.

Inspect stop conditions carefully.

---

# 26. Retry Amplification

Map retry layers:

```text
HTTP/provider client
Rig runtime
structured-output retries
invalid-tool retries
tool retry
application retry
```

Look for multiplicative retry behavior.

One layer should own each retry concern.

---

# 27. Timeouts

Inspect:

```text
model/provider timeout
tool timeout
MCP timeout
retrieval timeout
complete agent-run deadline
```

A long-running production agent must not be able to wait indefinitely.

---

# 28. Cancellation

Determine behavior under:

```text
request cancellation
deadline expiry
task abort
process shutdown
```

Pay particular attention to side-effecting tools.

Do not assume cancellation means a remote side effect never occurred.

---

# 29. Harness Architecture

Identify whether production agent runs are wrapped by deterministic application code controlling:

```text
run ID
trusted context
authorization
budget
deadline
model routing
approval
retry policy
telemetry
result validation
```

Do not demand a type literally named `Harness`.

Evaluate whether these responsibilities have a coherent deterministic owner.

---

# 30. Harness vs Prompt

Security/correctness controls must not exist solely as natural-language prompt instructions.

Look for prompt-only enforcement of:

```text
permissions
tenant scope
tool limits
approval
cost limits
side-effect policy
```

These are strong finding candidates.

---

# 31. Tool Inventory

Inventory every model-callable tool.

For each determine:

```text
purpose
input type
output type
side effects
authorization
trusted context
resource cost
external dependencies
error semantics
```

---

# 32. Tool Capability Design

Tools should represent useful capabilities rather than arbitrary infrastructure access.

Look for overly broad tools such as:

```text
execute_sql
run_shell
http_request_anywhere
write_any_file
generic_admin
```

unless explicitly required and strongly controlled.

---

# 33. Tool Least Privilege

Determine whether each agent receives only the tools required for its responsibility.

Look for a global tool registry exposed indiscriminately to every agent.

Do not infer safety from the system prompt.

---

# 34. Read vs Write Capabilities

Where useful, check separation between:

```text
read
create
update
delete
execute
```

An information agent should not receive destructive capabilities without reason.

---

# 35. Typed Tool Inputs

Inspect whether tool arguments are represented using meaningful Rust/Serde/schema types.

Look for excessive:

```rust
String
Value
HashMap<String, Value>
```

where enums/newtypes/structured inputs can encode important semantics.

Do not require custom types for trivial unconstrained text.

---

# 36. Tool Schemas

Evaluate whether schemas:

```text
make correct use easy
limit invalid values
use enums where appropriate
avoid huge bags of optional arguments
```

Agent tool ergonomics are part of reliability.

---

# 37. Tool Descriptions

Review important descriptions for:

```text
what the tool does
when to use it
when not to use it
side effects
important constraints
```

Do not create stylistic findings for wording preferences.

Create findings when ambiguous descriptions plausibly cause incorrect tool selection or unsafe use.

---

# 38. Portable vs Contextual Tools

Determine whether context-free tools use appropriately simple contracts.

Determine whether tools requiring trusted runtime state use contextual execution appropriately.

Do not require contextual tools when normal explicit application inputs are sufficient.

---

# 39. ToolContext

Inspect use of `ToolContext` where present.

Evaluate whether trusted host-only data such as:

```text
UserId
TenantId
AuthorizationContext
RequestId
ExecutionPolicy
```

is supplied by deterministic host code rather than model arguments.

Do not require ToolContext specifically if another equally strong host-controlled pattern is used.

---

# 40. Model-Controlled Identity

Flag cases where the model supplies authoritative:

```text
user ID
tenant ID
role
permission
credential
authorization scope
```

that host code could supply from trusted context.

This is a high-value security/correctness check.

---

# 41. Tool Authorization

Privileged tools must enforce authorization independently of model obedience.

Trace representative write/privileged tools.

Check:

```text
identity
tenant/resource scope
permission
policy
```

before side effects.

---

# 42. Tool Side-Effect Classification

Determine whether the architecture distinguishes at least conceptually between:

```text
read-only
reversible mutation
high-impact / irreversible action
```

Do not require a particular enum.

The system must understand which tools create external consequences.

---

# 43. High-Impact Approval

For high-impact operations inspect whether:

```text
deterministic policy
human approval where required
specific operation parameters
```

exist outside the model.

The model must not approve itself.

---

# 44. Approval Timing

Approval should apply to the concrete operation that will execute.

Look for flows where approval occurs before material parameters are known and can subsequently change.

---

# 45. Tool Idempotency

For side-effecting tools subject to retry/duplicate invocation, inspect:

```text
idempotency
unique operation IDs
deduplication
optimistic concurrency
```

Do not require idempotency where duplicate invocation is impossible or harmless.

---

# 46. Tool Output Size

Inspect tool results for unbounded context injection.

Examples:

```text
complete database tables
large documents
full HTTP bodies
massive search results
```

Tool output should be bounded and task-relevant.

---

# 47. Tool Output Structure

Check whether follow-up-relevant outputs contain stable identifiers.

Do not force the model to infer authoritative IDs from prose.

---

# 48. Tool Errors

Inspect whether tool failures are typed and semantically useful.

Look for:

```text
everything converted to String
internal error leakage
authorization refusal treated as generic failure
temporary failure indistinguishable from permanent failure
```

---

# 49. Invalid Tool Calls

Inspect retry/repair behavior for malformed calls.

Retries must be bounded.

Do not allow repair logic to reinterpret an ambiguous request into a different privileged action.

---

# 50. Hooks

Inventory Rig hooks.

For each determine:

```text
purpose
ordering
policy effect
model routing effect
tool rewriting
retry/recovery effect
```

Treat hooks like middleware.

---

# 51. Hook Complexity

Look for too many cross-dependent hooks with hidden behavior.

Hooks should primarily implement cross-cutting runtime policy/observation, not ordinary business workflows.

---

# 52. Hooks as Guardrails

Where hooks enforce guardrails, verify the relevant action cannot bypass them through an alternate execution path.

Prompt-level guidance does not replace deterministic hook/application policy.

---

# 53. Structured Output

Inventory programmatically consumed model outputs.

Determine whether typed structured output is used where appropriate.

Look for fragile prose parsing:

```text
regex
split markers
substring extraction
```

where a schema should define the contract.

---

# 54. OutputMode Semantics

Where Rig `OutputMode` is used, inspect whether the implementation understands the selected mode's guarantees.

Do not treat:

```text
Native
Tool
Prompted
Auto
```

as semantically interchangeable.

Verify against the actual Rig version.

---

# 55. Output Validation

Even schema-valid output must pass domain validation.

Trace conversion:

```text
model DTO
→ validated application/domain value
```

Do not allow structurally valid model data to bypass domain invariants.

---

# 56. Output Retry Bounds

Inspect structured-output correction/retry loops.

They must terminate deterministically.

Repeated invalid output should become an explicit failure.

---

# 57. Extractors

Where the operation is fundamentally structured extraction, evaluate whether a simpler extractor/structured call would be preferable to a general autonomous agent.

Do not prescribe simplification without concrete reliability/complexity benefit.

---

# 58. Prompt Architecture

Inventory material system prompts.

Determine whether prompts are:

```text
version controlled
focused
reviewable
testable
separated from external data
```

Large prompts are not automatically defects.

Report when prompts contain deterministic policy better enforced in code or become unmaintainable behavioral monoliths.

---

# 59. Instruction Trust

Inspect whether the system distinguishes:

```text
trusted host instructions
user input
retrieved documents
tool results
external content
```

External content must not silently become trusted instruction.

---

# 60. Prompt Injection

Trace agents consuming:

```text
web content
documents
emails
tickets
retrieval results
MCP data
tool outputs
```

Check whether sensitive tools or capabilities remain deterministically protected even when content contains malicious instructions.

Do not claim prompt injection is eliminated merely by adding a warning to the system prompt.

---

# 61. Context Budget

Determine whether agents have an intentional context strategy.

Inspect:

```text
static context
conversation history
retrieved context
tool definitions
tool outputs
```

Look for unbounded context growth.

---

# 62. Static Context

Large static context is repeatedly sent.

Identify cases where large knowledge bodies should be retrieved dynamically instead.

Do not optimize small prompts unnecessarily.

---

# 63. Retrieval Architecture

Where RAG is used map:

```text
document ingestion
chunking
embedding
storage
search
filtering
reranking
context assembly
```

Do not audit only the final agent call.

---

# 64. Vector Store Choice

Determine whether vector-store complexity matches requirements.

Do not report an external vector DB merely because a simpler store might exist.

Require evidence of unjustified operational complexity or unsuitable semantics.

---

# 65. Embedding Model Identity

Inspect whether embedding model/configuration is explicit enough to preserve vector compatibility.

Changing embedding models may be a data migration.

Look for silent changes that could mix incompatible embeddings.

---

# 66. Retrieval Grounding

For tasks requiring authoritative evidence, inspect behavior when retrieval:

```text
returns nothing
fails
returns weak matches
```

Do not allow the agent to silently behave as fully grounded when authoritative retrieval failed.

---

# 67. Retrieval Evaluation

Determine whether retrieval quality is tested separately from final answer quality where RAG is material.

Useful questions:

```text
Was relevant evidence retrieved?
Was required evidence missed?
Was irrelevant context over-retrieved?
```

---

# 68. Memory Architecture

Inventory conversation/long-term memory.

Distinguish:

```text
chat history
application state
domain state
long-term semantic memory
retrieval corpus
```

These must not be conflated.

---

# 69. In-Memory Memory

If process-local memory is used in production, determine whether loss on:

```text
restart
horizontal scaling
instance switch
```

is acceptable.

Do not report in-memory memory when conversations are intentionally ephemeral.

---

# 70. Memory Bounds

Check for explicit:

```text
message windows
token budgets
compaction
demotion
```

or equivalent bounded behavior.

Unbounded lifetime history is a finding candidate.

---

# 71. Memory Isolation

Trace memory keys/scopes.

Verify isolation by:

```text
user
tenant
conversation
agent
```

as required.

Cross-principal memory leakage is a severe defect.

---

# 72. Memory Compaction

Where summaries replace older context, inspect whether derived summaries are incorrectly treated as authoritative source-of-truth data.

Memory summaries may support reasoning.

They should not replace required canonical records.

---

# 73. Memory Poisoning

Inspect what content can become durable memory.

Look for automatic promotion of:

```text
model claims
retrieved hostile instructions
unverified user assertions
```

into trusted long-term state.

---

# 74. Ground-Truth Verification

For workflows where correctness matters, determine whether the agent can verify relevant facts through deterministic tools/systems.

Look for long autonomous reasoning chains operating only on prior model statements when authoritative verification is available.

---

# 75. MCP

Where MCP is used, inventory:

```text
servers
credentials
discovered tools
allowed tools
read/write capabilities
```

MCP is a capability transport, not a trust boundary.

---

# 76. Dynamic MCP Discovery

Inspect whether newly advertised MCP tools automatically become available to the model.

Capability expansion should be intentional for privileged systems.

---

# 77. MCP Least Privilege

Check whether MCP credentials/capabilities match agent responsibility.

Read-only agents should not normally connect using identities with destructive permissions.

---

# 78. Multi-Agent Architecture

Inventory:

```text
subagents
agents-as-tools
delegation
parallel agents
supervisor patterns
```

For each ask whether separation provides concrete value:

```text
context isolation
specialization
permission isolation
parallel independent work
model specialization
```

Do not reward complexity itself.

---

# 79. Agent Responsibilities

Prefer concrete responsibilities over vague personas.

Inspect whether subagents have bounded tasks and capability sets.

---

# 80. Nested Agent Bounds

Agent-to-agent delegation must remain bounded.

Inspect:

```text
depth
turn count
tool count
recursive delegation
```

Do not allow accidental recursive agent graphs.

---

# 81. Subagent Trust

Subagent output is still model output.

Check whether privileged consequences require deterministic validation before execution.

One model must not become the authorization source for another.

---

# 82. Parallel Agent Mutation

Identify agents running concurrently against mutable shared state.

Determine whether conflicts, duplicate work or race conditions are controlled.

---

# 83. Observability

Determine whether production agent runs can be reconstructed structurally.

Useful signals include:

```text
run ID
agent
provider/model
turn
tool name
tool call ID
tool status
token usage
latency
retry
finish reason
error category
approval
```

Do not require storage of full sensitive content.

---

# 84. Run Identity

Production material agent executions should have stable correlation IDs.

Trace whether:

```text
model calls
tool calls
retrieval
memory
final result
```

can be associated with the same run.

---

# 85. Content Telemetry

Inspect whether prompts/tool results/model outputs are recorded.

Full content telemetry is sensitive.

Determine whether it is:

```text
disabled by default
redacted
explicitly approved
retained appropriately
```

when used.

---

# 86. Usage / Cost Observability

Where agents have meaningful operational cost, inspect whether the system can observe:

```text
tokens
turns
tool calls
latency
provider usage
```

Do not demand monetary cost calculation when provider pricing data is unavailable.

---

# 87. Error Classification

Inspect whether failures distinguish useful categories such as:

```text
provider failure
timeout
budget exhaustion
invalid structured output
invalid tool call
tool refusal
tool failure
retrieval failure
memory failure
authorization failure
approval required
```

A single free-form error loses important operational semantics.

---

# 88. Graceful Failure

Determine whether the agent can explicitly return:

```text
cannot complete
partial result
missing evidence
dependency unavailable
approval required
```

rather than fabricating completion.

---

# 89. Deterministic Rig Tests

Inspect whether agent-loop behavior uses deterministic Rig test facilities where appropriate.

Examples:

```text
scripted completion turns
tool call sequences
turn-budget behavior
invalid-tool recovery
output validation
hooks
memory failure
```

Do not require mock models for every trivial function.

---

# 90. MockCompletionModel

Where agent runtime behavior is tested, inspect whether scripted model responses can prove exact model-call progression.

Unexpected extra model calls should ideally be observable/failing rather than silently accepted.

---

# 91. Tool Unit Tests

Every material deterministic tool should be directly testable without invoking an LLM.

Inspect tests for:

```text
valid input
invalid input
authorization
side effects
errors
idempotency
```

according to tool semantics.

---

# 92. ToolContext Tests

Context-dependent tools should test relevant:

```text
required context
missing context
tenant/user isolation
authorization
```

---

# 93. Hook Tests

Hooks implementing deterministic policy should have direct tests.

Do not rely solely on live-model behavior to prove guardrails.

---

# 94. Harness Tests

Inspect deterministic tests for:

```text
turn budgets
timeouts
tool budgets
approval
trusted context
routing
fallbacks
error mapping
```

according to implemented capabilities.

---

# 95. Memory Tests

Where memory matters inspect tests for:

```text
isolation
windowing
token budgets
compaction
load failure
persist failure
```

---

# 96. Retrieval Tests

Where RAG matters inspect separate tests for:

```text
retrieval
filtering
ranking
context selection
```

Do not rely solely on final answer evaluation.

---

# 97. Agent Evals

Every material production agent should have an evaluation strategy proportional to risk.

Identify:

```text
evaluation datasets
regression cases
live or recorded model evals
semantic graders
task outcome checks
```

Do not require elaborate eval infrastructure for experimental/non-production prototypes.

---

# 98. Eval Dataset Quality

Inspect whether evals contain:

```text
realistic tasks
known failures
edge cases
tool-selection cases
ambiguous inputs
adversarial inputs where relevant
```

A happy-path demo set is not a robust evaluation corpus.

---

# 99. Held-Out Evals

If prompts/tools are actively tuned against a dataset, determine whether held-out cases exist where overfitting risk is meaningful.

---

# 100. Outcome-Based Evaluation

Prefer evaluating successful task outcome rather than requiring one exact tool/reasoning sequence.

Process metrics remain useful secondary evidence.

---

# 101. Eval Metrics

Relevant metrics may include:

```text
task success
tool correctness
invalid tool rate
forbidden action rate
turn count
tool count
token usage
latency
retries
```

Do not create findings solely because one metric is not collected.

Tie metrics to actual engineering needs.

---

# 102. Prompt / Tool / Model Regression

Determine whether meaningful changes to:

```text
system prompts
tool names/descriptions
tool schemas
models
routing
```

are evaluated before deployment.

Compilation cannot detect behavioral regressions in these artifacts.

---

# 103. Model Upgrades

Inspect whether model upgrades are treated as behavioral changes.

Look for evidence of comparative evaluation where model behavior is production-critical.

---

# 104. Production Failure Corpus

Determine whether reproducible agent failures become regression/eval cases.

This is especially important for:

```text
incorrect tool use
hallucinated completion
prompt injection failures
structured output failures
cost explosions
```

---

# 105. Security Evals

For privileged agents inspect adversarial coverage such as:

```text
prompt injection
cross-tenant attempts
forbidden tool use
data exfiltration attempts
malicious retrieved content
high-cost loops
```

Deep security analysis remains owned by the Security skill.

This audit asks whether agent-specific guardrails are evaluated.

---

# 106. Rate / Concurrency Limits

Agent requests can amplify into many provider/tool calls.

Inspect limits protecting:

```text
provider quota
database
vector store
external APIs
CPU
```

Do not assume Tokio task capacity equals downstream capacity.

---

# 107. Generic Execution Tools

Treat generic:

```text
shell
SQL
filesystem mutation
arbitrary HTTP
code execution
```

tools as high-risk.

Inspect sandbox/policy/approval architecture when they exist.

---

# 108. Streaming

Where streaming is used inspect whether it preserves:

```text
budgets
guardrails
tool lifecycle
error handling
usage accounting
cancellation
```

Also determine whether unvalidated final content is streamed where final validation is required.

---

# 109. Candidate Finding Rule

A suspicious pattern is not a finding.

Before reporting:

1. inspect implementation
2. inspect agent construction
3. inspect harness
4. inspect tools
5. inspect tests/evals
6. inspect relevant configuration
7. verify actual Rig version semantics
8. determine concrete consequence

Only then create a confirmed finding.

---

# 110. Parallel Subagents

For repository-wide audits use up to three specialist subagents.

They MUST NOT modify files.

They return candidate findings only.

---

# 111. Subagent A — Architecture, Harness & Runtime

Analyze:

```text
agent necessity
architecture boundaries
agent construction
model/provider routing
AgentRun/AgentRunner
turn/retry/time budgets
harness
hooks
failure handling
streaming
observability
```

Return candidate findings with evidence.

---

# 112. Subagent B — Tools, Guardrails & Capabilities

Analyze:

```text
tool inventory
typed schemas
ToolContext
authorization
least privilege
side effects
approvals
idempotency
MCP
multi-agent permissions
generic execution tools
prompt injection boundaries
```

Return candidate findings only.

---

# 113. Subagent C — Context, Memory, RAG, Testing & Evals

Analyze:

```text
prompts
context budgets
memory
retrieval
vector stores
embeddings
grounding
deterministic Rig tests
agent evals
regression corpus
model/tool/prompt change evaluation
```

Return candidate findings only.

---

# 114. Parent Validation

Subagent output is NOT authoritative.

The parent MUST validate every candidate against repository evidence.

Reject false positives.

Merge symptoms sharing one root cause.

---

# 115. Finding IDs

Use:

```text
RIG-001
RIG-002
RIG-003
...
```

IDs remain stable throughout remediation.

---

# 116. Finding Status

Initial status:

```text
OPEN
```

Allowed lifecycle:

```text
OPEN
IN_PROGRESS
RESOLVED
NOT_APPLICABLE
ACCEPTED_RISK
BLOCKED
STALE
```

The audit MUST NOT autonomously set `ACCEPTED_RISK`.

---

# 117. Severity

Use:

```text
CRITICAL
HIGH
MEDIUM
LOW
INFO
```

Severity must reflect concrete effect rather than conceptual impurity.

---

# 118. CRITICAL

Examples may include:

```text
unbounded autonomous privileged execution
credible cross-tenant agent capability
model can directly execute catastrophic operations without deterministic authorization
generic remote code execution capability exposed to untrusted model input without containment
```

Use sparingly.

---

# 119. HIGH

Examples may include:

```text
privileged tool relies only on prompt-based authorization
unbounded agent loop with external side effects
high-impact action without deterministic approval
memory isolation failure
agent can select authoritative tenant/user identity
systemic prompt-injection path to privileged capability
```

---

# 120. MEDIUM

Examples may include:

```text
material agent lacks bounded retries/timeouts
structured outputs trusted without semantic validation
RAG grounding silently degrades
large unbounded tool outputs
production agent has no meaningful eval coverage
model/tool changes cannot be regression evaluated
```

---

# 121. LOW

Examples may include:

```text
localized tool-schema ambiguity
minor observability gap
unnecessary agent abstraction
non-critical context inefficiency
```

Do not elevate style preferences.

---

# 122. Confidence

Every finding MUST have:

```text
HIGH
MEDIUM
LOW
```

Use `Needs Investigation` instead of a weakly supported confirmed finding.

---

# 123. Finding Structure

Use:

```markdown
## RIG-001 — Short precise title

**Status:** OPEN
**Severity:** HIGH
**Confidence:** HIGH
**Category:** <architecture / tools / harness / memory / evals / ...>
**Standard:** §<section> <rule>
**Locations:**
- `src/...`
- `tests/...`

### Evidence

Concrete repository behavior.

### Agentic Failure Scenario

Describe how the issue manifests in an agent run.

### Impact

Describe correctness, security, reliability, cost, latency or maintainability impact.

### Existing Controls

Controls already present, if relevant.

### Why this violates the standard

Explain the violated agentic engineering invariant.

### Required remediation

Describe the required outcome without over-prescribing implementation.

### Verification

Define deterministic tests and/or eval evidence required.

### Resolution

Not yet implemented.
```

---

# 124. Root-Cause Deduplication

Do not create separate findings for every agent affected by one shared harness defect.

Example:

```text
all agents use prompt-only authorization
```

is likely one systemic root-cause finding.

---

# 125. Cross-Domain Correlation

A Rig finding may involve:

```text
Axum
application
Security
Testing
SQLx
```

but this audit should report the agentic root cause.

Do not duplicate deep findings owned by other standards unless they directly affect agent architecture.

---

# 126. Needs Investigation

Use for concerns requiring unavailable evidence.

Examples:

```text
live-model reliability unknown
actual provider cost unavailable
production context size unknown
RAG quality requires representative corpus
fallback provider behavior not testable
```

Define a concrete investigation plan.

---

# 127. Verified Strengths

Record controls actually verified.

Examples:

```text
all production runs have bounded max_turns
trusted tenant context remains host-only
write tools enforce authorization independently
high-impact tools require approval
structured outputs receive domain validation
conversation memory is tenant/conversation isolated
deterministic MockCompletionModel tests cover run loops
production agents have representative eval suites
```

Do not add generic praise.

---

# 128. Audit Report Structure

Produce:

```markdown
# Rust Rig Agentic Engineering Audit

## Executive Summary

## Repository Baseline

## Rig Version & Feature Baseline

## Agent Architecture Overview

## Harness & Capability Model

## Verification Results

## Finding Summary

## Critical Findings

## High Findings

## Medium Findings

## Low Findings

## Verified Strengths

## Needs Investigation

## Audit Limitations

## Recommended Remediation Order
```

---

# 129. Summary Table

Use:

```markdown
| ID | Severity | Confidence | Status | Category | Title |
|---|---|---|---|---|---|
| RIG-001 | HIGH | HIGH | OPEN | Tools | ... |
```

---

# 130. Remediation Order

Normally prioritize:

```text
unbounded/privileged autonomy
→ authorization / capability isolation
→ irreversible side effects
→ run budgets / failure safety
→ structured correctness
→ memory / retrieval correctness
→ eval/regression capability
→ observability
→ cost/context efficiency
→ maintainability
```

Respect architectural dependencies.

---

# 131. Audit Completeness

Audit is complete only when:

1. canonical Rig standard was read
2. actual Rig version/features were established
3. repository instructions were read
4. agent architecture was mapped
5. harness/capability architecture was mapped
6. representative execution paths were traced
7. applicable quality gates were run
8. tools and privileged capabilities were reviewed
9. prompts/context/memory/RAG were reviewed where used
10. testing/eval architecture was reviewed
11. specialist subagents were used where useful
12. candidates were independently validated
13. duplicate symptoms were correlated
14. verified strengths were recorded
15. uncertainty was separated from confirmed findings
16. report was persisted
17. existing source/tests/configuration were not modified

---

# 132. Final Rule

The audit must answer:

```text
Is the agentic system bounded, deterministic where it should be,
probabilistic only where that adds value, least-privileged,
observable, testable and evidence-backed?
```

The audit does not improve the system.

It establishes the verified implementation backlog.

Remediation belongs exclusively to:

```text
/rust-rig-agentic-engineering-fix
```
