# ADR: Global Cursor Developer Tooling

## Problem

Per-repository copies of setup scripts, version pins, skills, and routing rules do not scale across projects.

## Options evaluated

| Option | Pros | Cons |
|--------|------|------|
| A. Repo-local setup | Simple, proven | Duplicated per project |
| B. User-scope Cursor Plugin | One install, cross-repo rules/skills | Cloud does not load user plugins |
| C. Public Marketplace | One-click install | Requires public OSS + Cursor review |
| D. Personal saved Cloud Environment | Central runtime | Per-repo/repo-group scoped |
| E. Multi-repo Cloud Environment | Shared build | Over-clones, slow, broad attack surface |
| F. Minimal repo adapter | Thin cloud bootstrap | Still one file per repo |

## Decision

**Local:** Option B — `cursor-dev-toolkit` as User-scope Cursor Plugin at `~/.cursor/plugins/local/cursor-dev-toolkit`.

**Cloud:** Option F — minimal `.cursor/environment.json` per repo calling pinned toolkit `scripts/cloud-install.sh` and `scripts/cloud-start.sh`.

Superpowers remains a separate upstream plugin (not vendored). No invented plugin dependencies.

## Why

Cursor docs confirm user-scope plugins work locally across repos. Cloud agents run repo hooks only; user-level `~/.cursor/` is unavailable in VMs. Build `install` persists disk state for runtime CLIs.

## Fallback

Keep legacy scripts under `scripts/legacy/` in migrated repos until cloud verification completes.

## Migration path

1. Install toolkit globally (`install.sh` / `install.ps1`)
2. Slim project `AGENTS.md` to project-specific content
3. Replace fat `environment.json` with thin adapter
4. Remove redundant `.agents/skills/gbrain`, generic graphify rule copies
