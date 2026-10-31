# Architecture — VISUALIZER.Graphing

How to use `oa_graph_layout` and how it works inside.

← [Wiki home](../README.md)

## Status

Retired from the Scanalyzer (PLAN-109.04). The Scanalyzer's `graphing_rs` symlink to
this repo is gone (it lists the crate in `SUITE/suite.json` as data), and the 3D cloud now lays itself out in the browser
(`SCANALYZER.Like.audio/Web_Front/src/components/CloudTab/`). Nothing in the tree
runs the binary. `display.json` carries no `kind`, `entry` or `topic` and says
`"menu": false`, which `walk_project()` (`PODS/POD.THIN/WEB.OsApi/SRC/osapi/apps.py`)
honours, so the folder is out of the application menu despite its `Cargo.toml` marker.
Kept rather than deleted so the move back is a caller, not an archaeology dig.

## Use

```bash
cargo build --release            # → <target>/release/oa_graph_layout
echo '{"axes":{"x":"pitch","y":null,"z":"timbre"},
       "records":[{"group":"Drums","subgroup":"Kick","pitch":60,"timbre":"dark","length":1.2},
                  {"group":"Bass","pitch":40,"timbre":"warm","length":0.5}]}' \
  | "$(cargo metadata --format-version 1 --no-deps | jq -r .target_directory)/release/oa_graph_layout"
```

`<target>` is what `cargo metadata` reports: the APK.audio root `target/` inside
the monorepo (root `.cargo/config.toml`), `./target/` in a standalone checkout.
The README says the same.

| Stream | Shape |
|---|---|
| stdin | `{ axes:{x,y,z}, size, feature_labels, records }` — every field optional |
| stdout (ok, exit 0) | `{ groups, color_idx, x, y, z, sizes, size_label, size_legend }` |
| stdout (refused, exit 1) | `{ "error": "malformed request: ..." }`, same text on stderr |

**The contract is the fixture pair**, not this table:
`tests/fixtures/<name>.request.json` → `tests/fixtures/<name>.response.json`.
`tests/contract.rs` runs the built binary on every request fixture and compares
its stdout (numbers to 1e-9), and checks a malformed request exits 1 with
`{"error": ...}`. Add a pair to pin a new behaviour.

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
| `tests/contract.rs` | — | runs the binary on `tests/fixtures/*.request.json`, compares with `*.response.json`; malformed → exit 1 |

Data flow: stdin → `serde_json::from_str::<Request>` (error → `refuse`) →
`compute` → `serde_json::to_string` → stdout.

Dependencies: `serde`, `serde_json` only (`Cargo.toml`).
