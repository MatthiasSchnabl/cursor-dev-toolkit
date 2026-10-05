---
name: engineering-context-doctor
description: Verifies the local cursor-dev-toolkit engineering context, including installed plugin freshness, compact rule anchors, skill discovery structure, progressive-disclosure limits, reference Contents maps, and SessionStart context injection. Use after installs or updates, or whenever rules or skills appear missing or stale.
disable-model-invocation: true
---

# Engineering Context Doctor

Use this skill to prove that the local engineering rules and skills are installed and structurally usable.

## Run

Locate the installed plugin root, normally:

`~/.cursor/plugins/local/cursor-dev-toolkit`

Then execute:

`"$TOOLKIT_ROOT/scripts/verify-engineering-context.sh"`

If the source checkout lives outside Cursor's local plugin directory, run the normal installer first so the local plugin copy is refreshed.

## Interpret results

- **PASS** — checked contract is satisfied.
- **INFO** — diagnostic information or a comparison that does not apply in this install mode.
- **FAIL** — do not assume the engineering setup is current; fix the reported problem and reload Cursor.

The doctor verifies:

- plugin manifest/version consistency,
- required compact engineering rule anchors,
- every engineering `SKILL.md` is below 500 lines,
- long references expose `## Contents` within the first 100 lines,
- SessionStart injects the engineering context anchor,
- installed plugin bytes match the source checkout when both are available,
- installed/source revision information.

After correcting a failure, run **Developer: Reload Window** and rerun this doctor.
