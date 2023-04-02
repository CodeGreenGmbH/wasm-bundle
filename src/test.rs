use std::{env::var, fs, path::Path, process::Command};

use crate::{exec, run, Opt};

fn bundle_example(crate_name: &str, opt: Opt) {
    let base_dir = Path::new("examples").join(crate_name);
    cargo_clean(&base_dir);
    exec(&base_dir, &opt);
    let mut html_path = base_dir.join("target/wasm-bundle/debug");
    if let Some(example) = &opt.example {
        html_path = html_path.join("examples").join(format!("{example}.html"));
    } else {
        html_path = html_path.join(format!("{crate_name}.html"));
    }
    fs::copy(html_path, base_dir.join("index.html")).unwrap();
}

fn cargo_clean(base_dir: &Path) {
    let mut cmd = Command::new(var("CARGO").unwrap_or("cargo".into()));
    cmd.arg("clean");
    run(base_dir, cmd)
}

#[test]
fn wasm_counter() {
    bundle_example("wasm-counter", Opt::default())
}

#[test]
fn yew_counter() {
    bundle_example("yew-counter", Opt::default())
}

#[test]
fn library() {
    let mut opt = Opt::default();
    opt.example = Some("demo".to_string());
    bundle_example("library", opt)
}
