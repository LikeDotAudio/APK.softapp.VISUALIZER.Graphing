# Principles — VISUALIZER.Graphing

What `oa_graph_layout` actually keeps, each pointed at the code that keeps it.
Ecosystem rules live in the APK.audio root `Documentation/🏛Philosophy/`; they are
cited here, not copied.

← [Wiki home](../README.md)

## Kept

1. **Thick core, soft front** (root `Documentation/🏛Philosophy/Architecture/ARCHITECTURE_THICK_AND_SOFT.md`).
   The engine owns every number — axis values, group colour indices, ticks,
   bubble sizes — and whatever draws them only plots the response.
   `compute()` (`src/main.rs:264`) builds the whole `Response`; nothing is left for
   the drawer to compute.

2. **One implementation of the depth levels.** The group / subgroup depth, the
   categorical index and the size scaling exist once each:
   `axis_values()` (`src/main.rs:124`), `deeper_sub()` (`src/main.rs:104`),
   `compute_sizes()` (`src/main.rs:208`). Every caller files a sample at the same
   depth because there is only one place that decides it.

3. **A pipe, not a service.** One JSON request on stdin, one JSON response on
   stdout, then exit — no port, no bus, no files, no config (`main()`,
   `src/main.rs:288`). Any language can call it without a server.

4. **Refusal over fabrication** (root `Documentation/🏛Philosophy/Doctrine/readme.Refusal over fabrication.md`).
   Unreadable stdin, a request that is not valid JSON of the documented shape, or
   an unencodable response goes through `refuse()` (`src/main.rs:281`): it writes
   `{"error": "..."}` on stdout, the same on stderr, and exits 1. It no longer
   returns an empty layout for bad input (fixed under
   LOOSE-visualizer-visualizer-graphing-main). Test:
   `malformed_request_is_an_error_not_an_empty_layout`.

5. **Deterministic output.** Groups, timbre categories and depth levels are
   collected in `BTreeSet` / `BTreeMap`, so the same request always yields the same
   order and the same colour index (`compute()`, `axis_values()`).

## Where the code bends a principle

- **Missing fields are not refused.** A record without `group` lands in `Other`
  (`group_of()`, `src/main.rs:97`); a missing or non-numeric feature reads as 0
  (`num()`, `src/main.rs:81`); a `length` of 0 is drawn as 0.1. That is a
  deliberate default for sparse sample records, but a plotted 0 is indistinguishable
  from a measured 0.
- **Feature names are wire constants in code.** `"timbre"`, `"complexity"`,
  `"length"` and `"channels"` are matched by name in `axis_values()` and
  `compute_sizes()`; a new categorical feature needs a code change.
