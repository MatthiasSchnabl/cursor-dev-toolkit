# Cursor constraints (verified 2026-03)

Answers to architecture questions A–F from Cursor documentation.

| ID | Question | Answer |
|----|----------|--------|
| A | User-scope plugin works cross-repo locally | **Yes** — Customize → Install → User scope |
| B | User-scope plugins in Cloud Agents | **Not documented** — cloud VMs lack local `~/.cursor/` |
| C | Plugin hooks in Cloud | **Limited** — repo `.cursor/hooks.json` only; `sessionStart`/`workspaceOpen` **not** in cloud |
| D | Hooks install CLI / persist $HOME | **Local** sessionStart yes; **Cloud** install hook persists build snapshot |
| E | Plugin dependencies field | **No** official dependency mechanism |
| F | Private personal marketplace | **Teams/Enterprise only**; dev uses `~/.cursor/plugins/local/` |

## Decision

- **Local:** User-scope Cursor Plugin (`cursor-dev-toolkit`)
- **Cloud:** Minimal per-repo `environment.json` adapter calling pinned toolkit bootstrap

Expected status: **GLOBAL LOCAL VERIFIED; CLOUD REQUIRES MINIMAL REPO ADAPTER**

## Cloud experiment

Test in a repo without local tooling copies:

1. Thin `.cursor/environment.json` pointing at toolkit `v0.1.0`
2. Verify CLI tools after build
3. Plugin rules/skills in cloud: **expected PENDING** (user plugin not loaded in VM)

See [adr-global-cursor-tooling.md](adr-global-cursor-tooling.md).
