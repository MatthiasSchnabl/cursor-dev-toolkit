---
name: raggw-consumer-onboard
description: Connect the current application to RAGGW using the private, pinned, versioned RAGGW consumer-bootstrap skill. Install the canonical skill on demand; never duplicate private RAGGW instructions in this public toolkit.
disable-model-invocation: true
---

# RAGGW Consumer Onboarding

Use from the **consumer application's repository**. This is a public loader, not an independently maintained RAGGW integration contract.

1. Locate the installed toolkit root (`$CURSOR_DEV_TOOLKIT_ROOT` or `~/.cursor/plugins/local/cursor-dev-toolkit`). Do not clone the toolkit into the consumer app.
2. Run `bash "$CURSOR_DEV_TOOLKIT_ROOT/scripts/raggw-consumer-bootstrap.sh" --pin`; record the immutable tag and commit.
3. Run `bash "$CURSOR_DEV_TOOLKIT_ROOT/scripts/raggw-consumer-bootstrap.sh" install` (on demand; requires authenticated Git access to the **private** RAGGW repository). Never put tokens in clone URLs or logs. If it fails, stop and explain the missing Git access; do not substitute a branch tip or public copy.
4. Run `bash "$CURSOR_DEV_TOOLKIT_ROOT/scripts/raggw-consumer-bootstrap.sh" --check`. Read the `RAGGW_CONSUMER_SKILL=...` file and its references. **Follow the complete canonical instructions**, not this loader alone.
5. Work from the canonical RAGGW OpenAPI, MCP, Discovery and operator-provisioning contracts. Keep secrets out of code and logs; the consumer cannot self-grant RAGGW privileges. No production provisioning, publish, release or deploy without explicit approval.
6. Run the canonical consumer acceptance checks against an authorized isolated/test space. Record exact skill pin, verified capabilities, and remaining operator dependencies.

The application owns its domain knowledge, user authorization and business-action tools. RAGGW provides knowledge storage and retrieval only. Do not confuse onboarding with `/app-knowledge-publish`, which is an explicit staged-only release action.
