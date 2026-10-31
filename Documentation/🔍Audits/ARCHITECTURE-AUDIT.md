# 🔍 Architecture audit — VISUALIZER.Graphing (2026-10-30)

Audited against the APK.audio multi-repo architecture standard: coupling, boundaries, duplication, contracts.
← [Audits](INDEX.md) · Improvements: [🛣Roadmap/IMPROVEMENTS.md](../%F0%9F%9B%A3Roadmap/IMPROVEMENTS.md)

## Bounded context
`oa_graph_layout`: computes the layout of the Scanalyzer's 3D sample cloud (axis values, group colour indices, ticks, bubble sizes) from one JSON request on stdin, answered as JSON on stdout. Rust, one 334-line file (`src/main.rs`) plus a contract test, depends only on `serde` and `serde_json`. **Retired** (PLAN-109.04): nothing calls it.

**Sampled:** all tracked code (`Cargo.toml`, `src/main.rs`, `display.json`), README and wiki. Ran `cargo test --offline`: 3 unit + 2 contract tests pass (re-checked after the 2026-10-30 remediation).

## Context map
| Talks to | How (topic / endpoint / file / import) | Relationship |
|---|---|---|
| SCANALYZER.Like.audio | lists the crate by portal path in `SUITE/suite.json` (`03_ENGINES`); the old `graphing_rs` symlink is gone; no caller | data reference only |
| WEB.OsApi project walker | `display.json` `"menu": false` keeps it out of the application menu | Conformist |
| any caller (none today) | stdin/stdout JSON, pinned by `tests/fixtures/*.request.json` → `*.response.json` | Published Language (fixture) |

## Scorecard
| Category | Score (0–3) | Finding |
|---|---|---|
| Coupling & Cohesion | 3 | Self-contained pipe with no outward references; no file reference into it remains (the Scanalyzer lists it as data). |
| Boundary Integrity | 3 | Its I/O types are serde structs (`Request`, `Response`) separate from the computation, and malformed input is refused with an error. |
| Duplication Control | 2 | The browser cloud (`SCANALYZER.Like.audio/Web_Front/src/components/CloudTab/`) does the same job; the docs now say so and this copy is marked retired. One copy remains the cost of keeping the crate reversible. |
| Testing & Contracts | 3 | 3 unit tests plus `tests/contract.rs` running the binary on a request/response fixture pair; `cargo test` is the one command. |
| Change management | 3 | Retired in the manifest (no `kind`/`entry`/`topic`, `"menu": false`, "(retired)" name) and out of the menu; crate `0.1.0`, no tags — fine for a retired, uncalled crate. |
Score key: 3 meets the standard · 2 minor gaps · 1 significant gaps · 0 absent / violated.

## Findings
1. **Good: a clean pipe with refusal over fabrication.** `refuse()` / `main()` (`src/main.rs:281-304`) write `{"error": ...}` and exit 1 when input is malformed; missing fields default through `#[serde(default)]` (`src/main.rs:26-45`). The tests `malformed_request_is_an_error_not_an_empty_layout` and `malformed_request_exits_1_with_error` pin this behaviour.
2. **Good: the contract is a fixture.** `tests/fixtures/cloud.request.json` / `cloud.response.json`, checked by `tests/contract.rs`.
3. **Closed across repos:** WEB.OsApi honours `"menu": false` (`accd14c`) and the Scanalyzer's `graphing_rs` symlink is gone (`1dc4f43`).
