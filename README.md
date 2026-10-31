# VISUALIZER.Graphing

> 📉 **Graph Layout Engine** — `oa_graph_layout`, the Rust "math and placement"
> step for the Scanalyzer's 3D sample cloud. Retired from the app (PLAN-109.04):
> nothing in the tree calls it today.

---

## 🧩 Architectural Role
* **Family**: Scanalyzer (softapp tree `PODS/POD.THIN/Softapps/VISUALIZER/`)
* **Category**: Visualizer — layout maths, no drawing
* **Pipeline Stage**: none declared (`subGroup` is `DSP Core`, which carries no stage number)

---

## ⚙️ What it does

One binary, `oa_graph_layout` (`src/main.rs`, crate `oa-graph-layout`). It reads
**one JSON request on stdin** and writes **one JSON response on stdout**, then
exits. No ports, no MQTT, no files, no config.

```
in : { axes: {x, y, z}, size, feature_labels, records }
out: { groups, color_idx, x, y, z, sizes, size_label, size_legend }
```

- **Axes.** Each of `x`/`y`/`z` names a record field, or is `null`:
  - `null` — the hierarchical *group / subgroup* depth: each `group`, plus its
    curated `subgroup` one level deeper (a subgroup that only echoes the group
    name is folded back into it). Comes with tick positions and names.
  - `"timbre"` — categorical: sorted distinct timbres become 0, 1, 2…, with ticks.
  - `"complexity"` — the field's value, negated for mono files (`channels == 1`).
  - anything else — the record's number (`true`/`false` read as 1/0, missing as 0).
- **Colours.** `groups` is the sorted distinct `group` list (empty → `Other`);
  `color_idx` is each record's index into it.
- **Sizes.** `size` names a field (default `length`). Numeric fields scale
  min…max onto 25…285; `timbre` gets one size per category. `size_legend`
  holds the min / mid / max reference bubbles.
- **Labels.** `feature_labels` maps a field key to its display label; unknown
  keys label themselves.

There is **no** PCA and **no** k-means clustering here (an earlier
`display.json` said there was).

---

## 🚀 Build, run, test

```bash
cargo build --release          # → <target>/release/oa_graph_layout
echo '{"axes":{"x":"pitch","y":null,"z":"timbre"},
       "records":[{"group":"Drums","subgroup":"Kick","pitch":60,"timbre":"dark","length":1.2},
                  {"group":"Bass","pitch":40,"timbre":"warm","length":0.5}]}' \
  | "$(cargo metadata --format-version 1 --no-deps | jq -r .target_directory)/release/oa_graph_layout"
```

`<target>` is wherever cargo puts it: in the monorepo the root
`.cargo/config.toml` sends every build to the APK.audio root `target/`; a
standalone checkout uses `./target/`. Ask `cargo metadata` (as above) rather
than guessing.

Checked 2026-10-30: builds offline with `serde` / `serde_json` only, and the
request above returns two groups, a group/subgroup tick axis on `y` and a
three-entry size legend. `cargo test` runs three unit tests (malformed input is
an error, missing fields default, group/depth ticks) and the contract test
`tests/contract.rs`, which runs the binary on every
`tests/fixtures/<name>.request.json` and compares stdout with
`<name>.response.json`. **That fixture pair is the I/O contract**: a future
caller tests against it, and a change to the output changes the fixture in the
same commit.

---

## 🔌 Usage & Integration
* **Callers**: none. `SCANALYZER.Like.audio/graphing_rs` is a symlink to this
  repo and the Scanalyzer README marks it *retired from the app, PLAN-109.04*;
  the 3D cloud now lays itself out in the browser
  (`SCANALYZER.Like.audio/Web_Front/src/components/CloudTab/`).
* **Web Portal Entry**: none — no OS display frames it, and `display.json`
  declares no `entry`, `kind` or `topic` and says `"menu": false`. Until
  WEB.OsApi honours `menu: false`, `walk_project()`
  (`PODS/POD.THIN/WEB.OsApi/SRC/osapi/apps.py`) still lists the folder as a
  `rust` app because it holds a `Cargo.toml`; the entry's name says
  "(retired)".
* **Remote Repository**: https://github.com/LikeDotAudio/APK.softapp.VISUALIZER.Graphing

---

## 🏛 Philosophy

- **Thick core, soft front.** The engine owns the numbers (axis values,
  indices, sizes, ticks); whatever draws them just plots what comes back —
  `src/main.rs`, header comment and `compute()`.
- **The depth levels, once in Rust — but no longer the only copy.**
  Group/subgroup levels, categorical indexing and size scaling exist once in
  this crate (`axis_values()`, `compute_sizes()`). The live 3D cloud does the
  same job in the browser (`SCANALYZER.Like.audio/Web_Front/src/components/CloudTab/`),
  so while this crate is retired the two can disagree; the browser is the one
  users see.
- **A pipe, not a service.** stdin → stdout, one request per run, no port and
  no bus (`main()`), so it can be called from any language without a server.
- **Refuse, don't fabricate.** Unreadable stdin or a request that is not valid
  JSON of the documented shape writes `{"error": "..."}` on stdout (and stderr)
  and exits 1 — `refuse()` / `main()`. Only *missing* fields fall back: they
  read as 0 or `Other` (`num()`, `group_of()`).

---

## 📚 Wiki

The depth lives in [Documentation/README.md](Documentation/README.md): principles,
architecture and delivery.

---

## 🛣 Plans

None in this repository. Plans live in `Documentation/🛣Roadmap/roadmap.htm` at
the APK.audio root.

<!-- wiki:smoke -->
**Smoke test** — ✅ PASS 1/1 since 2026-10-30 12:51 EDT — [results](Documentation/%F0%9F%A7%A9Manual%20and%20Artifacts/SMOKE.md)
<!-- /wiki:smoke -->
