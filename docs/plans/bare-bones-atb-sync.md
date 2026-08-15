# Bare-bones `atb sync`

First cut is a Rust crate whose public types are the API, and whose only binary entry is `atb sync`. The normative spec is [docs/specs/atb-sync.md](../specs/atb-sync.md) — iterate there. This plan tracks sequencing only.

1. **Spec** — done. Review decisions folded in: symlinks followed, skip-nothing discovery, nested-`SKILL.md` error, abort-on-first-failure apply, strict config validation, Unix-only. Deferred ideas live in the spec's TODO section (dry-run, filters, apply-behavior flags, nested skills).
2. **Scaffold** — done. Cargo package `agent-tool-box`, bin `atb`, edition 2024. `atb sync` parses the full flag surface (mutual exclusion included) and exits non-zero with a "spec in flight" pointer; no `discover` / `plan` / `apply` logic. Deps and modules beyond the CLI stub arrive with implementation.
3. **Implement** — blocked on spec iteration settling. Fill in `discover` / `plan` / `apply`, the `SkillCopy` adapter, and the spec's test list.
