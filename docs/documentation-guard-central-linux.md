# Central Linux developer distribution

Scope: Linux amd64 only, using the existing generic Contabo test broker. Windows is out of current scope per owner decision on 26 November 2026. This does not change the existing installer defaults, production services, or RustRIMA science approval. No per-project binary build or new service.

Source baseline: bd6ec0f996ad0955d40e8d6dc3ee46a07baea882; locked dependency SHA-256 ce94e298d2ba071113cd29a464191c11b7244f696ab05d2981f7aef7bb3306f7. Linux native quality gate and push PASS (26/26 tests), locked release build and SHA-pinned installation PASS. RustSec reported no vulnerabilities against 1568 advisories; its own registry update warned, independently complemented by a fresh direct sparse-index check of 51 external package versions/checksums/not-yanked (all PASS). See PR #7 for job evidence.

Build a distinct image from the exact Git commit containing this Dockerfile, with the existing local Rust QA base read back before building. Verify final image identity and base layer inheritance. No existing tag overwrite. The derived binary is separately built; record its actual hash rather than assume equality. Use the existing test_run image parameter, providers=[], and docs-guard --root . doctor in each authorized workspace; initially inspect read-only. A consumer with no config needs a reviewed project-owned config and staged input paths before plan. Do not edit a dirty active consumer workspace or disable existing guard/scientific gates. Use parallel derived inventory paths until reference parity. No Windows or universal automatic rollout required for Linux developer use.

## Available on Contabo — verified 2026-11-26

The shared QA image is built and locally available to the existing Rust-MCP test broker. No new daemon, port, global runner default or per-project installation.

- Image: `rust-mcp-test-docs-guard:0.1.0-e2ffbe3f`
- Image ID: `sha256:a36ca8f18b66933c869017edf7dc32b843233ad5fb352354fee3c51feef5a981`
- CLI SHA-256: `0c4e68492f7604523be34adcf6602e3f8a7c6092fa57eba30a1708a25a20cdc7`
- Image build revision: e2ffbe3ff5d6e42f9a52108e9a9aab21ecff905c. Existing base layer prefix verified.
- Live consumer test: job 9d9164e8eb2f46b7b946be834651d1e0, exit 0, 12 seconds, version 0.1.0, Git 2.39.5, executable hash verified. Consumer repository diagnostic PASS; no project configuration or migration acceptance implied.

Use the existing `test_run` tool with the consumer's authorized workspace, the pinned image above, `providers=[]`, and command:

```sh
printf '%s  %s\n' 0c4e68492f7604523be34adcf6602e3f8a7c6092fa57eba30a1708a25a20cdc7 /usr/local/bin/docs-guard | sha256sum -c - && docs-guard --root . --json doctor
```

Check image identity before use and verify binary hash in the worker. Subsequent calls use project-owned reviewed configuration and the same image. Do not fall back to rebuilding or downloading a different binary if this reference is unavailable. Keep existing project gates until migration acceptance. Central Linux availability PASS; automatic default activation and consumer migration are separate.
