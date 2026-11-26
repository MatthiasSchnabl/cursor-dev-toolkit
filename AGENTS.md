# Repository agent contract

This is the global Cursor Dev Toolkit; README.md and the engineering context anchor in rules/development-tool-routing.mdc define scope and routing. Read the matching engineering standards and existing audit ledger before making changes. Use the existing toolkit standards for Rust, CLI, testing and security; no new audit framework.

For centrally managed Rust QA, obtain and follow the current Consumer-Startvertrag from the authorized runner operator. Verify repository identity, HEAD, project ownership, existing canonical source workspace, live tools and available capacity. Do not bypass workspace admission, share another project's source workspace, or alter runner infrastructure to accommodate this repository. Use the existing authorized quality runner; no GitHub Actions fallback. Do not allocate a database/test environment for the standalone documentation CLI.

Rust CLI source lives under tools/docs-guard; root Cargo.toml defines the workspace. Commit the reviewed root Cargo.lock and use locked quality gates. The consumer toolkit runtime must not require a Rust toolchain. See docs/documentation-guard.md and docs/documentation-guard-acceptance.md. Keep release pins pending until real test evidence and approval. Audit findings remain open until verified; source inspection is not PASS.

Do not commit private project content, secrets, generated build outputs, or infrastructure details to this public repository.
