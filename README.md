# cursor-dev-toolkit

> Engineering context architecture: compact Cursor rules + SessionStart anchor + progressively loaded skills/references. Run `/engineering-context-doctor` after installs/updates or whenever rules appear not to apply.

Reusable Cursor developer tooling: **GBrain**, **Graphify**, **Superpowers**, **gstack**.

One-time install → available in all local Cursor projects via User-scope plugin.

## What it provides

- Global tool routing rules and skills
- Pinned runtime bootstrap (gstack, Graphify with SQL schema support, GBrain, Superpowers checkout)
- `/tooling-doctor`, `/tooling-setup`, `/tooling-update` commands
- `/engineering-context-doctor` — verifies installed plugin freshness and engineering context architecture
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
