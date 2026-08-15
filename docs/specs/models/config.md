# `Config`, `Target`

The configuration side of sync: one job describing what to sync and where. A
`Config` is built two ways — from `--src` / `--dst` / `--kind` flags, or from a
YAML file — but both produce the same shape defined here. The YAML surface
syntax and its validation live in [config-spec](../config-spec.md); this file
defines the resulting data.

## `Config`

One sync job: a single `Kind` discovered under one `source`, fanned out to one
or more `Target`s.

| Field | Type | Required | Notes |
|---|---|---|---|
| `kind` | [`Kind`](enumerations.md#kind) | no | The one category this job syncs. `default: Skill`. |
| `source` | `Path` | yes | Root of the tree discovery walks. |
| `targets` | `List<`[`Target`](#target)`>` | yes | Where to write; **non-empty** — a config with no targets is an error. |

**One `Kind` per `Config`.** A job syncs exactly one category. Syncing skills
and commands is two jobs, not one config with a `kinds:` list — multi-kind
routing is explicitly out of scope (see [config-spec](../config-spec.md#out-of-scope)).

**Two ways to build one.** The flag path (`--src` / `--dst` / `--kind`) yields a
`Config` with a single `Target` whose `tool` is empty. The YAML path yields a
`Config` with one `Target` per `targets` entry, each `tool` set from its key.
The two input paths are mutually exclusive; the resulting `Config` is identical
in shape.

## `Target`

One output destination for a sync, optionally labeled with the `Tool` it belongs
to.

| Field | Type | Required | Notes |
|---|---|---|---|
| `tool` | [`Tool`](enumerations.md#tool)`?` | no | The harness this destination is for. Set from the YAML `targets.<tool>` key; empty on the flag path. |
| `output` | `Path` | yes | Directory artifacts are written under. From `--dst`, or `targets.<tool>.output`. |

`tool` being optional is the seam between the two input paths: the flag path has
no place to name a tool, so it leaves `tool` empty; the YAML path always names
one via the map key. In v1 the label is not read by the copy (all tools share
one layout) — it is retained so a future per-tool adapter can branch on it, and
so a plan can report which destination it targets.

**Where used**

- **sync** — a `Config` is the entry point of the pipeline; each `Target`
  becomes one [`SyncPlan`](sync-plan.md#syncplan).
- **config** — the YAML schema deserializes into `Config` + `Target`. Tilde
  (`~/`) expansion on `source` and `output` happens on this path; see
  [config-spec](../config-spec.md#schema).
