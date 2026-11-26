# Rust CLI Engineering Standard v1

Status: proposed toolkit standard; implementation and QA remain subject to review.
Reviewed upstream: 2026-11-26. Complements the general Rust, Testing & Quality and Security & Supply-Chain standards; does not replace them.

## Contents

1. Interface and parser
2. Output, exit status and errors
3. Side effects and security
4. Tests and release gates
5. Dependency and distribution policy
6. Sources and rationale

## 1. Interface and parser

- Default to clap 4 with derive for non-trivial Rust CLIs: Parser, Subcommand, Args and ValueEnum as appropriate. Do not hand-roll an argument parser. Small one-purpose programs can justify std-only parsing.
- Model the domain separately from transport; keep main thin and delegate behavior to testable functions/modules. Use typed inputs, PathBuf for filesystem arguments, explicit subcommands and meaningful help for public options.
- Provide --help and --version. Reject invalid and conflicting flags. Avoid ambiguous destructive defaults. Use explicit scope/target and non-interactive commands for automation.
- Version the CLI as a contract. Breaking changes to arguments, exit semantics or JSON schema require migration guidance and an intentional version change. Avoid unnecessary abstractions, plugins, async runtimes or TUI frameworks.

## 2. Output, exit status and errors

- Human output by default. Primary results on stdout; diagnostics and human errors on stderr. No progress or decorative escape sequences in machine output.
- For agent/CI-consumed tools, provide --json or an equivalent documented format. Emit one versioned JSON result per successful parse, with stable outcome/error codes, useful context, and actionable diagnostics. If syntax errors remain clap-native (stderr, code 2), explicitly document this boundary and test it.
- Exit 0 is successful execution, not necessarily a verified PASS; define what each command proves. Nonzero codes distinguish policy/check findings from invalid invocation/environment failures when relevant. Default convention for guards: 0 success, 1 findings, 2 usage/config/execution error.
- Use Result/typed errors; no panic for expected invalid input, failed subprocesses, or I/O. Do not leak secrets. Propagate output write failures, subprocess errors and timeouts without claiming success. Handle broken pipes intentionally; no panic.
- Stable machine schema is not the same as stable human prose. Tests should assert semantic fields and exit codes, not incidental whitespace.

## 3. Side effects and security

- Read-only check/status/plan operations must not modify source, index, files or configuration. Explicit write subcommands; preview changes for significant mutations. Never silently fix a failed check. Repeated generation converges.
- Validate paths and scope before writing; reject traversal, symlink escapes and unsupported input. Avoid clobbering user files. Prefer atomic replacement per file; explicitly document multi-file partial failure and recovery. No claim of safety against concurrent malicious workspace writers without isolation.
- External programs via Command with separate arguments, no constructed shell strings; validate trusted executable, environment, cwd and argument boundaries. Bound expensive external operations and output at the appropriate trust boundary. Fail closed if a dependency is missing. No interactive auth/prompt or network access for an offline guard.
- Treat repository contents as untrusted data. No executing project code merely to check documentation. Keep private consumer data out of public tool repos.

## 4. Tests and release gates

- Unit-test pure invariants. Run clap CommandFactory::command().debug_assert() and parser tests for invalid combinations.
- Integration-test the actual executable with isolated temporary fixtures. Assert status, stdout/stderr, JSON contract, help/version, invalid inputs, missing dependencies, read-only behavior, idempotence, write failure and recovery where applicable. Include Linux and Windows if distributed there; test spaces and Unicode paths and document unsupported cases.
- std::process::Command + Cargo's CARGO_BIN_EXE_* is sufficient for simple binary tests; assert_cmd/assert_fs/trycmd may be justified when they materially simplify suites. Do not add them automatically.
- Required release evidence: cargo fmt --all -- --check; cargo check --locked --all-targets; cargo clippy --locked --all-targets -- -D warnings; cargo test --locked; cargo build --release --locked; existing RustSec/dependency policy. Run on the authorized QA infrastructure, not a substitute runner.
- Record exact source revision, toolchain, target, exit status and real job evidence. Unrun or ignored tests are not PASS. No merge/release on source review alone.

## 5. Dependency and distribution policy

- Prefer mature libraries and necessary features only; review dependency tree, RustSec, licenses and MSRV. Commit Cargo.lock for binary applications; build/install with locked dependencies. Do not fabricate lockfiles or claim a resolved version from a major-range declaration.
- Build centrally and distribute versioned, checksum-verified target binaries. Consumer projects need no Rust compiler. State OS/architecture/system-library and external executable requirements. Update explicitly; pin accepted artifacts, preserve rollback. No new service or auto-installation merely for a CLI.

## 6. Sources and rationale

Primary sources, retrieved 2026-11-26:

- [clap derive reference](https://docs.rs/clap/latest/clap/_derive/) and [derive tutorial](https://docs.rs/clap/latest/clap/_derive/_tutorial/): typed declarations, subcommands, validation.
- [clap tutorial](https://docs.rs/clap/latest/clap/_tutorial/): command configuration assertion; [crate overview](https://docs.rs/clap/latest/clap/): testing options.
- Rust CLI Book: [argument parsing](https://rust-cli.github.io/book/tutorial/cli-args.html), [testing](https://rust-cli.github.io/book/tutorial/testing.html), [output](https://rust-cli.github.io/book/tutorial/output.html), [exit codes](https://rust-cli.github.io/book/in-depth/exit-code.html).
- [Command Line Interface Guidelines](https://clig.dev/): human and automation interfaces, output streams, explicit changes and error reporting (community guidance, not a formal standard).
- [Rust Command API](https://doc.rust-lang.org/std/process/struct.Command.html): argument separation and Windows process caveats.
- [Cargo build](https://doc.rust-lang.org/cargo/commands/cargo-build.html) and [Cargo install](https://doc.rust-lang.org/cargo/commands/cargo-install.html): lockfile and reproducible dependency resolution.

Choice: clap derive rather than custom parsing; std process tests rather than another framework until a concrete need exists. Avoid a separate audit/fix skill pair for now; the existing general quality workflow applies. JSON and exit semantics above are toolkit policy choices, not claims that clap or the cited guidance mandates this exact schema.
