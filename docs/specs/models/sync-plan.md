# `SyncPlan`, `FileOp`

The planning side of sync: the concrete filesystem work computed for one target,
printed before it is applied. Plans are pure data — computing them touches
nothing on disk; applying them is a separate step.

## `FileOp`

«sum» — a single filesystem operation. v1 has exactly one variant.

| Variant | Fields | Meaning |
|---|---|---|
| `Copy` | `from: Path`, `to: Path` | Copy the file at `from` to `to`, creating parent directories and overwriting an existing file. Symlinks are materialized — the link's target content is written, not the link. |

`FileOp` is a union so the vocabulary can grow (a delete for a future `clean`, a
region write for merge-style targets) without changing `SyncPlan`. In v1 every
op is a native `Copy`; no other variant exists, and nothing is ever deleted.

## `SyncPlan`

The `FileOp`s computed for one [`Target`](config.md#target).

| Field | Type | Required | Notes |
|---|---|---|---|
| `target` | [`Target`](config.md#target) | yes | The destination this plan writes to. |
| `ops` | `List<`[`FileOp`](#fileop)`>` | yes | The operations to perform, in order. May be empty if nothing was discovered. |

**Cardinality.** One `SyncPlan` per `Target`: planning a `Config` with N targets
yields N plans. The op count depends on `Kind` — a `Skill` expands to one `Copy`
per file under the skill directory (`{output}/{id}/{relpath}`), while a
`Command` or `Agent` is a single `Copy` to `{output}/{id}`.

**Plan then apply.** A plan is computed and printed (each `Copy` as
`from -> to`) before any write happens, so the printed plan and the writes cannot
diverge. Apply walks `ops` in order and stops at the first failure; files already
written stay. This ordering behavior lives in [atb-sync](../atb-sync.md#apply) —
the model here is just the data those steps pass around.

**Where used**

- **sync** — `plan` returns one `SyncPlan` per target; `apply` consumes them.
