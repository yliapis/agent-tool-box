# `atb discover` — spec

This file is the spec for the list command. `atb discover` walks a tree. Then the command builds a [Catalog](domain-model.md#catalog). Then the command prints the Catalog.

The command calls the same `discover(root, kind)` that [`atb sync`](atb-sync.md) runs before plan and apply. The command stops after the Catalog. The command does not plan. The command does not write. This file is the place for design changes.

## Behavior

```
atb discover
atb discover --src ~/src/dotfiles/ai-coding
atb discover --src ~/src/dotfiles/ai-coding --kind command
```

The first command walks `.` for skills. The second command walks that tree for skills. The third command walks that tree for commands.

`--src` is the search root (`Catalog.root`). Sync uses the same flag name. The default is `.` (the current working directory). `--kind` is the `Kind` from sync. The default is `skill`.

The v1 values are `skill`, `command`, and `agent`. The `plugin` value is not in v1. This value needs `Kind::Plugin` in the binding.

```mermaid
flowchart LR
  flags[discover flags] --> discover[discover by kind]
  discover --> catalog[Catalog]
  catalog --> text[list text]
```

The walk, the markers, the walk-error classification, zero matches, a duplicate `id`, and a nested `SKILL.md` are in the Discovery section of [atb-sync](atb-sync.md#discovery). This spec does not repeat those rules. A skip line goes to stderr. When a discover error is fatal, the command exits non-zero. Then the command prints nothing to stdout.

A successful Catalog is non-empty. That rule is a [Catalog invariant](domain-model.md#invariants). When the tree has no matches, `atb discover` fails. The error uses the same marker and search-root hint as sync. An empty list with exit 0 is a later choice. See TODO.

## Print

The command prints the Catalog to stdout. The output has no color. The output has no box drawing.

The paths are the paths that the Catalog stores. The command does not change the form of the paths.

The output is a header, a blank line, and one card per artifact. The artifacts stay in Catalog order (id, then source). A blank line separates two cards. The output ends with a newline.

Header:

```
{n} {kind-word} under {root}
```

`{n}` is `artifacts.len()`. `{root}` is `catalog.root`. `{kind-word}` is the kind in string form. When `n` is not 1, the word is plural:

| `Kind` | `n == 1` | `n != 1` |
|---|---|---|
| `Skill` | `skill` | `skills` |
| `Command` | `command` | `commands` |
| `Agent` | `agent` | `agents` |

Each card:

```
{id}
  {name}
  {description}
  {source}
```

The body of the card has these rules:

- The first line is always `{id}`. This line is the Catalog identity. It is not `meta.name`.
- When `meta.name` is `Some` and it is not equal to `id`, the card includes that name. In other cases, the card omits the name line. A skill that repeats the directory as `name:` does not print a name line. An agent with `id` `reviewer.md` and `name:` `reviewer` prints a name line.
- When `meta.description` is `Some`, the card includes that description. When `meta.description` is `None`, the card omits the description line. When the value contains a newline, the command replaces each newline with one space. As a result, each field stays on one line.
- The last line of the card is always `{source}`.

`ArtifactMeta` is not a domain noun. Sync does not read it for dest paths. This command prints `ArtifactMeta`. The command uses the frontmatter reader of [atb-sync](atb-sync.md#rust-api). When the frontmatter is missing or is not valid, the fields stay `None`. Then the card omits those lines.

This example is two skills in a temp tree (the sync fixture):

```
2 skills under /tmp/src

alpha
  Alpha skill
  /tmp/src/plugins/core/skills/alpha

beta
  Beta skill
  /tmp/src/plugins/extra/skills/beta
```

This example is a command with a description and with no `name:`:

```
1 command under /tmp/src

critique.md
  TODO: what /critique does.
  /tmp/src/commands/critique.md
```

This example is a skill with no frontmatter:

```
1 skill under /tmp/src

alpha
  /tmp/src/plugins/core/skills/alpha
```

## CLI

```
atb discover [--src <dir>] [--kind skill|command|agent]
```

The two flags are optional. The default of `--src` is `.`. The default of `--kind` is `skill`. The command has no `--dst` flag. The command has no `--config` flag. The command has no `--tool` flag.

When discover fails, the command exits non-zero. The errors are the same as the errors in sync: a fatal walk error, zero matches, duplicate ids, and a nested `SKILL.md` (Skill only). When `--src` is missing or is not a directory, the walk treats that path as a fatal error on `root`.

## Rust API

`discover` is public. This command adds the function that builds the list text. `Catalog` and `Kind` are in the [domain model](domain-model.md). The Rust block has no field comments.

```rust
pub fn format_catalog(catalog: &Catalog) -> String;
```

The binary calls `discover(&src, kind)`. Then it prints `format_catalog` to stdout. The crate has no `DiscoverSpec`. Two flags do not need an input record.

## Crate layout

- `src/main.rs` adds the `discover` subcommand.
- `src/discover.rs` adds `format_catalog`.
- `src/lib.rs` re-exports `format_catalog`.
- The crate needs no new dependency.

## Platform

The command supports Unix only. The rule is the same as [atb-sync](atb-sync.md).

## Tests

The tests do not include the walk again. Those cases are in the sync tests. The tests compare the list text and the CLI defaults to the rules in this file.

The output of `format_catalog` on the two-skill fixture must equal the first example. The root path comes from the temp dir. A skill with no frontmatter omits the name and the description. When the `name:` of an agent is the stem and the `id` is `{stem}.md`, the card includes the name line. When a description contains a newline, the card prints one line.

`atb discover --src <fixture>` prints the skill list and exits 0. `atb discover --src <fixture> --kind command` prints the command list. When the tree has no matches, the command exits non-zero and prints nothing to stdout. Skip lines from the walk go to stderr.

## TODO (considered, not committed)

- **`--format json` / `--format plain`.** v1 has one format: the list in this file. The `json` format prints the Catalog. The `plain` format prints one artifact per line (`id` and `source`) for scripts.
- **`--kind` all.** This flag builds three Catalogs in one run. An empty kind and an empty tree conflict with the non-empty Catalog invariant. The command accepts one kind per run.
- **Empty Catalog as success.** When the command finds nothing, the command can print `0 skills under {root}` and exit 0. That change belongs in the Catalog invariant. Or the empty rule moves to sync only. The command does not print an empty Catalog.
- **Group by plugin.** The source tree is `plugins/*/{skills,commands,agents}`. A grouped list infers a plugin name from `source`. Catalog has no plugin field. The list stays flat and sorted by `id`.
- **Move the walk spec here.** This command is the named `discover`. The walk stays in the Discovery section of [atb-sync](atb-sync.md#discovery), so that one file holds one pipeline. A later split can move the walk. The behavior does not change.
- **`plugin` kind.** The gap is the same as the gap in the sync CLI. This value needs `Kind::Plugin` in the binding.

## Cut from v1 (rationale)

| Item | Why it goes |
|---|---|
| Positional `PATH` | All verbs use flags. `--src` is already the flag name for `root`. |
| Color / box drawing | The other verbs print one line per result. Structure is enough. |
| `--tool` | This command walks the source tree. Tool destinations belong to sync. |
| `--config` | A Distribution is a write work order. This command has no Target. |
| `DiscoverSpec` | The command has two flags. Then it calls `discover`. |
| Labeled card fields (`name:`, `description:`, `source:`) | The line order is fixed. Most skills use the same text for `id` and `name`. Labels are extra. |
| Wrapping descriptions | The terminal wraps the text. The command does not wrap the text. |

## Out of scope

These items are out of scope:

- plan, copy, delete, or edit of artifacts
- MCP, rules, hooks
- plugin marketplace layout
- Windows
- any change to the discover walk
- any change to Catalog invariants
