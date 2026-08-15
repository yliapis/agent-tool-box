# `Artifact`, `ArtifactMeta`

The discovery side of sync: one capability found in the source tree, and the
optional metadata read from its frontmatter.

## `Artifact`

One discovered capability — the unit that sync copies to a target.

| Field | Type | Required | Notes |
|---|---|---|---|
| `kind` | [`Kind`](enumerations.md#kind) | yes | Which category this is; selects the [`KindLayout`](kind-layout.md#the-mapping) row that shaped the rest. |
| `id` | `Text` | yes | The destination name under a target's `output`. **Derived** — see [Invariants](#invariants). |
| `source` | `Path` | yes | Where the artifact's content lives — the [`KindLayout` referent](kind-layout.md#the-mapping): the skill directory, or the command/agent file. |
| `meta` | [`ArtifactMeta`](#artifactmeta) | yes | Frontmatter-derived name/description; the record is always present, its fields may be empty. |

### Invariants

- **`id` is derived, not free-standing.** `id` is a function of
  (`kind`, `source`) — the
  [`KindLayout` id rule](kind-layout.md#the-id-derivation): the skill
  directory's name, or the command/agent filename including extension. A
  binding stores it for convenience, but it carries no information beyond
  `kind` + `source`.
- **`id` is unique within a discovered set.** The *discovered set* is the
  `List<Artifact>` one discovery run produces for a single
  (`source` root, `Kind`) pair. Within it, `id` is unique: two artifacts with
  the same `id` would resolve to the same destination path, so a collision is
  a hard error at discovery time, not a last-write-wins merge.
- **Identity is per-`Kind`.** A skill and a command may share a name without
  conflict — they are never in the same discovered set.
- **`meta` is total; its fields are not.** The record is always present;
  missing or malformed frontmatter leaves its fields empty and is never an
  error.

## `ArtifactMeta`

Optional descriptive metadata lifted from the artifact's frontmatter — the
first `---` / `---` block of its
[primary file](kind-layout.md#the-mapping) (`SKILL.md` for a skill, the file
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
frontmatter, so the fields its template writes discover populated rather than
empty — `description` always (the given text or a placeholder), `name` for the
kinds whose template writes one. The canonical statement is the
[round-trip law](README.md#the-round-trip-law).

**Where used**

- **sync** — `discover` produces `Artifact`s; `plan` consumes them to build
  [`FileOp`](sync-plan.md#fileop)s. `ArtifactMeta` is populated during discovery.
- **scaffold** — does not construct `Artifact`s, but its output is defined by
  what a subsequent discovery will read back into one (the round-trip law).
