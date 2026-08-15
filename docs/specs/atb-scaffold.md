# `atb scaffold` — spec

Normative spec for the second `atb` verb. `atb scaffold` creates the skeleton of a new artifact — a skill, command, or agent — in the layout [`atb sync`](atb-sync.md) discovers, so the two verbs compose: scaffold into the source tree, sync out to the tools. Sync's v1 cut every verb but `sync` and cut `--tool` for lack of divergence; scaffold is where a second verb and `--tool` come back. It reuses sync's `Kind` and `Tool` and changes nothing about sync. Iterate here.

## Behavior

```
atb scaffold --name code-review
atb scaffold --kind command --name critique --dir ~/src/dotfiles/ai-coding/plugins/core
```

The first writes `./skills/code-review/SKILL.md`; the second writes `…/plugins/core/commands/critique.md`, which the next `atb sync` picks up. **Round-trip invariant**: immediately after a successful scaffold, `discover(dir, kind)` succeeds and includes exactly the new artifact, with the `id` and `meta` fields the [round-trip law](models/README.md#the-round-trip-law) states — that statement is canonical, including its collision proviso and the fact that the command template writes no `name:` frontmatter (so a command's `meta.name` stays `None`).

Scaffold never overwrites: an existing destination is an error and nothing is written. `--tool` (default `claude`) selects the template flavor; v1 ships claude templates only, and any other value exits non-zero (see [`--tool`](#--tool)).

## Layout and templates

The path each `kind` creates, and the `id` it discovers as, are the
scaffold-path and id-from-`name` rows of the
[`KindLayout` mapping](models/kind-layout.md#the-mapping); the templates below
are the content written at those paths.

Intermediate directories (`skills/`, the skill dir, `commands/`, `agents/`) are created with `create_dir_all`; `{dir}` itself must already exist — a missing `--dir` is more likely a typo than intent.

Templates are `const` strings in the binary with `{name}` / `{description}` filled by `format!` — no template engine, no user template dirs. The description is written as a YAML double-quoted scalar (escaping `\`, `"`, and newlines), so any `--description` text yields parseable frontmatter; `name` needs no quoting by construction (see Validation). When `--description` is absent, the per-kind placeholder below stays in.

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

The frontmatter must parse under sync's frontmatter reader (first `---` / `---` pair, YAML): the fields a template writes never discover as `None`. (The command template writes only `description:`, so a command's `meta.name` is `None` by design, not by parse failure.)

## Validation

Every problem is an error, and all checks precede the first write — an error writes nothing.

- **name** — `^[a-z0-9]([a-z0-9-]*[a-z0-9])?$`, at most 64 chars. This is the strictest of the four tools' naming rules (Claude's skill-name constraint), so a scaffolded name is legal everywhere, filesystem-safe, and needs no YAML quoting. The name is the stem; scaffold appends `.md` itself, so `--name critique.md` is invalid, not redundant.
- **dir** — `--dir` (default `.`) must exist and be a directory.
- **collision** — the artifact path (`{dir}/skills/{name}` for skills; the `.md` file otherwise) must not exist — file or directory, empty or not. Deliberate overwrite is a TODO flag, not a default.
- **tool** — any value other than `claude` fails, pointing at the per-tool-templates TODO.

## `--tool`

Sync v1 cut `--tool` because all four tools share one copy layout. Scaffold keeps it because templates are exactly where tools will diverge: frontmatter schemas and argument placeholders are per-tool facts, not per-layout ones. v1 still ships a single template set — claude-flavored, because the source tree has one canonical flavor and sync copies that flavor everywhere — so `claude` is the default and the only value that succeeds. `cursor`, `codex`, `opencode` parse (the enum is sync's) but exit non-zero naming the missing template set: failing keeps the flag honest, where silently emitting claude files for `--tool cursor` would not. When non-claude templates land, their formats must be verified against current tool docs at implementation time, not designed from memory (see TODO).

## CLI

```
atb scaffold --name <name> [--kind skill|command|agent] [--tool claude|cursor|codex|opencode]
             [--dir <path>] [--description <text>]
```

`--kind` defaults to `skill` (sync's default); `--dir` to `.`; `--tool` to `claude`. On success, print each created file as `created {path}` — paths as constructed from `--dir` as given, no canonicalization — mirroring sync's plan print. Exit non-zero on: invalid name, missing or non-directory `--dir`, existing destination, non-claude `--tool`, or a failed write.

## Rust API

`ScaffoldSpec` is the Rust binding of the [`ScaffoldSpec` domain model](models/scaffold-spec.md); `Kind` and `Tool` are sync's enums, defined language-agnostically in [enumerations](models/enumerations.md). The block is deliberately comment-free — field semantics live in the models (see [normativity](models/README.md#normativity)); defaults are the [CLI](#cli)'s.

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

`scaffold` validates, renders, writes, and returns the created paths; the binary prints them. `Kind` and `Tool` are sync's enums — scaffold adds no types beyond `ScaffoldSpec` and no `FileOp` variant (see Cut). Every v1 kind writes exactly one file; the `Vec` is for kinds that won't (plugin, TODO). `Result` is `anyhow::Result`, as everywhere.

## Crate layout

- `src/scaffold.rs` — `ScaffoldSpec`, the three templates, `scaffold()`
- `src/main.rs` — add the `scaffold` subcommand
- No new deps: the name rule is the regex above checked with a hand-rolled `chars()` loop, not a `regex` dep.

## Platform

Unix only, same stance as [atb-sync](atb-sync.md).

## Tests (thin)

Scaffold a skill in a temp dir: `skills/foo/SKILL.md` exists and `discover(dir, Skill)` returns exactly id `foo` with `meta.name == Some("foo")` (the round-trip invariant). Scaffold a command: `commands/critique.md` exists, discovered id `critique.md`. One test that `--description` text with a `:` and quotes still yields parseable frontmatter. Error paths: pre-existing `skills/foo` fails and writes nothing; bad names fail (`Foo`, `foo_bar`, `-foo`, `foo-`, empty, 65 chars); `--tool cursor` fails; a missing `--dir` fails.

## TODO (considered, not committed)

- **Per-tool templates** — the reason `--tool` exists. A (tool × kind) template matrix: e.g. cursor commands are plain markdown, opencode commands carry `description`/`agent`/`model` frontmatter, codex has prompts rather than commands — and some cells are unsupported and should error. Every cell must be pinned against current tool docs before implementing; this spec deliberately commits to none of them.
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
