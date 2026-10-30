# Architecture — VISUALIZER.Graphing

How to use `oa_graph_layout` and how it works inside.

← [Wiki home](../README.md)

## Status

Retired from the Scanalyzer (PLAN-109.04). `SCANALYZER.Like.audio/graphing_rs` is a
symlink to this repo, but the 3D cloud now lays itself out in the browser
(`SCANALYZER.Like.audio/Web_Front/src/components/CloudTab/`). Nothing in the tree
runs the binary. The OS application menu still lists it as a `rust` app from
`display.json` (`walk_project()` in `PODS/POD.THIN/WEB.OsApi/SRC/server.py`).

## Use

```bash
cargo build --release            # binary lands in the APK.audio root target/ (see .cargo/config.toml there)
echo '{"axes":{"x":"pitch","y":null,"z":"timbre"},
       "records":[{"group":"Drums","subgroup":"Kick","pitch":60,"timbre":"dark","length":1.2},
                  {"group":"Bass","pitch":40,"timbre":"warm","length":0.5}]}' | oa_graph_layout
```

| Stream | Shape |
|---|---|
| stdin | `{ axes:{x,y,z}, size, feature_labels, records }` — every field optional |
| stdout (ok, exit 0) | `{ groups, color_idx, x, y, z, sizes, size_label, size_legend }` |
| stdout (refused, exit 1) | `{ "error": "malformed request: ..." }`, same text on stderr |

## Inside

One file, `src/main.rs`.

| Piece | Lines | Does |
|---|---|---|
| `Request`, `Axes` | 26–45 | serde input; `#[serde(default)]` on every field, so missing ≠ malformed |
| `Response`, `Axis`, `Ticks`, `SizeRef` | 47–78 | serde output |
| `num()`, `sstr()`, `group_of()` | 81–100 | field access; missing → 0 / `""` / `Other` |
| `deeper_sub()` | 104 | a subgroup counts only when it does not echo the group name |
| `axis_values()` | 124 | `null` → group/subgroup depth with ticks; `timbre` → categorical index with ticks; `complexity` → negated for mono (`channels == 1`); else the numeric field |
| `compute_sizes()` | 208 | numeric → min…max scaled onto 25…285 with a min/mid/max legend; `timbre` → one size per category |
| `compute()` | 264 | sorted distinct groups → `color_idx`, the three axes, the sizes |
| `refuse()`, `main()` | 281–304 | read stdin, parse or refuse, compute, write |
| `tests` | 306– | malformed input is an error; missing fields default; groups and depth ticks |

Data flow: stdin → `serde_json::from_str::<Request>` (error → `refuse`) →
`compute` → `serde_json::to_string` → stdout.

Dependencies: `serde`, `serde_json` only (`Cargo.toml`).
