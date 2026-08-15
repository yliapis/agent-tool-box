# `SyncPlan`, `FileOp`

The planning side of sync: the concrete filesystem work computed for one target,
printed before it is applied. Plans are pure data — computing them touches
nothing on disk; applying them is a separate step.

## `FileOp`

«sum» — a single filesystem operation. v1 has exactly one variant.

| Variant | Fields | Meaning |
|---|---|---|
| `Copy` | `from: Path`, `to: Path` | **Postcondition:** after apply, `to` exists holding the content `from` resolves to (a symlinked `from` is materialized — the target's content is written, not the link). *How* apply achieves this — parent-directory creation, overwriting — is behavior: [atb-sync](../atb-sync.md#apply). |

`FileOp` is a union so the vocabulary can grow (a delete for a future `clean`, a
region write for merge-style targets) without changing `SyncPlan`. In v1 every
op is a `Copy`; no other variant exists, and nothing is ever deleted.

## `SyncPlan`

The `FileOp`s computed for one [`Target`](config.md#target).

| Field | Type | Required | Notes |
|---|---|---|---|
| `target` | [`Target`](config.md#target) | yes | The destination this plan writes to — the config's own `Target`, referenced, not owned. See [Invariants](#invariants). |
| `ops` | `List<`[`FileOp`](#fileop)`>` | yes | The operations to perform, in order. |

## Invariants

- **A plan is a derived relation.** Planning a `Config` yields exactly one
  `SyncPlan` per target, in order: `plans[i].target` *is* `config.targets[i]` —
  the same value, not an independent entity. The class diagram draws this as a
  reference rather than a composition for that reason.
- **`ops` ordering is meaningful.** Apply executes `ops` in order and stops at
  the first failure. The ordering *behavior* lives in
  [atb-sync](../atb-sync.md#apply); the order itself is data.
- **`ops` is non-empty in v1.** Discovery errors on zero matches, and every
  artifact expands to at least one op under the
  [`KindLayout` copy shape](kind-layout.md#the-mapping) (a skill directory
  contains at least its `SKILL.md`), so a constructed plan always has work. The
  type still permits empty so a future filtered discovery doesn't change the
  shape.

**Plan then apply.** A plan is computed and printed (each `Copy` as
`from -> to`) before any write happens, so the printed plan and the writes
cannot diverge. The op count per artifact is the
[`KindLayout` copy shape](kind-layout.md#the-mapping): one `Copy` per file under
a skill directory, a single `Copy` for a command or agent.

**Where used**

- **sync** — `plan` returns one `SyncPlan` per target; `apply` consumes them.
