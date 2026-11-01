# Delivery — VISUALIZER.Graphing

How it is built, tested and shipped, and where it lands.

← [Wiki home](../README.md)

| Step | How | Automated? |
|---|---|---|
| Build | `cargo build --release` (crate `oa-graph-layout`, binary `oa_graph_layout`). In the monorepo the root `.cargo/config.toml` sends every build to the root `target/`. | No — no image and no CI job builds it. |
| Test | `cargo test` — three unit tests in `src/main.rs` plus the contract test `tests/contract.rs` (binary on `tests/fixtures/`). Checked 2026-10-30: 3 + 2 passed. | No CI in this repo (no `.github/`); `cargo test` is the one command. |
| Smoke | the monorepo's smoke run writes its result to [SMOKE.md](../%F0%9F%A7%A9Manual%20and%20Artifacts/SMOKE.md). | Run from the monorepo, by hand. |
| Source | https://github.com/LikeDotAudio/APK.softapp.VISUALIZER.Graphing — a submodule at `PODS/POD.THIN/Softapps/VISUALIZER/VISUALIZER.Graphing`. | Pushed by the monorepo's `.apk.scripts/deploy_all.py`. |

## Where it lands

- **WEB.Frontend-Assets image** copies the whole `PODS/POD.THIN/Softapps` tree to
  `/srv/www/Softapps`, so this source is in it — but its nginx answers 404 for
  `.rs`, `.toml` and `.lock` (`PODS/POD.THIN/WEB.Frontend-Assets/Docker/nginx.conf`,
  PLAN-3422.01), and there is no page to serve. Only `README.md` and
  `display.json` are reachable.
- **NODE.BareMetal image** copies the Softapps tree into its build context for an
  unrelated crate; it does not build this one.
- **OS application menu**: not listed — `display.json` says `"menu": false` and the
  walker honours it (WEB.OsApi `accd14c`). No live URL, no frame, no topic.

## Not automated

Building the binary, running the tests, and any release artifact. There is no
live URL: the engine is retired and nothing calls it.

## Delivered from the 2026-10-30 audit

- **Retirement settled in the manifest** — `display.json` drops `kind`, `entry`, `tech`, `class`, `autostart` and the publisher-less `topic` `APK.audio/Gui/Display/Graphing_Engine`, is named "(retired)" and says `"menu": false`. The crate stays (reversible; the cloud could be wired back to it). Taking the folder out of the OS application menu needs WEB.OsApi's walker to honour `menu: false` (it lists any folder with a `Cargo.toml`), and the `SCANALYZER.Like.audio/graphing_rs` symlink is the Scanalyzer's to remove — both handoffs filed. Verified: `python3 -m json.tool display.json`.
- **I/O contract as a fixture pair** — `tests/fixtures/cloud.request.json` / `cloud.response.json` (axis by field, group/subgroup depth with an echoing subgroup folded, categorical timbre, size legend, feature label) and `tests/contract.rs`, which runs the built binary on every `*.request.json` and compares stdout to 1e-9, plus a malformed-request exit-1 check. Verified: `cargo test --offline` — 3 unit + 2 contract pass; a hand-corrupted response fixture made `fixtures_round_trip` fail.
- **Build output path agrees** — README and ARCHITECTURE both say `<target>/release/oa_graph_layout`, where `<target>` is what `cargo metadata` reports (root `target/` in the monorepo via root `.cargo/config.toml`, `./target/` standalone), and run it through `cargo metadata … | jq -r .target_directory`. Verified by running that command.
- **"One implementation" claim corrected** — README Philosophy and PRINCIPLES now say the depth levels exist once *in this crate* and that the browser cloud (`SCANALYZER.Like.audio/Web_Front/src/components/CloudTab/`) is a second, live copy while this one is retired.
- **PR-04 / CC-06 Generated lane indexes current** (from the SDK/contracts audit) — the lane `INDEX.md` files and `Documentation/README.md` lane counts in HEAD were stale (e.g. Philosophy row title vs the page's H1, Audits/Roadmap counts 0 after the audit pages landed); regenerated with `python3 .apk.scripts/wiki.py <path>` and committed. Verified: `wiki.py --check` on all seven VISUALIZER repos — 0 with problems.
- **WS-03 No declared topic without a publisher** (from the SDK/contracts audit) — `APK.audio/Gui/Display/Graphing_Engine` is gone: the `topic` key was removed from `display.json` (the engine has no bus client); see the bullets above. Verified: `grep -rn` over `PODS` for the topic finds only docs that record its removal.
- **Cross-repo follow-ups closed** — WEB.OsApi's walker now skips a folder whose own `display.json` says `"menu": false` (`accd14c`), and SCANALYZER.Like.audio removed its `graphing_rs` symlink, listing the crate in `SUITE/suite.json` instead (`1dc4f43`). Verified: `apps.project_apps()` from `WEB.OsApi/SRC` no longer lists VISUALIZER.Graphing; `graphing_rs` is absent from the Scanalyzer checkout.

## Releases
- **Released `2.0.0`, tag `v2.0.0`** (2026-11-01, ROAD-3452, the owner's V2) — no SDK pin; every manifest says 2.0.0; GitHub release `v2.0.0`.
