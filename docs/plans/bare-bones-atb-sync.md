# Bare-bones `atb sync`

First cut is a Rust crate whose public types are the API, and whose only binary entry is `atb sync`. The normative spec is [docs/specs/atb-sync.md](../specs/atb-sync.md) — iterate there. YAML `--config` design lives in [docs/specs/config-spec.md](../specs/config-spec.md). This plan tracks sequencing only.

1. **Spec** — done. Review decisions folded in: symlinks followed, skip-nothing discovery, nested-`SKILL.md` error, abort-on-first-failure apply, Unix-only. `--config` is a TODO feature (flag optional, fails when present). Other deferred ideas live in the spec's TODO section (dry-run, filters, apply-behavior flags, nested skills).
2. **Scaffold** — done. Cargo package `agent-tool-box`, bin `atb`, edition 2024. `atb sync` parses `--src` / `--dst` / `--kind`; `--config` is optional and exits non-zero when present. No `discover` / `plan` / `apply` logic yet. Deps and modules beyond the CLI stub arrive with implementation.
3. **Implement** — blocked on spec iteration settling. Fill in `discover` / `plan` / `apply`, the `SkillCopy` adapter, and the spec's test list.
