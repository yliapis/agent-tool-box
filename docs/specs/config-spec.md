# `atb sync --config` — spec

YAML schema for `--config`. This surface builds a [Distribution](domain-model.md#distribution). CLI behavior (mutual exclusion with `--src` / `--dst`, current "flag optional, fails when present" stance) lives in [atb-sync](atb-sync.md).

## Schema

One `kind` per Distribution. `kind` is required.

```yaml
kind: skill
source: ~/src/dotfiles/ai-coding
targets:
  cursor:
    output: ~/.cursor/skills
  claude:
    output: ~/.claude/skills
```

YAML `source` becomes `Distribution.root`. Targets are keyed by `Tool` name (`claude`, `cursor`, `codex`, `opencode`). That is how `Target.tool` gets `Some(...)`. The flag path leaves it `None`.

## Validation

**Every problem is an error**, not a warning: unknown YAML fields (`deny_unknown_fields`), a missing `kind:`, a typo'd target key (anything not a `Tool` variant name), and an empty or missing `targets:`.

## Types

`Distribution`, `Target`, `Tool`, and `Kind` live in [atb-sync](atb-sync.md). Their language-agnostic definitions are in the [domain model](domain-model.md). This path only adds YAML → `Distribution`.

## Implementation

- `Cargo.toml` — add `serde`, `serde_yaml_ng` (`serde_yaml` is archived; this is the API-compatible fork)
- `src/config.rs` — YAML → `Distribution` (the flags path already builds a `Distribution`; this adds the file path)

## Tests

One parse test for the YAML above. Error-path: an omitted `kind:` fails; an unknown config field fails.

## Out of scope

- `kinds:` multi-kind routing — one kind per Distribution
- `version`, `defaults`, `merge`
