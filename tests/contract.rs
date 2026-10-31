//! The I/O contract of `oa_graph_layout`, as fixtures a caller can test against.
//!
//! `tests/fixtures/<name>.request.json` goes in on stdin; the binary's stdout
//! must equal `tests/fixtures/<name>.response.json` (numbers compared to 1e-9).
//! A malformed request must exit 1 with `{"error": ...}` on stdout.
//! Change a fixture only together with the behaviour it pins.

use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

use serde_json::Value;

fn run(stdin: &str) -> (i32, String) {
    let mut child = Command::new(env!("CARGO_BIN_EXE_oa_graph_layout"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .expect("spawn oa_graph_layout");
    child
        .stdin
        .take()
        .unwrap()
        .write_all(stdin.as_bytes())
        .unwrap();
    let out = child.wait_with_output().unwrap();
    (
        out.status.code().unwrap_or(-1),
        String::from_utf8(out.stdout).unwrap(),
    )
}

fn same(a: &Value, b: &Value, at: &str) {
    match (a, b) {
        (Value::Number(x), Value::Number(y)) => {
            let (x, y) = (x.as_f64().unwrap(), y.as_f64().unwrap());
            assert!((x - y).abs() < 1e-9, "{at}: {x} != {y}");
        }
        (Value::Array(x), Value::Array(y)) => {
            assert_eq!(x.len(), y.len(), "{at}: length");
            for (i, (p, q)) in x.iter().zip(y).enumerate() {
                same(p, q, &format!("{at}[{i}]"));
            }
        }
        (Value::Object(x), Value::Object(y)) => {
            let mut kx: Vec<_> = x.keys().collect();
            let mut ky: Vec<_> = y.keys().collect();
            kx.sort();
            ky.sort();
            assert_eq!(kx, ky, "{at}: keys");
            for k in kx {
                same(&x[k], &y[k], &format!("{at}.{k}"));
            }
        }
        _ => assert_eq!(a, b, "{at}"),
    }
}

#[test]
fn fixtures_round_trip() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    let mut checked = 0;
    for entry in std::fs::read_dir(&dir).unwrap() {
        let path = entry.unwrap().path();
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        let Some(stem) = name.strip_suffix(".request.json") else {
            continue;
        };
        let request = std::fs::read_to_string(&path).unwrap();
        let expected: Value = serde_json::from_str(
            &std::fs::read_to_string(dir.join(format!("{stem}.response.json"))).unwrap(),
        )
        .unwrap();
        let (code, stdout) = run(&request);
        assert_eq!(code, 0, "{stem}: exit code");
        let got: Value = serde_json::from_str(&stdout).unwrap();
        same(&got, &expected, stem);
        checked += 1;
    }
    assert!(checked > 0, "no fixtures in {}", dir.display());
}

#[test]
fn malformed_request_exits_1_with_error() {
    let (code, stdout) = run("not json");
    assert_eq!(code, 1);
    let got: Value = serde_json::from_str(&stdout).unwrap();
    assert!(got["error"].as_str().unwrap().starts_with("malformed request"));
}
