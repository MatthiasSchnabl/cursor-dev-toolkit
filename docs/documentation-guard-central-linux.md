# Central Linux developer distribution

Scope: Linux amd64 only, using the existing generic Contabo test broker. Windows is out of current scope per owner decision on 26 November 2026. This does not change the existing installer defaults, production services, or RustRIMA science approval. No per-project binary build or new service.

Source baseline: bd6ec0f996ad0955d40e8d6dc3ee46a07baea882; locked dependency SHA-256 ce94e298d2ba071113cd29a464191c11b7244f696ab05d2981f7aef7bb3306f7. Linux native quality gate and push PASS (26/26 tests), locked release build and SHA-pinned installation PASS. RustSec reported no vulnerabilities against 1568 advisories; its own registry update warned, independently complemented by a fresh direct sparse-index check of 51 external package versions/checksums/not-yanked (all PASS). See PR #7 for job evidence.

Build a distinct image from the exact Git commit containing this Dockerfile, with the existing local Rust QA base read back before building. Verify final image identity and base layer inheritance. No existing tag overwrite. The derived binary is separately built; record its actual hash rather than assume equality. Use the existing test_run image parameter, providers=[], and docs-guard --root . doctor in each authorized workspace; initially inspect read-only. A consumer with no config needs a reviewed project-owned config and staged input paths before plan. Do not edit a dirty active consumer workspace or disable existing guard/scientific gates. Use parallel derived inventory paths until reference parity. No Windows or universal automatic rollout required for Linux developer use.

Image build/live consumer proof: pending until actual terminal evidence.
