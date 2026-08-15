# `atb sync --config` — spec

YAML schema for `--config`. CLI behavior (mutual exclusion with `--src`/`--dst`, current "flag optional, fails when present" stance) lives in [atb-sync](atb-sync.md).

## Schema

One `kind` per `Config` (default `skill`).

```yaml
kind: skill
source: ~/src/dotfiles/ai-coding
targets:
  cursor:
    output: ~/.cursor/skills
  claude:
    output: ~/.claude/skills
```

`kind` is optional in YAML and defaults to `skill`. `kinds:` is an unknown field and is an error (`deny_unknown_fields`). No `version`, `defaults`, `kinds`, `merge`. Tilde expansion on `source` and `output`: a manual `~/` prefix swap using `std::env::home_dir()` (un-deprecated since Rust 1.87) — no `shellexpand` dep.

Targets are keyed by `Tool` name (`claude`, `cursor`, `codex`, `opencode`). That is how `Target.tool` gets `Some(...)`; the flag path leaves it `None`.

## Validation

**Every problem is an error**, not a warning: unknown YAML fields (`deny_unknown_fields`), a typo'd target key (anything not a `Tool` variant name), and an empty or missing `targets:`.

## Types

`Config`, `Target`, `Tool`, and `Kind` live in [atb-sync](atb-sync.md); their language-agnostic definitions are in the [domain models](models/README.md) ([config](models/config.md), [enumerations](models/enumerations.md)). This path only adds YAML → `Config`.

## Implementation

- `Cargo.toml` — add `serde`, `serde_yaml_ng` (`serde_yaml` is archived; this is the API-compatible fork)
- `src/config.rs` — YAML → `Config` (the flags path already builds a `Config`; this adds the file path)

## Tests

One parse test for the YAML above; one parse test that omitted `kind:` defaults to `skill`. Error-path: an unknown config field fails.

## Out of scope

- `kinds:` multi-kind routing — one kind per `Config`
- `version`, `defaults`, `merge`
