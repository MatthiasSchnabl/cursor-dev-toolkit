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

## Controlled migration

Prefer parallel derived paths for initial evaluation, leaving the original guard and files intact. Configure existing curated roles without moving documents. The default example discovers configured extensions throughout the repository (`document_roots = ["."]`), including root Markdown and nonstandard documentation directories. Explicit optional documents may be included even outside selected roots/extensions; absence is not an error. Required documents cannot be excluded from the register or link audit.

For an approved in-place cutover, review the two legacy generated files, commit the existing baseline, stage the migration configuration, and obtain each current object ID with `git rev-parse :<path>`. Run `plan --adopt-register <OID> --adopt-manifest <OID>`, review changed outputs and legacy adoptions, then repeat with `generate`. Each supplied object ID must match an unmanaged, regular, tracked output. Dirty or missing worktree targets are refused. Omit adoption flags for an already managed target. Stage and check the generated outputs; retain the old guard until parity and QA approval. Do not pass adoption flags to ordinary generation. On failure review and restore only affected outputs from the preserved baseline before retry; no broad reset.

Generation checks staged Git attributes for filters, working-tree encoding, ident expansion and CRLF output rules before writing. Use LF for generated files. This is a preflight, not proof that the entire live staging environment matches the staged attributes. Links to symlinks and submodule roots are explicitly unsupported rather than certified by their presence.

## Change-dependent documentation (opt-in)

Define named rules in the project configuration. A rule watches literal path or directory prefixes (no globs) and requires all listed curated documents to change, or a change to one explicit review document. The review document must record a concrete justification; the tool only verifies that it changed, never that the justification is sufficient.

Replace the example's empty `change_rules = []` with TOML array-of-table entries (place them after the roles table):

```toml
[[change_rules]]
id = "architecture-change"
watch = ["src/api"]
require = ["docs/ARCHITECTURE.md", "docs/PROJECT_STATUS.md"]
review_document = "docs/CHANGE_REVIEW.md"
```

Declare all target documents as role paths or required/optional curated documents. The watch scope must not include the target or generated artifacts/config. For QA use `check --commit <candidate> --base <trusted-ancestor>`; for staged work use `check --index --base HEAD`. A trusted runner must always supply the externally selected base, including when the candidate removes all rules. Current and prior rules are checked when the policy changes, preventing a same-change bypass within that comparison. Policy changes themselves still require explicit review; a caller-chosen base is not authorization or branch protection. A configured rule without a base is a configuration error. Generation and plan do not certify change rules. Bootstrap the policy on an existing baseline before enforcing it on subsequent changes. Initial uncommitted repositories with enabled rules need a baseline commit first.

Change detection compares Git entries (mode and object ID); additions, deletions and renames are represented as changed paths. A required document or review alternative must change its blob ID; mode-only changes cannot satisfy the obligation. For commit checks the comparison base must precede the candidate (not be the same commit). No semantic inference or automatic approval. JSON reports identify the comparison base and whether change rules were checked.

## Runtime bounds and diagnostics

Git subprocesses are direct argument-vector calls, with concurrent pipe draining, 20-second per-call deadline, cumulative 180-second Git invocation budget, 32 MiB stdout/stdin caps and 64 KiB stderr cap. The direct child is killed/reaped on failure; this is not a general process-tree sandbox. Markdown audit is limited to 4 MiB per file, generated/worktree output to 32 MiB per file, and findings to 10,000; exceeding bounds is an execution error, never a truncated PASS. These initial limits require calibration on real repositories. Use trusted Git on PATH and existing runner-level timeout/isolation as the outer boundary.

Stale manifests report counts and up to five example paths each for added, removed and changed Git entries; rename appears as remove/add, not inferred identity. Broken links include line numbers. Generated-output path checks reject Windows reserved names, trailing dot/space, illegal characters and Windows reparse points; cross-platform alias and concurrent filesystem behavior still need tests. Unsupported input is explicit failure, not silently excluded.

## Remaining acceptance gates

1. Resolve/pin dependencies and commit Cargo.lock; run fmt, check, clippy, tests and release build on authorized QA infrastructure.
2. Differential comparison against existing reference behavior using synthetic fixtures; privately run real consumer migration tests. Preserve domain-specific guards separately.
3. Integration tests: clean HEAD vs staged/unstaged changes, generation convergence, missing links/files, invalid configuration, symlink/submodule, path encoding, SHA-256 repositories, filters/CRLF, Windows/Linux; document exact job evidence.
4. Verify per-file atomic replacement, partial-failure recovery, Git environment controls and filters; establish supported cross-platform path policy and strict snapshot identity evidence for index mode.
5. Verify changed-output reporting, opt-in change rules, trusted-base enforcement and review-document alternative. Anchor/HTML and advanced non-Markdown links remain outside v1.
6. Validate the explicit offline installer and activate only approved pins. Integrate existing local/cloud bootstrap only after release acceptance; no per-repository toolkit clone.
7. Extend existing external QA runner with allowlisted standalone check, no weakening or replacement of existing gates before proven parity. No GitHub Actions, no changes to runner/Contabo during source-draft phase.
8. Pilot on a reference repository and a structurally different non-Rust repository. Keep previous guard until migration acceptance. Required status checks and protected-branch enforcement are separate from local agent rules.

## Central binary distribution (source draft)

`versions.env` holds the version and release-state pin. `scripts/docs-guard-release.lock.tsv` contains immutable approved target/source/checksum rows; it intentionally has no rows and the release state is pending. `scripts/docs-guard-install.sh doctor|install <local-release-binary>|verify` is an explicit, offline installer/verification adapter using the existing toolkit shell helpers. No download, compilation or implicit PATH activation; no action in existing bootstrap/update until the release is accepted.

Once real QA has produced target binaries and approved their hashes, commit exact pins and release assets from the same reviewed source. Install into the existing user-local toolkit data directory, version/target-specific, outside the plugin copy tree. Verify SHA-256 before copying, after copying and on each verify. Refuse replacing a different binary under an existing immutable version. Use the verified absolute executable path in Cursor and runner integrations, not an unverified PATH match. The installer uses same-filesystem hard-link publication; unsupported filesystems fail closed. Test this on Linux and native Windows via Git Bash before activation. The isolated `scripts/docs-guard-install.selftest.sh` covers pending/missing pins, bad checksums, installation, idempotence and tampering using synthetic bytes; it never executes an artifact or writes the actual home directory. The test is authored but not run. macOS and untested target rows are not supported by this draft. Rollback selects a previously accepted immutable version, not an overwritten binary.

## Agent contract after acceptance

Read AGENTS.md and canonical index/status/decisions/evidence/handoff first. Update curated docs with material changes or explicitly justify no change. Stage inputs, generate derived inventory, stage outputs, check index, then check the actual immutable commit in QA. Never interpret an inventory PASS as a semantic or product release approval.

## Dependency rationale

`tempfile` is used for safe temporary-file creation and same-directory replacement (and isolated test fixtures), rather than custom random naming or truncating targets. See [NamedTempFile::persist](https://docs.rs/tempfile/latest/tempfile/struct.NamedTempFile.html#method.persist). Git environment controls are based on [Git documentation](https://git-scm.com/docs/git). Dependency resolution and RustSec remain unverified until authorized QA.
