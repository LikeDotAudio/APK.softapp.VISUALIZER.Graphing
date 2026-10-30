# Delivery — VISUALIZER.Graphing

How it is built, tested and shipped, and where it lands.

← [Wiki home](../README.md)

| Step | How | Automated? |
|---|---|---|
| Build | `cargo build --release` (crate `oa-graph-layout`, binary `oa_graph_layout`). In the monorepo the root `.cargo/config.toml` sends every build to the root `target/`. | No — no image and no CI job builds it. |
| Test | `cargo test` — three unit tests in `src/main.rs`. Checked 2026-10-30: 3 passed. | No CI in this repo (no `.github/`). |
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
- **OS application menu** lists it from `display.json` (no live URL, no frame).

## Not automated

Building the binary, running the tests, and any release artifact. There is no
live URL: the engine is retired and nothing calls it.
