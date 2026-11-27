# Central Linux developer distribution

**Current approved image: `rust-mcp-test-docs-guard:0.1.0-5d625c9f`.** Use the updated release section below. The previous image is retained solely for rollback.

Scope: Linux amd64 only, using the existing generic Contabo test broker. Windows is out of current scope per owner decision on 26 November 2026. This does not change the existing installer defaults, production services, or RustRIMA science approval. No per-project binary build or new service.

Source baseline: bd6ec0f996ad0955d40e8d6dc3ee46a07baea882; locked dependency SHA-256 ce94e298d2ba071113cd29a464191c11b7244f696ab05d2981f7aef7bb3306f7. Linux native quality gate and push PASS (26/26 tests), locked release build and SHA-pinned installation PASS. RustSec reported no vulnerabilities against 1568 advisories; its own registry update warned, independently complemented by a fresh direct sparse-index check of 51 external package versions/checksums/not-yanked (all PASS). See PR #7 for job evidence.

Build a distinct image from the exact Git commit containing this Dockerfile, with the existing local Rust QA base read back before building. Verify final image identity and base layer inheritance. No existing tag overwrite. The derived binary is separately built; record its actual hash rather than assume equality. Use the existing test_run image parameter, providers=[], and docs-guard --root . doctor in each authorized workspace; initially inspect read-only. A consumer with no config needs a reviewed project-owned config and staged input paths before plan. Do not edit a dirty active consumer workspace or disable existing guard/scientific gates. Use parallel derived inventory paths until reference parity. No Windows or universal automatic rollout required for Linux developer use.

## Previous image (rollback only) — verified 2026-11-26

The shared QA image is built and locally available to the existing Rust-MCP test broker. No new daemon, port, global runner default or per-project installation.

- Image: `rust-mcp-test-docs-guard:0.1.0-e2ffbe3f`
- Image ID: `sha256:a36ca8f18b66933c869017edf7dc32b843233ad5fb352354fee3c51feef5a981`
- CLI SHA-256: `0c4e68492f7604523be34adcf6602e3f8a7c6092fa57eba30a1708a25a20cdc7`
- Image build revision: e2ffbe3ff5d6e42f9a52108e9a9aab21ecff905c. Existing base layer prefix verified.
- Live consumer test: job 9d9164e8eb2f46b7b946be834651d1e0, exit 0, 12 seconds, version 0.1.0, Git 2.39.5, executable hash verified. Consumer repository diagnostic PASS; no project configuration or migration acceptance implied.

For rollback only, use the existing `test_run` tool with the consumer's authorized workspace, the previous pinned image above, `providers=[]`, and command:

```sh
printf '%s  %s\n' 0c4e68492f7604523be34adcf6602e3f8a7c6092fa57eba30a1708a25a20cdc7 /usr/local/bin/docs-guard | sha256sum -c - && docs-guard --root . --json doctor
```

Check image identity before use and verify binary hash in the worker. Subsequent calls use project-owned reviewed configuration and the same image. Do not fall back to rebuilding or downloading a different binary if this reference is unavailable. Keep existing project gates until migration acceptance. Central Linux availability PASS; automatic default activation and consumer migration are separate.

## Updated Linux release — 2026-11-27

**Approved scope: central Linux developer QA image, opt-in use only.** Existing global installer pins and consumer-specific migration approvals stay unchanged. Supersedes the previous image for new native knowledge-export use; the previous image is retained as rollback.

- Image: `rust-mcp-test-docs-guard:0.1.0-5d625c9f`
- Image ID: `sha256:2c448aa3893bddecc5c77f17bc8161e4a21b3c8817b1166295004b1eaba8998f`
- Source: `5d625c9f4122e1dc4ebf65ec2ba18e3b4c793631` (exact Git build context, revision label overridden explicitly to match this commit).
- Executable SHA-256: `a428bf615b29f6396be7465ebb3628b026c9f8ac2194e5a39bc5889e8eb2a55d`.
- Verified existing base identity and layer prefix, Linux amd64, locked release build PASS. Cargo.lock is identical to the previously audited baseline, with no added dependencies. RustSec job `4ae3d03f364f4e32b022f06214faa55c` found no vulnerability in 1568 advisories on 2026-11-26; separate sparse-index job `76f588bf8a27450984e90ee5dcf462dd` verified all 51 external versions/checksums/not-yanked. This is reused evidence, not a new advisory-database scan.
- Quality job `e78e62d76443465c8d976e3c7540cf35`: fmt, check, Clippy, 27/27 tests, release build PASS on identical Rust source. Image smoke `d361a8afbd4046039a2c926c12822d72`: rejected unmodified example configuration (invalid placeholder identity). Corrected smoke `263cac00a1694f208a9210c0054aec1a`: exit 0, 10 seconds; executable hash, doctor, init, generate, committed snapshot check and native knowledge export PASS on isolated synthetic repository. No Python required. No external provider.

Use the new image explicitly through existing `test_run` and verify its executable before running the project-specific command:

```sh
printf '%s  %s\n' a428bf615b29f6396be7465ebb3628b026c9f8ac2194e5a39bc5889e8eb2a55d /usr/local/bin/docs-guard | sha256sum -c - && docs-guard --root . --json doctor
```

This is central availability, not an automatic migration of any consumer, independent security certification, or approval of GBrain indexing. No Windows distribution or project gate replacement.
