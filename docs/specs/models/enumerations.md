# Enumerations — `Tool`, `Kind`

Two closed enumerations are the shared vocabulary every other model is built
from. Both are small and fixed: adding a variant is a deliberate change that
ripples into discovery, copy layout, templates, and the CLI, so they are defined
once here and referenced everywhere.

## `Tool`

Which **agent harness** a capability is destined for — a destination CLI or app
whose config directories hold skills, commands, and agents.

| Variant | String form | Meaning |
|---|---|---|
| `Claude` | `claude` | Anthropic's Claude Code / Claude apps. |
| `Cursor` | `cursor` | Cursor editor. |
| `Codex` | `codex` | OpenAI Codex CLI. |
| `OpenCode` | `opencode` | OpenCode. |

The **string form** is the identity used outside the program: it is the
`targets.<tool>` key in the YAML config and the `--tool` flag value. The set is
closed — any other string is an error, not a new tool.

**Where used**

- **sync** — labels a [`Target`](config.md#target) (`Target.tool`). In v1 all
  four harnesses share one copy layout, so the label is carried but not read by
  the copy; it exists so a future per-tool adapter can branch on it.
- **config** — the `targets` map is keyed by string form; each recognized key
  sets the corresponding `Target.tool` to `Some`.
- **scaffold** — selects the template flavor via
  [`ScaffoldSpec.tool`](scaffold-spec.md#scaffoldspec). v1 ships `Claude`
  templates only; the other three variants are valid values but have no template
  set yet and are rejected at use time.

## `Kind`

Which **category of capability** an artifact is. `Kind` is the primary axis of
the tool: it decides how discovery finds an artifact, how an artifact's `id` and
`source` are derived, how the copy is laid out, and which template scaffold
emits.

| Variant | String form | Meaning |
|---|---|---|
| `Skill` | `skill` | A directory of instructions plus supporting files, marked by a `SKILL.md`. |
| `Command` | `command` | A single-file slash command. |
| `Agent` | `agent` | A single-file agent definition. |

Every input surface that selects a `Kind` — sync's `--kind` flag, the YAML
`kind:` field, scaffold's `--kind` — defaults an omitted value to `skill`. Each
surface spec owns its default ([sync CLI](../atb-sync.md#cli),
[YAML schema](../config-spec.md#schema),
[scaffold CLI](../atb-scaffold.md#cli)); the models carry no defaults.

### Per-kind layout

`Kind` determines the discovery marker, how an
[`Artifact`](artifact.md#artifact)'s `id` and `source` are derived, the copy
shape, and the scaffold path — one rule per variant, defined once as the
[`KindLayout` mapping](kind-layout.md#the-mapping). The full discovery walk and
copy behavior live in [atb-sync](../atb-sync.md#discovery).

**Where used**

- **sync** — drives discovery and the copy layout; one `Config` carries exactly
  one `Kind`.
- **config** — the optional `kind:` field (default `Skill`).
- **scaffold** — selects which skeleton to create and where
  ([`ScaffoldSpec.kind`](scaffold-spec.md#scaffoldspec)).
