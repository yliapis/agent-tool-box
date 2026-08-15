# `ScaffoldSpec`

The inputs to create one new artifact skeleton. `scaffold` is the inverse of
discovery: where sync reads `Artifact`s out of the source tree, scaffold writes
one in, laid out so a subsequent sync discovers it. This file defines the input
record; the templates, layout, and validation *rules* live in
[atb-scaffold](../atb-scaffold.md).

## `ScaffoldSpec`

| Field | Type | Required | Notes |
|---|---|---|---|
| `kind` | [`Kind`](enumerations.md#kind) | yes | Which skeleton to create. `--kind` defaults an omitted value to `skill` ([CLI](../atb-scaffold.md#cli)). |
| `tool` | [`Tool`](enumerations.md#tool) | yes | Template flavor. `--tool` defaults to `claude`; v1 accepts only `Claude` — see [Invariants](#invariants). |
| `name` | `Text` | yes | The artifact's stem; the `id` it will discover as. Constrained — see [name](#name). |
| `description` | `Text?` | no | Fills the generated frontmatter's `description:`. Empty → a per-kind placeholder is written instead. |
| `dir` | `Path` | yes | The base directory the skeleton is created under. `--dir` defaults to `.` ([CLI](../atb-scaffold.md#cli)). Existence is a precondition of the operation, not of the value — see [Invariants](#invariants). |

### `name`

`name` is the tightest-constrained field in any model here because it becomes a
directory name, a filename, and a YAML scalar all at once:

- Pattern `^[a-z0-9]([a-z0-9-]*[a-z0-9])?$` — lowercase alphanumerics and
  internal hyphens; no leading, trailing, or doubled edge hyphen.
- At most 64 characters.

This is the strictest of the four harnesses' naming rules, so a name that
passes is legal on every tool, filesystem-safe, and needs no YAML quoting. It is
the **stem only**: scaffold appends `.md` itself for command and agent kinds, so
a `name` like `critique.md` is invalid (it fails the pattern), not redundant.

### Invariants

- **`name` is machine-safe by construction** — the pattern and length bound
  [above](#name).
- **v1: `tool = Claude`.** The declared type admits all four `Tool` variants,
  but v1's actual value set is `{Claude}`: only `Claude` has a template set,
  and any other value is rejected before anything is written
  ([atb-scaffold](../atb-scaffold.md#--tool)). The field is deliberately wider
  than v1 needs — per-tool templates are the reason it exists.
- **`dir` must exist** — a **precondition of the `scaffold` operation**,
  checked before any write ([validation](../atb-scaffold.md#validation)), not a
  constraint the value can carry: a `ScaffoldSpec` naming a missing directory
  is a well-formed value that the operation rejects.

### Kind → what gets created

The file written for each `kind`, and the `id` it will discover as, are the
[`KindLayout`](kind-layout.md#the-mapping) scaffold-path and id-from-`name`
rows — the round-trip back to [`Artifact`](artifact.md#artifact), stated
canonically as the [round-trip law](README.md#the-round-trip-law).

`tool` selects the template flavor for the file written. In v1 only `Claude`
has a template set; the other three `Tool` variants are accepted as values but
have no templates yet, so using them is an error rather than a silent
Claude-flavored write.

**Relationship to sync's models.** `ScaffoldSpec` adds no new field types — it
reuses [`Kind`](enumerations.md#kind) and [`Tool`](enumerations.md#tool) and
produces output shaped so discovery reads it back as an `Artifact` whose
[`ArtifactMeta`](artifact.md#artifactmeta) is populated. Scaffold does not
introduce a [`FileOp`](sync-plan.md#fileop) variant: it writes one file directly
rather than emitting a plan.

**Where used**

- **scaffold** — the sole input to the `scaffold` operation, which validates it,
  renders the template, and writes the file.
