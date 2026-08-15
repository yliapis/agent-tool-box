# `KindLayout`

The per-`Kind` layout rule: how each [`Kind`](enumerations.md#kind) variant
maps onto the filesystem. `KindLayout` is a **static mapping**, not a runtime
value — no binding defines a `KindLayout` type, and exactly one row exists per
`Kind` variant, fixed at design time. Discovery, the copy, and scaffold are
each an implementation of some of its columns.

It is defined once, here, because it is the most-shared fact in the system:
[`Artifact`](artifact.md#artifact) identity, the copy shape in
[`SyncPlan`](sync-plan.md#syncplan), and [`ScaffoldSpec`](scaffold-spec.md#scaffoldspec)
output are all projections of this table, and their agreement is exactly what
makes the [round-trip law](README.md#the-round-trip-law) hold.

## The mapping

| | `Skill` | `Command` | `Agent` |
|---|---|---|---|
| **Marker** — what discovery matches | `**/skills/*/SKILL.md` | `**/commands/*.md`, direct children only | `**/agents/*.md`, direct children only |
| **`source` referent** | the skill directory | the matched file | the matched file |
| **`id` from `source`** | the directory name | the filename, extension included | the filename, extension included |
| **Primary file** — where frontmatter is read | `{source}/SKILL.md` | `source` itself | `source` itself |
| **Copy shape** — ops per artifact | one `Copy` per file under `source`, to `{output}/{id}/{relpath}` | one `Copy` of `source`, to `{output}/{id}` | one `Copy` of `source`, to `{output}/{id}` |
| **Scaffold path** | `{dir}/skills/{name}/SKILL.md` | `{dir}/commands/{name}.md` | `{dir}/agents/{name}.md` |
| **`id` from `name`** | `{name}` | `{name}.md` | `{name}.md` |

Behavior around the mapping — symlink following, the nested-`SKILL.md` error,
what happens to a file in `commands/foo/bar.md` — stays in
[atb-sync](../atb-sync.md#discovery).

## The `id` derivation

`Artifact.id` is a **function**, never free-standing data. It has two
derivations, one per direction:

- **From a discovered `source`** — the skill directory's name, or the
  command/agent filename including its extension.
- **From a scaffold `name`** — `{name}` for a skill; `{name}.md` for a command
  or agent (scaffold appends the extension itself).

The two derivations agree by construction: scaffolding `name` at the scaffold
path and then discovering yields exactly the `id` the name row predicts. That
agreement is the mechanical core of the
[round-trip law](README.md#the-round-trip-law).

**Where used**

- **sync** — discovery implements the marker, referent, and id rows;
  `CopyAdapter` implements the copy-shape row
  ([atb-sync](../atb-sync.md#discovery)).
- **scaffold** — writes the scaffold-path row and relies on the id agreement
  ([atb-scaffold](../atb-scaffold.md#layout-and-templates)).
