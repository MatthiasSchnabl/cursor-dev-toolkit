# cursor-dev-toolkit

Reusable Cursor developer tooling: **GBrain**, **Graphify**, **Superpowers**, **gstack**.

One-time install → available in all local Cursor projects via User-scope plugin.

## What it provides

- Global tool routing rules and skills
- Pinned runtime bootstrap (gstack, Graphify, GBrain, Superpowers checkout)
- `/tooling-doctor`, `/tooling-setup`, `/tooling-update` commands
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
