# `Artifact`, `ArtifactMeta`

The discovery side of sync: one capability found in the source tree, and the
optional metadata read from its frontmatter.

## `Artifact`

One discovered capability — the unit that sync copies to a target.

| Field | Type | Required | Notes |
|---|---|---|---|
| `kind` | [`Kind`](enumerations.md#kind) | yes | Which category this is; fixes how `id` and `source` were derived. |
| `id` | `Text` | yes | The destination name under a target's `output`. See [Identity](#identity). |
| `source` | `Path` | yes | Where the artifact's content lives: the skill directory, or the command/agent file. |
| `meta` | [`ArtifactMeta`](#artifactmeta) | yes | Frontmatter-derived name/description; the record is always present, its fields may be empty. |

### Identity

`id` is the artifact's name at the destination and is derived from `kind`:

| `kind` | `id` | `source` |
|---|---|---|
| `Skill` | the skill directory name (e.g. `foo`) | the skill directory |
| `Command` | the filename including extension (e.g. `critique.md`) | the file |
| `Agent` | the filename including extension | the file |

**Uniqueness.** Within one discovered set (a single `Kind` under one `source`),
`id` is unique. Two artifacts with the same `id` would resolve to the same
destination path, so a collision is a hard error at discovery time, not a
last-write-wins merge. Identity is per-`Kind`: a skill and a command may share a
name without conflict because they are never discovered together.

## `ArtifactMeta`

Optional descriptive metadata lifted from the artifact's frontmatter — the
first `---` / `---` block of its primary file (`SKILL.md` for a skill, the file
itself for a command or agent).

| Field | Type | Required | Notes |
|---|---|---|---|
| `name` | `Text?` | no | The `name:` frontmatter value, if present. |
| `description` | `Text?` | no | The `description:` frontmatter value, if present. |

Both fields are best-effort: **missing or malformed frontmatter is not an
error** — the corresponding field simply stays empty. `ArtifactMeta` never
carries anything beyond these two fields; other frontmatter keys are not read.

**Round-trip with scaffold.** A freshly
[scaffolded](scaffold-spec.md#scaffoldspec) artifact is written with valid
frontmatter, so immediately after scaffold the same artifact discovers with
`meta.name` (and, when `--description` was given, `meta.description`) populated
rather than empty. See the round-trip invariant in
[atb-scaffold](../atb-scaffold.md#behavior).

**Where used**

- **sync** — `discover` produces `Artifact`s; `plan` consumes them to build
  [`FileOp`](sync-plan.md#fileop)s. `ArtifactMeta` is populated during discovery.
- **scaffold** — does not construct `Artifact`s, but its output is defined by
  what a subsequent discovery will read back into one (the round-trip
  invariant).
