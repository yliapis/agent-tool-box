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
| `kind` | [`Kind`](enumerations.md#kind) | yes | The one category this job syncs. Every constructed `Config` carries one; both input surfaces default an omitted kind to `skill` ([CLI](../atb-sync.md#cli), [YAML](../config-spec.md#schema)). |
| `source` | `Path` | yes | Root of the tree discovery walks. |
| `targets` | `List<`[`Target`](#target)`>` | yes | Where to write. **Non-empty** — see [Invariants](#invariants). |

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
| `tool` | [`Tool`](enumerations.md#tool)`?` | no | The harness this destination is for. Constrained per-`Config` — see [Invariants](#invariants). |
| `output` | `Path` | yes | Directory artifacts are written under. From `--dst`, or `targets.<tool>.output`. |

In v1 the `tool` label is not read by the copy (all tools share one layout) —
it is retained so a future per-tool adapter can branch on it, and so a plan can
report which destination it targets.

## Invariants

- **`targets` is non-empty.** A config with no targets is an error
  ([config-spec](../config-spec.md#validation)).
- **One `Kind` per `Config`.** A job syncs exactly one category. Syncing
  skills and commands is two jobs, not one config with a `kinds:` list —
  multi-kind routing is explicitly out of scope
  ([config-spec](../config-spec.md#out-of-scope)).
- **`tool` labeling is coherent.** Within one `Config`, exactly two
  configurations occur: **every** target carries a `tool` (the YAML path —
  each `targets.<tool>` key labels its entry), or `targets` is a **single**
  element whose `tool` is empty (the flag path — the flags have no place to
  name a tool). A mixed list, or a multi-target list with unlabeled entries,
  is never constructed and has no defined meaning. This is the reason `tool`
  is optional at all: the optionality encodes the flag path's missing label,
  not a per-target free choice.

**Where used**

- **sync** — a `Config` is the entry point of the pipeline; each `Target`
  becomes one [`SyncPlan`](sync-plan.md#syncplan).
- **config** — the YAML schema deserializes into `Config` + `Target`. Tilde
  (`~/`) expansion on `source` and `output` happens on this path; see
  [config-spec](../config-spec.md#schema).
