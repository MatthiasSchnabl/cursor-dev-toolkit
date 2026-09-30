# cursor-dev-toolkit

Reusable Cursor developer tooling: **GBrain**, **Graphify**, **Superpowers**, **gstack**.

One-time install → available in all local Cursor projects via User-scope plugin.

## What it provides

- Global tool routing rules and skills
- Pinned runtime bootstrap (gstack, Graphify, GBrain, Superpowers checkout)
- `/tooling-doctor`, `/tooling-setup`, `/tooling-update` commands
- `/rust-engineering-audit` — evidence-based Rust audit (`skills/engineering/rust-engineering/`; standard body must stay in sync with `rules/rust-engineering-standard.mdc`)
- `/rust-engineering-fix` — controlled remediation against an open audit in `docs/engineering-audits/rust/`
- `/rust-axum-engineering-audit` — evidence-based Axum/HTTP audit (`skills/engineering/rust-axum-engineering-audit/`; standard body in `skills/engineering/_standards/` must stay in sync with `rules/rust-axum-engineering-standard.mdc`)
- `/rust-axum-engineering-fix` — controlled remediation against an open audit in `docs/engineering-audits/axum/`
- `/testing-quality-engineering-audit` — evidence-based testing and quality audit (`skills/engineering/testing-quality-engineering-audit/`; standard body in `skills/engineering/_standards/` must stay in sync with `rules/testing-quality-engineering-standard.mdc`)
- `/testing-quality-engineering-fix` — controlled remediation against an open audit in `docs/engineering-audits/testing/`
- `/security-supply-chain-engineering-audit` — evidence-based security and supply-chain audit (`skills/engineering/security-supply-chain-engineering-audit/`; standard body in `skills/engineering/_standards/` must stay in sync with `rules/security-supply-chain-engineering-standard.mdc`)
- `/security-supply-chain-engineering-fix` — controlled remediation against an open audit in `docs/engineering-audits/security/`
- `/rust-rig-agentic-engineering-audit` — evidence-based Rig/agentic audit (`skills/engineering/rust-rig-agentic-engineering-audit/`; standard body in `skills/engineering/_standards/` must stay in sync with `rules/rust-rig-agentic-engineering-standard.mdc`)
- `/rust-rig-agentic-engineering-fix` — controlled remediation against an open audit in `docs/engineering-audits/rig/`
- `/rust-sqlx-engineering-audit` — evidence-based SQLx/PostgreSQL audit (`skills/engineering/rust-sqlx-engineering-audit/`; standard body in `skills/engineering/_standards/rust-sqlx-postgresql-engineering-standard.md` must stay in sync with `rules/rust-sqlx-engineering-standard.mdc`)
- `/rust-sqlx-engineering-fix` — controlled remediation against an open audit in `docs/engineering-audits/sqlx/`
- `/observability-logging-tracing-engineering-audit` — evidence-based observability, logging, and tracing audit (`skills/engineering/observability-logging-tracing-engineering-audit/`; standard body in `skills/engineering/_standards/observability-logging-tracing-engineering-standard.md` must stay in sync with `rules/observability-logging-tracing-engineering-standard.mdc`)
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

`install.ps1` / `install.sh` remain the path when the checkout lives somewhere else. On Windows, `install.ps1` copies the tree into `plugins\local` (Cursor does not load a plugin that only lives outside that folder). Re-run it after edits to refresh that copy.

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
