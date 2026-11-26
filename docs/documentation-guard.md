# Documentation Guard — implementation draft

Status: SOURCE DRAFT; not compiled, released or operationally approved. No runtime rollout.

## Decision

Independent Rust CLI inside the toolkit repository, not a new service. Offline Git CLI supplies the snapshot and object IDs. The toolkit owns distribution; each consumer owns its configuration and curated documents. The generic core must never infer semantic correctness, scientific approval, or release status. Application Knowledge Engineering remains a separate app-owned knowledge/publishing workflow. No consumer-private fixtures or identifiers belong in this public repository.

## Engineering quality

Apply the [Rust CLI Engineering Standard](../skills/engineering/_standards/rust-cli-engineering-standard.md) with the existing general standards. Use clap derive and actual executable tests. This source draft is not an accepted CLI or runtime integration.

## Snapshot and commands

- `docs-guard init`: creates a configuration example only; never overwrites a config. Adapt identity and create/locate required docs.
- Stage configuration and curated inputs. `docs-guard plan` performs generation preflight without writing. `docs-guard generate` writes only the two derived artifacts, never stages them. Stage those artifacts and run `docs-guard check --index`.
- `docs-guard check` checks immutable HEAD; `--commit REF` resolves a specific commit; `--index` explicitly checks staged content. Working-tree edits are not certified by either mode.
- `status` reports the same snapshot findings; `doctor` diagnoses Git and repository location. `--json` gives structured reports with `schema_version=1`, stable outcome and finding codes; CLI syntax errors are clap-native on stderr with exit 2. Help/version remain human-readable.
- Exit 0 = command succeeded (generation or diagnosis is NOT a quality PASS); 1 = policy findings; 2 = execution/configuration error. No automatic fixes in check.
- Inventory records all supported Git paths and object IDs; submodule entries record commit IDs, symlinks record their Git objects. Manifest excludes itself to avoid recursion. Register is included using its predicted raw blob hash and normal non-executable mode. Generated files require LF, no clean filters, and mode 100644; check staged output before committing.
- Link audit uses a Markdown parser and checks repository-local file/directory targets against the selected snapshot, not the live filesystem. Fragment/heading validation, HTML and non-Markdown link formats are not implemented in this draft. External URLs are not fetched.
- Unsupported non-UTF-8 and control/ambiguous path characters fail explicitly rather than disappearing from inventories. Output traversal and symlinks are rejected. Unmanaged tracked targets and dirty output files are refused. Same-directory temporary files are used for per-file replacement; a crash or second-file failure can leave partially updated output, which must be reviewed and regenerated before staging. No transaction across both files; not a sandbox against concurrent malicious workspace writers. Git redirection environment variables are rejected; lazy fetching, replacement objects, optional locks and interactive prompts are disabled. Git support for these controls must be verified on both OS targets.

## Minimum information roles

Minimal: scope, agent rules, index, current status (roadmap, risks, next action).
Standard adds architecture, decisions (including rejected approaches), verification (evidence), handoff, history (milestones and PR/issue references).
Production adds operations/deployment. Scientific adds a scientific status role. Both extend standard; they are separate extensions, not cumulative with each other. Roles may share files; map existing paths to avoid migration churn. Additional requirements and document roots are configurable. No private project-specific policy is built in.

## Remaining acceptance gates

1. Resolve/pin dependencies and commit Cargo.lock; run fmt, check, clippy, tests and release build on authorized QA infrastructure.
2. Differential comparison against existing reference behavior using synthetic fixtures; privately run real consumer migration tests. Preserve domain-specific guards separately.
3. Integration tests: clean HEAD vs staged/unstaged changes, generation convergence, missing links/files, invalid configuration, symlink/submodule, path encoding, SHA-256 repositories, filters/CRLF, Windows/Linux; document exact job evidence.
4. Verify per-file atomic replacement, partial-failure recovery, Git environment controls and filters; establish supported cross-platform path policy and strict snapshot identity evidence for index mode.
5. Verify changed-output reporting in plan; optional anchors/advanced link syntax and change-trigger rules need design/test. Add generic enforcement for documentation-change review only where justified.
6. Add toolkit installation/update/doctor, version pin and checksum-verified release assets; no release until verified. Use existing local/cloud adapter infrastructure and no per-repository toolkit clone.
7. Extend existing external QA runner with allowlisted standalone check, no weakening or replacement of existing gates before proven parity. No GitHub Actions, no changes to runner/Contabo during source-draft phase.
8. Pilot on a reference repository and a structurally different non-Rust repository. Keep previous guard until migration acceptance. Required status checks and protected-branch enforcement are separate from local agent rules.

## Agent contract after acceptance

Read AGENTS.md and canonical index/status/decisions/evidence/handoff first. Update curated docs with material changes or explicitly justify no change. Stage inputs, generate derived inventory, stage outputs, check index, then check the actual immutable commit in QA. Never interpret an inventory PASS as a semantic or product release approval.

## Dependency rationale

`tempfile` is used for safe temporary-file creation and same-directory replacement (and isolated test fixtures), rather than custom random naming or truncating targets. See [NamedTempFile::persist](https://docs.rs/tempfile/latest/tempfile/struct.NamedTempFile.html#method.persist). Git environment controls are based on [Git documentation](https://git-scm.com/docs/git). Dependency resolution and RustSec remain unverified until authorized QA.
