# Documentation Guard v0.1 — static review and acceptance ledger

Status: SOURCE REVIEW ONLY / NOT RUN / NO MERGE / NO RELEASE.

## Source review scope

Reviewed the generic CLI, Git snapshot comparison, generated-output ownership, legacy adoption, parser contract, Git process boundary, change-dependent documentation and installer pins. Compared with the toolkit Rust CLI, general Rust, Testing & Quality and Security & Supply Chain standards. No private consumer code copied. This is not an independent runtime audit; no GBrain, Graphify or gstack execution in the authorized GitHub-only phase. Context7 was unavailable in the previous phase. No Contabo or GitHub Actions jobs, no fabricated test results.

### Findings addressed in source

| Finding | Source response | Evidence status |
|---|---|---|
| Arbitrary legacy overwrite | Explicit exact staged object ID and dirty-file refusal | Test authored |
| Root/nonstandard docs missing | Repository-wide configurable discovery, optional documents | Test authored |
| Link targets outside Git | Snapshot-based Markdown parser, symlink/submodule target rejection | Partial test coverage |
| Configuration can remove change obligations in same diff | Trusted base required by runner; old and current rules evaluated | Test authored |
| File mode changes could satisfy documentation update | Require a changed blob object ID | Static review; dedicated mode-only test pending |
| Stale inventory lacks specifics | Added/removed/changed counts and samples | Test authored |
| Git process may hang or overproduce output | Concurrent bounded pipes, deadlines, direct child cleanup | Bounded reader unit test authored; process tests pending |
| Unverified artifact may be installed | Pending pin, immutable target and SHA-256 lock | Installer selftest authored |
| Read-only result may hide a side effect | Explicit snapshot readback; worktree/index comparison test | Test authored |

### Residual risks and explicit limits

- All Rust source, shell scripts, toolkit wiring and tests are uncompiled/unexecuted. Cargo.lock is absent. Resolve actual dependency versions, minimum toolchain and RustSec results before claiming any build.
- Git executable trust, partial clones, alternate environment/config sources, process-tree cleanup, timeout and cumulative resource behavior need real negative tests; the CLI is not a sandbox.
- Per-file replacement is not a two-file transaction; verify failure between writes, recovery, LF/filter behavior, case aliases, path encodings, Windows junctions and filesystem support.
- No Markdown heading-fragment or raw HTML/non-Markdown link audit in v1. No semantic verification, PR API synchronization or inferred rename identity. Rules prove changed blobs, not a sufficient review justification.
- Trusted QA chooses the comparison base and validates configuration/changes to policy. Without that control, local CLI use cannot enforce merge protection or detect intentional comparison-base manipulation. Base rules are not retroactive to an unrelated history.
- Installer scripts are drafts; pins empty, activation disabled. Hard-link behavior under native Windows Git Bash must be demonstrated. No implicit downloads or bootstrapping.
- Full reference parity and real non-Rust consumer acceptance remain mandatory. Preserve existing guard and project-specific checks until then.

## Ordered acceptance plan

All execution is deliberately deferred to the authorized QA environment. No GitHub Actions. The following commands are planned, not executed. Run from the toolkit repository; use the established Contabo Rust-MCP workflow once available, not a new runner.

1. Verify exact draft HEAD/tree, source scope and workspace identity. Resolve the root `Cargo.lock` once with `cargo generate-lockfile --manifest-path Cargo.toml`; inspect resolved dependencies, license, toolchain/MSRV, RustSec. Commit reviewed lockfile. No fabricated or arbitrary lock pin.
2. Run formatting (apply mechanical formatting in a dedicated source commit if needed), then `cargo fmt --manifest-path Cargo.toml --all -- --check`, `cargo check --manifest-path Cargo.toml --locked --all-targets`, `cargo clippy --manifest-path Cargo.toml --locked --all-targets -- -D warnings`, `cargo test --manifest-path Cargo.toml --locked`, `cargo build --manifest-path Cargo.toml --release --locked`. Apply existing supply-chain gates on the exact locked dependency tree.
3. Run installer selftest (`bash scripts/docs-guard-install.selftest.sh`) and existing toolkit engineering-context verification. Validate actual binary CLI and help, stdout/stderr, error schema, checks and generation on Linux and native Windows.
4. Extend tests where needed: actual timeout/overflow/missing Git, process cleanup, mode-only change, conflicting config paths, symlink/junction, submodule, SHA-256 Git repository, CRLF/filter, partial output failure, read-only snapshots and maximum-size handling. Failures block acceptance, not just report warnings.
5. Privately compare with the existing Python reference on the exact same source snapshot. Compare inventory set, Git object IDs, document scope and findings; document intentional format/coverage differences. Preserve project-specific checks and original output files during parallel pilot.
6. Test a real non-Rust consumer with distinct documentation layout and change rules. No consumer-side compiler/runtime dependency beyond supported Git and system requirements.
7. Only after successful evidence: build target binaries, record actual source SHA/target/toolchain/hash and publish approved release pins. Verify install/update/rollback. Extend Rust-MCP allowlisted existing gate and toolkit bootstrap only then. No automatic merge or runtime change in the current draft.

## Evidence record template

For each run record source commit/tree, Cargo.lock identity, runner/job ID, command, toolchain and target, exit code, test count (including ignored), output artifact references and decision. Record failures and corrections separately. All planned entries are currently NOT_RUN.

## Go/no-go

Source draft may proceed to compilation and QA. Productive rollout, merge and replacement of existing guards: NO_GO until successful acceptance.

## Error-contract acceptance extension

See [error-handling audit](documentation-guard-hardening-audit.md). Validate each public category against real CLI failures, including invalid config/commit, unavailable Git, Git exit/timeout/overflow, denied filesystem writes, changed snapshot, second-write failure, broken stdout/stderr, cleanup failure and secret-canary absence. Confirm stable JSON and exit code 2 (policy findings remain exit 1), no duplicate JSON on stream failure, and accurate completed/attempted output recovery. Unit-level injected sequence failure is not a substitute for actual filesystem fault tests. All NOT_RUN.

## Runner admission preflight (2026-11-26)

Status: BLOCKED before execution. Central runner health was reachable; no canonical source checkout for this repository could be identified in the available workspace inventory. The centrally enforced workspace lifecycle requires an authorized, clean, synchronized canonical main checkout before admitting a task workspace. Do not reuse another project's checkout or bypass admission via legacy clone/create, host access or alternate runners. No build/test job submitted, no source checkout created, no lockfile fabricated.

The root Cargo workspace now declares the CLI member with resolver 2 so the existing root-manifest quality profiles can discover it. The application lockfile belongs at repository root; generate it using authorized Cargo before locked gates. Build outputs are ignored and excluded from user-plugin copies. AGENTS.md documents consumer rules. Keep the existing draft branch, do not activate the release. Once the canonical source checkout is provisioned by the workspace owner, verify its origin/HEAD/clean status, bind it through the normal lifecycle, checkout the reviewed draft, resolve the root lockfile and proceed with the ordered gates. No database environment needed.

## Contabo execution evidence — 2026-11-26

Admission blocker resolved: canonical checkout provisioned by operator, WLP-03 main binding confirmed, exact branch baseline 8ea45216674578ab5cbf065c5412af3f4aa35db1. No R2 or R3 needed.

- Lockfile: job dab28c918b3f4e04b368aeed100168d3, exit 0; Cargo/rustc 1.99.1, Git 2.39.5. Root lockfile SHA-256 ce94e298d2ba071113cd29a464191c11b7244f696ab05d2981f7aef7bb3306f7; 51 registry dependencies, checksum fields present. License/security audit remains pending.
- Job 71d1df36d236472c95d5dff8b131f047: formatted source, locked all-target check PASS. Native job c933fdfdb75d40f9abc68d592e9ae480 identified two Clippy findings, corrected without suppression. Job 18fcd3a7ab7c427fa055b0367c45af7c formatting regression corrected.
- Native job 1f006fdd09944465a40964ebf39a2d06: fmt/check/clippy PASS, 7/7 unit PASS; 18 integration tests failed launching the CLI binary (OS ENOENT). Generic runner had concurrently built into the same target tree under a different absolute mount path, so stale CARGO_BIN_EXE paths are a plausible cause, not yet proven. Keep target trees separate for subsequent generic/native jobs; do not weaken or skip the native gate.
- Generic job c318997323aa4ff2b8ba8144253c91ca: 7 unit + 17 integration PASS; one synthetic fixture attempted git rm of an uncommitted staged file. Fixed fixture by committing baseline.
- Generic job 88ef13ff6dc54635be5778ba7cedbab9: locked 7/7 unit + 18/18 integration PASS, zero ignored, release build PASS; installer test failed because its temporary home was on non-executable /tmp. Overall job exit 1, not full PASS.
- Installer preflights execute permission before publishing and rechecks afterward. Job b154eb13af2748daabc37235dbc5a5b7 proved /tmp is mounted noexec and installer selftest PASS with TMPDIR inside ignored executable build scratch. No host mount change.
- Added Unix missing-Git CLI regression; rerun required. All findings remain open pending scoped runtime verification and final gates. No release pins approved.

Next: native gate and commit/push, separate generic target path, supply-chain validation, remaining real failure injection, Windows and reference/pilot acceptance.
