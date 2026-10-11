# cursor-dev-toolkit

> Engineering context architecture: compact Cursor rules + SessionStart anchor + progressively loaded skills/references. Run `/engineering-context-doctor` after installs/updates or whenever rules appear not to apply.

Reusable Cursor developer tooling: **GBrain**, **Graphify**, **Superpowers**, **gstack**.

One-time install → available in all local Cursor projects via User-scope plugin.

## What it provides

- Global tool routing rules and skills
- Pinned runtime bootstrap (gstack, Graphify with SQL schema support, GBrain, Superpowers checkout)
- `/tooling-doctor`, `/tooling-setup`, `/tooling-update` commands
- `/engineering-context-doctor` — verifies installed plugin freshness and engineering context architecture
- `/frontend-engineering-audit` / `/frontend-engineering-fix` — shared TypeScript frontend architecture audit/fix
- `/nextjs-engineering-audit` / `/nextjs-engineering-fix` — Next.js 16.4 App Router with strict version gate
- `/sveltekit-engineering-audit` / `/sveltekit-engineering-fix` — SvelteKit 3 / Svelte 5 with strict version gate
- `/app-knowledge-inventory` — OpenAPI-gated, evidence-based application knowledge inventory for in-app assistants
- `/app-knowledge-publish` — explicit RAGGW release-space staging only; never automatic production activation
- `/raggw-consumer-onboard` — on-demand private, pinned RAGGW consumer bootstrap with integrity checks
- `/rust-engineering-audit` — evidence-based Rust audit; full standard loads progressively from the skill reference
- `/rust-engineering-fix` — controlled remediation against an open audit in `docs/engineering-audits/rust/`
- `/rust-axum-engineering-audit` — evidence-based Axum/HTTP audit with progressive standard loading
- `/rust-axum-engineering-fix` — controlled remediation against an open audit in `docs/engineering-audits/axum/`
- `/testing-quality-engineering-audit` — evidence-based testing/quality audit with progressive standard loading
- `/testing-quality-engineering-fix` — controlled remediation against an open audit in `docs/engineering-audits/testing/`
- `/security-supply-chain-engineering-audit` — evidence-based security/supply-chain audit with progressive standard loading
- `/security-supply-chain-engineering-fix` — controlled remediation against an open audit in `docs/engineering-audits/security/`
- `/rust-rig-agentic-engineering-audit` — evidence-based Rig/agentic audit with progressive standard loading
- `/rust-rig-agentic-engineering-fix` — controlled remediation against an open audit in `docs/engineering-audits/rig/`
- `/rust-sqlx-engineering-audit` — evidence-based SQLx/PostgreSQL audit with progressive standard loading
- `/rust-sqlx-engineering-fix` — controlled remediation against an open audit in `docs/engineering-audits/sqlx/`
- `/observability-logging-tracing-engineering-audit` — evidence-based observability/logging/tracing audit with progressive standard loading
- `/observability-logging-tracing-engineering-fix` — controlled remediation against an open audit in `docs/engineering-audits/observability/`
- Cloud install/start scripts for minimal per-repo adapters

## Install (local)

### Linux / macOS / Git Bash

```bash
git clone https://github.com/MatthiasSchnabl/cursor-dev-toolkit.git
cd cursor-dev-toolkit
./install.sh
```

### Windows (PowerShell)

```powershell
git clone https://github.com/MatthiasSchnabl/cursor-dev-toolkit.git
cd cursor-dev-toolkit
.\install.ps1
```

Then: **Developer: Reload Window** → verify in **Customize** (User scope).

`install.ps1` / `install.sh` remain the path when the checkout lives somewhere else. The installers copy the plugin tree into `plugins/local` when the source checkout lives elsewhere. This is intentional: Cursor skips symlinks whose target resolves outside the local plugin directory. Re-run the installer after edits to refresh the installed copy.

## Engineering plugin

The same repository root is the Cursor plugin (`.cursor-plugin/plugin.json`). It versions engineering standards plus audit/fix skills for:

- Rust
- Axum
- SQLx/PostgreSQL
- Testing & Quality
- Security & Supply Chain
- Rig / Agentic Engineering

Lifecycle:

```text
Audit
→ persistent audit ledger
→ Fix against an open audit
→ verification
→ finding RESOLVED
```

A fix must not select an audit whose `implementation_status` is already `resolved`. Run a new audit instead.

### Install on a new machine

Clone straight into Cursor's local plugin directory so the repository root is the plugin root.

```powershell
New-Item -ItemType Directory -Force "$HOME\.cursor\plugins\local"
cd "$HOME\.cursor\plugins\local"
git clone https://github.com/MatthiasSchnabl/cursor-dev-toolkit.git
```

If that folder already exists as a non-git copy, move it aside first, then clone. Do not clone over the copy.

Then **Developer: Reload Window** (or restart Cursor). In **Customize → Plugins / Skills**, confirm `cursor-dev-toolkit` and the audit/fix commands.

### Update an installed copy

```powershell
cd "$HOME\.cursor\plugins\local\cursor-dev-toolkit"
git pull
```

Then reload Cursor.

### Same commit on both machines

```bash
git rev-parse HEAD
```

The private PC and the work laptop should print the same commit.

### Changing the plugin

```text
edit the cursor-dev-toolkit checkout
→ commit
→ push
→ git pull on the other machine
→ Cursor reload
```

Do not keep a second hand-edited plugin copy.

## RAGGW Consumer Bootstrap (on demand, private pin)

In any new consumer application, run `/raggw-consumer-onboard`. The global toolkit command loads the canonical private RAGGW consumer skill **only when needed**. No private RAGGW skill content is checked into this public toolkit or duplicated across applications.

The toolkit pins the annotated RAGGW tag `raggw-consumer-bootstrap-1` to commit `edeacb51d1d9067bcfbce8e9bda2e04bf74996bb` and verifies the skill tree and individual file hashes. Access to the private `MatthiasSchnabl/RAGGW` repository is required. Configure Git Credential Manager, or `gh auth login` followed by `gh auth setup-git`. Do not place tokens in scripts or remotes.

Manual access from Git Bash, Linux or macOS:

```bash
TOOLKIT_ROOT="${CURSOR_DEV_TOOLKIT_ROOT:-$HOME/.cursor/plugins/local/cursor-dev-toolkit}"
bash "$TOOLKIT_ROOT/scripts/raggw-consumer-bootstrap.sh" --pin
bash "$TOOLKIT_ROOT/scripts/raggw-consumer-bootstrap.sh" install
bash "$TOOLKIT_ROOT/scripts/raggw-consumer-bootstrap.sh" --check
```

The last command prints the path to the **verified private canonical** `SKILL.md`, cached under `~/.cursor/raggw-consumer-bootstrap/<commit>/`. This cache is not committed or uploaded. The loader refuses missing access, SHA/tree/hash drift and modified caches. A new RAGGW skill version requires deliberate changes to the pin and lock in the toolkit, not a moving `main` reference.

The skill prepares operator provisioning and guides REST, MCP, ingestion and retrieval validation; it does not self-provision Production or execute business actions. `/app-knowledge-inventory` and `/app-knowledge-publish` remain separate application-owned knowledge-management workflows.

## Application Knowledge Engineering (opt-in)

Every application has its own assistant (e.g. Rust Rig). RAGGW stores **versioned product/technical knowledge**, not live user data or application action authority. The toolkit is installed once globally; do **not** clone it per project.

**Hard prerequisite:** a bundled OpenAPI **JSON** 3.0/3.1 document with unique operationIds. Missing OpenAPI means immediate abort **before inventory files are written**. The current validator performs structural and local-reference checks; a pinned full OpenAPI conformance linter is still needed for production acceptance.

In any app root:

```bash
bun "$HOME/.cursor/plugins/local/cursor-dev-toolkit/scripts/app-knowledge.mjs" preflight
```

Then invoke `/app-knowledge-inventory` to create the app-owned `app-knowledge.config.json` plus `docs/app-knowledge/` artefacts. Follow [the Application Knowledge Contract](docs/application-knowledge-contract.md) and its [config example](docs/examples/app-knowledge.config.example.json). To enforce relevant staged changes before every commit, opt in with:

```bash
bun "$HOME/.cursor/plugins/local/cursor-dev-toolkit/scripts/app-knowledge.mjs" install-hook
```

The hook is local to that app, points to the globally installed plugin and refuses to overwrite existing hooks. It invokes `diff-check`: changed watched source files require a staged inventory manifest and updated docs, or a documented no-user-visible-behavior-change decision. A global rule alone cannot enforce Git commits; this local hook does.

The explicit `/app-knowledge-publish` command first creates an immutable release plan, then (only with explicit approval and runtime OAuth credentials) may **stage** the documents to an isolated pre-registered RAGGW Knowledge Space. It **never activates** production. See [RAGGW release integration](docs/application-knowledge-raggw.md).

Offline self-tests:

```bash
bun "$HOME/.cursor/plugins/local/cursor-dev-toolkit/scripts/app-knowledge.selftest.mjs"
```

## Frontend Engineering (Next.js and SvelteKit)

The global plugin now includes **three separate standards** with a shared core. No extra toolkit clone in the application repo:

- [Frontend Engineering Core](skills/engineering/_standards/frontend-engineering-core.md): TypeScript, API contracts, state, security, reliability, tests and accessibility (not visual design).
- [Next.js Standard](skills/engineering/_standards/nextjs-engineering-standard.md): **Next.js 16.4.0 App Router**, Server/Client Components, Server Functions and configuration-aware Cache Components.
- [SvelteKit Standard](skills/engineering/_standards/sveltekit-engineering-standard.md): **SvelteKit 3.0.0**, Svelte 5 runes, server/universal load, native form actions, SSR and new v3 Vite configuration.

**Read-only hard gate:** the tested, time-stamped release pins are held in [frontend-versions.json](scripts/frontend-versions.json). As of October 11, 2026 the framework pins are Next.js 16.4.0 and SvelteKit 3.0.0. Peer/runtime minimums come from manufacturer migration docs; patch pins are intentionally strict until the standard has been revalidated.

In the **frontend package root** (e.g. project/frontend in a monorepo), with Node and installed packages:

```bash
node "$HOME/.cursor/plugins/local/cursor-dev-toolkit/scripts/frontend-preflight.mjs" --root .
```

Windows PowerShell (from the frontend package root, not necessarily the Git repository root):

```powershell
node "$HOME\.cursor\plugins\local\cursor-dev-toolkit\scripts\frontend-preflight.mjs" --root .
```

The gate detects the framework from package.json and validates **actual node_modules package versions**, compatible declared semver ranges, router/config mode, runtime floors, lockfile and build script. A shared workspace-root lockfile is recognized. Without installed dependencies the result is a hard stop; ask permission to install using the project's current package manager. The gate does not contact registries, install anything or change files.

If the project differs from the tested baseline, the agent must **STOP and ask you** whether to prepare a controlled upgrade. If approved, it should assess official migration instructions, peers, version impact, tests and rollout before making changes. Never silently run interactive or non-interactive codemods. A decline means no v3/v16 standard-driven edits to the older app.

Run explicit audit/fix workflows from Cursor, never merge the two phases. Audits write source-evidenced ledgers in docs/engineering-audits/{frontend,nextjs,sveltekit}/. Fixes require an existing eligible open/partial ledger and local type, lint, test and production-build gates.

Run offline regression tests locally:

```bash
node "$HOME/.cursor/plugins/local/cursor-dev-toolkit/scripts/frontend-preflight.selftest.mjs"
```

Version evidence: [Next.js 16.4 announcement](https://nextjs.org/blog/next-16-4), [Next upgrading](https://nextjs.org/docs/app/getting-started/upgrading), [SvelteKit 3 release](https://svelte.dev/blog/sveltekit-3-is-here), [SvelteKit 3 migration](https://svelte.dev/docs/kit/migrating-to-sveltekit-3).

## Engineering context loading

The engineering setup deliberately uses three layers:

1. **Compact Cursor rules** keep critical invariants reliably attached without loading full standards.
2. **SessionStart hook** injects toolkit version/revision plus the routing duty into every new Agent session.
3. **Skills + references** use progressive disclosure. `SKILL.md` stays below 500 lines; long references expose a `## Contents` map near the top.

The full standards are no longer duplicated into auto-attached rule bodies. They remain canonical references under `skills/engineering/`.

After cloning, updating, or changing the toolkit:

```bash
./scripts/verify.sh
```

or in Cursor:

```text
/engineering-context-doctor
```

If the source checkout is outside `~/.cursor/plugins/local/cursor-dev-toolkit`, rerun the installer first; the doctor fails when the installed copy differs from the source checkout.

GBrain verification accepts both `GBRAIN_DATABASE_URL` and file-plane configuration in `~/.gbrain/config.json`; doctor JSON is parsed with Bun, so `jq` is not required.

## Verify

```bash
./scripts/verify.sh
./scripts/verify.sh --check-project-graph
```

## Update

1. Bump refs in `versions.env`
2. `./scripts/bootstrap.sh`
3. `./scripts/verify.sh`

## Uninstall

```bash
./scripts/uninstall.sh
# Windows: .\uninstall.ps1
```

Removes plugin link and gbrain launcher only. Does not delete gstack/superpowers checkouts or GBrain data.

## Cloud (per repository)

Minimal [`.cursor/environment.json`](docs/cloud-adapter.example.json):

```json
{
  "install": "git clone --depth 1 --branch v0.1.0 https://github.com/MatthiasSchnabl/cursor-dev-toolkit.git \"$HOME/.cursor-dev-toolkit\" && \"$HOME/.cursor-dev-toolkit/scripts/cloud-install.sh\"",
  "start": "\"$HOME/.cursor-dev-toolkit/scripts/cloud-start.sh\""
}
```

Set `GBRAIN_DATABASE_URL` in Cursor Dashboard → Cloud Agents → Secrets.

## Required secrets (names only)

| Secret | Required | Purpose |
|--------|----------|---------|
| `GBRAIN_DATABASE_URL` | Runtime | Shared Supabase brain |
| `GBRAIN_DIRECT_DATABASE_URL` | Optional | Sync/DDL on IPv4-only hosts |
| `OPENAI_API_KEY` | Optional | New embeddings / LLM features |

## Marketplace

**MARKETPLACE READY:** yes (MIT, no secrets, no business data). Not auto-submitted. Manual publish: [cursor.com/marketplace/publish](https://cursor.com/marketplace/publish).

## Known limitations

- User-scope plugin rules/skills are **not** loaded in Cursor Cloud VMs
- Cloud requires per-repo `environment.json` adapter (or saved environment per repo group)
- Private marketplace requires Teams/Enterprise

See [docs/cursor-constraints.md](docs/cursor-constraints.md) and [docs/adr-global-cursor-tooling.md](docs/adr-global-cursor-tooling.md).
