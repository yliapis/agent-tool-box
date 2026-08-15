# `atb scaffold` — spec

Normative spec for the second `atb` verb. `atb scaffold` creates the skeleton of a new artifact — a skill, command, or agent — in the layout [`atb sync`](atb-sync.md) discovers. The two verbs compose: scaffold into the source tree, sync out to the Tools. Sync's v1 cut every verb but `sync` and cut `--tool` for lack of divergence. Scaffold is where a second verb and `--tool` come back. It reuses sync's `Kind` and `Tool` and changes nothing about sync. Iterate here.

`scaffold` is the Artifact factory. An Artifact is born on disk in a valid layout.

## Behavior

```
atb scaffold --name code-review
atb scaffold --kind command --name critique --dir ~/src/dotfiles/ai-coding/plugins/core
```

The first writes `./skills/code-review/SKILL.md`. The second writes `…/plugins/core/commands/critique.md`. The next `atb sync` picks that file up.

**Round-trip.** Immediately after a successful scaffold, `discover(dir, kind)` succeeds. The Catalog includes exactly the new artifact. `id` and `meta` match the [round-trip law](domain-model.md#the-round-trip-law). That statement is canonical. It includes the collision proviso. The command template writes no `name:` frontmatter, so a command `meta.name` stays `None`.

Scaffold never overwrites. An existing destination is an error. Nothing is written. `--tool` (default `claude`) selects the template flavor. v1 ships claude templates only. Any other value exits non-zero (see [`--tool`](#--tool)).

## Layout and templates

The path each `kind` creates, and the `id` it discovers as, are the scaffold-path and id-from-`name` rows of the [layout convention](domain-model.md#layout-convention). The templates below are the content written at those paths.

Intermediate directories (`skills/`, the skill dir, `commands/`, `agents/`) are created with `create_dir_all`. `{dir}` itself must already exist. A missing `--dir` is more likely a typo than intent.

Templates are `const` strings in the binary with `{name}` / `{description}` filled by `format!`. No template engine. No user template dirs. The description is written as a YAML double-quoted scalar (escaping `\`, `"`, and newlines). Any `--description` text yields parseable frontmatter. `name` needs no quoting by construction (see Validation). When `--description` is absent, the per-kind placeholder below stays in.

`Skill` → `{dir}/skills/{name}/SKILL.md`:

```markdown
---
name: {name}
description: "TODO: what this skill does and when it should trigger — this line is the trigger signal."
---

# {name}

TODO: instructions the agent follows once this skill triggers.
```

`Command` → `{dir}/commands/{name}.md`:

```markdown
---
description: "TODO: what /{name} does."
---

TODO: the prompt. `$ARGUMENTS` expands to whatever follows `/{name}`.
```

`Agent` → `{dir}/agents/{name}.md`:

```markdown
---
name: {name}
description: "TODO: when to delegate to this agent."
---

TODO: the system prompt. Describe how this agent works, not when to pick it — `description` above does that.
```

The frontmatter must parse under sync's frontmatter reader (first `---` / `---` pair, YAML). The fields a template writes never discover as `None`. (The command template writes only `description:`, so a command `meta.name` is `None` by design, not by parse failure.)

## Validation

Every problem is an error. All checks precede the first write. An error writes nothing.

- **name** — `^[a-z0-9]([a-z0-9-]*[a-z0-9])?$`, at most 64 chars: the [`Artifact` identity rule](domain-model.md#identity). Scaffold is where names are born. The name is the stem. Scaffold appends `.md` itself. `--name critique.md` is invalid, not redundant.
- **dir** — `--dir` (default `.`) must exist and be a directory.
- **collision** — the artifact path (`{dir}/skills/{name}` for skills, the `.md` file otherwise) must not exist — file or directory, empty or not. Deliberate overwrite is a TODO flag, not a default.
- **tool** — any value other than `claude` fails, pointing at the per-tool-templates TODO.

## `--tool`

Sync v1 cut `--tool` because all four Tools share one copy layout. Scaffold keeps it because templates are where Tools will diverge. Frontmatter schemas and argument placeholders are per-tool facts, not per-layout ones.

v1 still ships a single template set. The set is claude-flavored. The source tree has one canonical flavor, and sync copies that flavor everywhere. `claude` is the default and the only value that succeeds. `cursor`, `codex`, and `opencode` parse (the enum is sync's). Then the command exits non-zero and names the missing template set. A failure keeps the flag honest. A silent claude file for `--tool cursor` is a false result.

When non-claude templates land, their formats must match current tool docs at implementation time. Design from memory is out of scope (see TODO).

## CLI

```
atb scaffold --name <name> [--kind skill|command|agent] [--tool claude|cursor|codex|opencode]
             [--dir <path>] [--description <text>]
```

`--kind` defaults to `skill` (sync's default). `--dir` defaults to `.`. `--tool` defaults to `claude`. On success, print each created file as `created {path}`. Paths are as constructed from `--dir` as given, with no canonicalization. This mirrors sync's plan print. Exit non-zero on: invalid name, missing or non-directory `--dir`, existing destination, non-claude `--tool`, or a failed write.

## Rust API

`ScaffoldSpec` is this command's input record, not a domain noun, so it is defined here and nowhere else ([why](domain-model.md#deliberately-not-modeled)). Its `kind` and `tool` are sync's enums, defined in the [domain model](domain-model.md). `name` carries the [`Artifact` identity rule](domain-model.md#identity). Defaults are the [CLI](#cli)'s.

```rust
pub struct ScaffoldSpec {
    pub kind: Kind,
    pub tool: Tool,
    pub name: String,
    pub description: Option<String>,
    pub dir: PathBuf,
}

pub fn scaffold(spec: &ScaffoldSpec) -> Result<Vec<PathBuf>>;
```

`scaffold` runs the checks, renders the template, writes the file, and returns the created paths. The binary prints them. `Kind` and `Tool` are sync's enums. Scaffold adds no types beyond `ScaffoldSpec` and no `FileOp` variant (see Cut). Every v1 kind writes exactly one file. The `Vec` is for kinds that will not (plugin, TODO). `Result` is `anyhow::Result`, as everywhere.

## Crate layout

- `src/scaffold.rs` — `ScaffoldSpec`, the three templates, `scaffold()`
- `src/main.rs` — add the `scaffold` subcommand
- No new deps: the name rule is the regex above checked with a hand-rolled `chars()` loop, not a `regex` dep.

## Platform

Unix only, same stance as [atb-sync](atb-sync.md).

## Tests (thin)

Scaffold a skill in a temp dir: `skills/foo/SKILL.md` exists and `discover(dir, Skill)` returns a Catalog whose only new id is `foo` with `meta.name == Some("foo")` (the round-trip law). Scaffold a command: `commands/critique.md` exists, discovered id `critique.md`. One test that `--description` text with a `:` and quotes still yields parseable frontmatter. Error paths: pre-existing `skills/foo` fails and writes nothing. Bad names fail (`Foo`, `foo_bar`, `-foo`, `foo-`, empty, 65 chars). `--tool cursor` fails. A missing `--dir` fails.

## TODO (considered, not committed)

- **Per-tool templates** — the reason `--tool` exists. A (tool × kind) template matrix. For example, cursor commands are plain markdown, opencode commands carry `description`/`agent`/`model` frontmatter, and codex has prompts rather than commands. Some cells are unsupported and must fail. Every cell must be pinned against current tool docs before implementing. This spec commits to none of them.
- **`mcp-server` kind** — registering an MCP server is an *entry* in a per-tool config (Claude's `.mcp.json`, Cursor's `mcp.json`, Codex's `config.toml`): a JSON/TOML merge job, not a file birth — the same grain mismatch that kept `McpServerSpec` out of sync v1. "Scaffold an MCP server" can also mean generating a server *project* (a package with a manifest and entry point), a different noun entirely. Needs its own spec that first decides which noun it means.
- **`plugin` kind** — the source tree is plugin-shaped (`plugins/*/{skills,commands,agents}`); scaffolding `plugins/{name}/` with a `.claude-plugin/plugin.json` manifest and empty kind dirs is the natural next kind up, and the reason `scaffold()` returns a `Vec`.
- **Scaffolding into tool config dirs** — v1 targets the source tree and lets sync fan out. For Claude the two coincide — a project's `.claude/` uses the same `{skills,commands,agents}` layout, so `--dir .claude` already works — but other tools have their own conventions (`.cursor/commands/`, …); a mode mapping `--tool` to those destinations would make scaffold useful without a source tree.
- **`--force`** — overwrite an existing destination. Cut until someone actually wants it; collision-is-error is the safe default.
- **Template overrides** — a user templates dir (`~/.config/atb/templates/…`) shadowing the built-ins.

## Cut from v1 (rationale)

| Item | Why it goes |
|---|---|
| Template engine (handlebars etc.) | Two `format!` substitutions |
| Interactive wizard / prompts | Five flags cover it, and scaffold must stay scriptable |
| Richer frontmatter stubs (`argument-hint`, `allowed-tools`, `model`, `tools`) | The stub's job is to parse and trigger; optional fields are the author's first edit, not the generator's |
| `FileOp::Write` / reusing `SyncPlan` | Scaffold writes one file; a shared plan adds a variant sync never emits |
| Kind-as-subcommand (`atb scaffold skill <name>`) | One grammar across verbs: flags, like sync |
| Cross-kind name collision check | Ids are per-kind in discovery too |

## Out of scope

Editing or linting existing artifacts (a future `check`'s job), MCP, rules, hooks, plugin manifests, marketplace publishing, per-tool SKILL.md rewriting, Windows, any change to sync behavior.
