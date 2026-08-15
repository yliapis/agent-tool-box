# Bare-bones `atb sync`

Repo is docs-only today. First cut is a Rust crate whose public types are the API, and whose only binary entry is `atb sync`. Work is split so the spec can be iterated before any feature code exists.

## Deliverables (in order)

1. **Spec, not code** — write `docs/specs/atb-sync.md`. Everything below the divider is its seed: it moves there, gets iterated in place, and is cut from this plan once the spec lands. From then on the spec is the source of truth; this plan tracks sequencing and open questions only.
2. **Scaffold, no logic** — `cargo init`: package `agent-tool-box`, bin `atb`, edition 2024, deps and file layout from the crate-layout section, modules stubbed. `atb sync` parses the full flag surface (`ArgGroup` mutual exclusion included) and exits non-zero with a "spec in flight" message. `cargo build` / `clippy` / `test` stay green.

No `discover` / `plan` / `apply` code until spec iteration settles.

## Open questions (settle during spec iteration)

- `--dry-run`: still cut, but `sync` overwrites files under `~/.claude` / `~/.agents` with no preview mode, and the flag is one `if` before `apply`. Keep the cut or take it back?
- Symlinks inside a skill dir: does discovery follow them, and does copy materialize them (`fs::copy` follows links) or refuse?
- Junk files inside a skill dir (`.DS_Store`, other dotfiles): copied as-is today — skip instead? Discovery skips hidden *dirs* only.
- Nested `SKILL.md` beneath an already-matched skill dir: descend (two artifacts) or stop at the first match?
- `apply` failure mid-plan: abort on first failed copy or best-effort with a summary? Already-written files stay either way.
- Config validation: empty `targets:`, typo'd target key, unknown YAML fields (`deny_unknown_fields`?) — error or ignore?
- Platform: tilde swap and paths assume Unix; state explicitly whether Windows is unsupported in v1.

---

Everything below is spec seed — destined for `docs/specs/atb-sync.md`, kept here only until that file exists.

## What the first cut does

Two equivalent ways to say the same job:

```
atb sync --config sync.yaml
atb sync --tool cursor --src ~/src/dotfiles/ai-coding --dst ~/.agents/skills
```

`--config` and the flag trio are two ways to build one `Config`, and they are mutually exclusive (a clap `ArgGroup`). "Flags override config" sounds simple but is ill-defined the moment a config has two targets — which one does `--dst` override? — so v1 refuses the mix instead of defining merge semantics. `sync` always discover → plan → apply (print the plan, then write).

Discovery walks `--src` for `**/SKILL.md`. That matches the real tree under `~/src/dotfiles/ai-coding/plugins/*/skills/<name>/SKILL.md`. A src of `.../ai-coding/skills` will find nothing today; the search root should be `ai-coding` (or `ai-coding/plugins`). Zero discovered skills is an error, not an empty success — the message should carry that search-root hint. Two skill dirs with the same name (possible across plugins) is also an error: both would land at `{dst}/{slug}/`, and silently letting the last one win hides the collision.

Each skill directory is copied as-is to `{dst}/{slug}/` (`SKILL.md`, `references/`, `LICENSE`, …). All four tools share that layout in v1. `--tool` is recorded on the `Target` so a later adapter can diverge; it does not change the copy today.

```mermaid
flowchart LR
  flags[CLI flags]
  yaml[sync.yaml]
  flags --> config[Config]
  yaml --> config
  config --> discover[discover SKILL.md]
  discover --> artifacts[Artifact]
  artifacts --> plan[Adapter.plan]
  config --> plan
  plan --> syncPlan[SyncPlan]
  syncPlan --> apply[apply]
  apply --> disk[dst slug files]
```

## Keep vs cut

**Keep (the pipeline still needs these):**

- `Tool` — closed enum: `claude`, `cursor`, `codex`, `opencode` (add variants later; no stringly `ToolId`, no `registerTool`)
- `Artifact` — one skill directory, not a kind union
- `Target` — `tool` + `output`
- `Config` — `source` + `targets[]` (flags synthesize a one-target config)
- `FileOp` — `Copy { from, to }` only
- `SyncPlan` — `target` + `ops`
- `Adapter` — `fn plan(&[Artifact], &Target) -> Vec<FileOp>`
- `discover` / `plan` / `apply` as functions (no `Engine` trait)

**Cut (redundant or unused by `atb sync`):**

| Spec item | Why it goes |
|---|---|
| `registerTool` / `ToolSpec` / open `ToolId` | Four tools, one layout; a match on `Tool` is enough |
| `ArtifactKind` (`rule`, `mcpServer`, `command`, `ignore`) | CLI is skills-only |
| `McpServerSpec` | No MCP command |
| `ArtifactMeta.targets/exclude/scope/globs/activation/order` | Not read; optional `name` / `description` from frontmatter only |
| `Source` wrapper | `PathBuf` + `Vec<Artifact>` is enough |
| `TargetConfig.merge` / `writeRegion` / `activation` / `options` | Skills are whole-dir copies, not AGENTS.md regions |
| Fidelity matrix + plan warnings | Every v1 op is a native copy |
| `Adapter.parse` / `importFrom` | No import command |
| `Manifest` / `check` / `clean` / `delete` ops | No drift or cleanup command yet |
| `kinds:` routing, `conflict`, `clean`, `scope` | No second kind and no lockfile |
| `generate` / `check` / `import` / `clean` CLI | One verb: `sync` |

## Rust types (the API)

Single crate, `edition = "2024"` (stable since Rust 1.85; nothing here needs 2021). Package `agent-tool-box`, bin name `atb`. `Result` is `anyhow::Result` throughout — no custom error enum in v1.

```rust
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Tool { Claude, Cursor, Codex, OpenCode }

pub struct Artifact {
    pub id: String,              // directory name, e.g. "stop-slop"
    pub source_dir: PathBuf,     // folder that contains SKILL.md
    pub meta: ArtifactMeta,
}

pub struct ArtifactMeta {
    pub name: Option<String>,
    pub description: Option<String>,
}

pub struct Target {
    pub tool: Tool,
    pub output: PathBuf,         // --dst / targets.<tool>.output
}

pub struct Config {
    pub source: PathBuf,
    pub targets: Vec<Target>,
}

pub enum FileOp {
    Copy { from: PathBuf, to: PathBuf },
}

pub struct SyncPlan {
    pub target: Target,
    pub ops: Vec<FileOp>,
}

pub trait Adapter {
    fn plan(&self, artifacts: &[Artifact], target: &Target) -> Vec<FileOp>;
}

pub fn discover(source: &Path) -> Result<Vec<Artifact>>;
pub fn plan(artifacts: &[Artifact], targets: &[Target]) -> Vec<SyncPlan>;
pub fn apply(plans: &[SyncPlan]) -> Result<()>;
```

`SkillCopy` is the only adapter: for each artifact, emit a `Copy` for every file under `source_dir` to `output.join(id).join(relpath)`. `plan()` is a `Tool -> &dyn Adapter` match that currently returns that same adapter for every variant.

Config YAML (no `version`, `defaults`, `kinds`, `merge`):

```yaml
source: /Users/yliapis/src/dotfiles/ai-coding
targets:
  cursor:
    output: ~/.agents/skills
  claude:
    output: ~/.claude/skills
```

Tilde expansion on `source` and `output`: a manual `~/` prefix swap using `std::env::home_dir()` (un-deprecated since Rust 1.87) — no `shellexpand` dep. `serde` + `serde_yaml_ng` for load (`serde_yaml` is archived/unmaintained; `serde_yaml_ng` is the API-compatible fork); `clap` derive for the CLI.

## Crate layout

- [Cargo.toml](../../Cargo.toml) — deps: `clap` (derive), `serde`, `serde_yaml_ng`, `walkdir`, `anyhow`. Frontmatter: split on the first `---` / `---` and parse the YAML block; no extra crate unless that proves painful. Missing or malformed frontmatter is fine — `meta` fields just stay `None`.
- [src/lib.rs](../../src/lib.rs) — re-export the types and the three functions
- [src/model.rs](../../src/model.rs) — types above
- [src/config.rs](../../src/config.rs) — YAML + flag merge → `Config`
- [src/discover.rs](../../src/discover.rs) — `walkdir` for `SKILL.md`; `id` = parent dir name; skip hidden dirs
- [src/adapter.rs](../../src/adapter.rs) — `Adapter` + `SkillCopy`
- [src/apply.rs](../../src/apply.rs) — `create_dir_all` + `fs::copy`; overwrite existing files, never delete (stale files in `dst` are the future `clean`'s problem)
- [src/main.rs](../../src/main.rs) — `atb sync` only
- [README.md](../../README.md) — crate purpose + the two invocations

CLI shape:

```
atb sync --config <path>
atb sync --tool <claude|cursor|codex|opencode> --src <dir> --dst <dir>
```

Exactly one of `--config` or the flag trio; all three of `--tool`, `--src`, `--dst` are required when `--config` is absent. Print each `Copy` as `from -> to`, then apply. Exit non-zero on: mixing `--config` with the trio, missing paths, unknown `--tool`, zero skills discovered, or duplicate skill ids.

## Tests (thin)

Fixture with two skill dirs (one with a `references/` file). Assert `discover` ids, assert plan destinations are `{dst}/{id}/SKILL.md`, assert `apply` writes both files. One config-parse test for the YAML above. Two error-path tests: a src with no `SKILL.md` fails with the search-root hint, and duplicate skill ids fail discovery.

## Out of scope

Per-tool SKILL.md rewriting, `compatibility:` filtering, region markers, MCP/rules/commands, plugin registry, lockfile, dry-run flag, flag-over-config merge semantics, deleting stale files from `dst`, `check` / `import` / `clean`.
