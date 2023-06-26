use std::{env::var, fs, path::Path, process::Command};

use crate::{exec, run, Opt};

fn bundle_example(workspace: Option<&str>, crate_name: &str, opt: Opt) {
    let mut base_dir = Path::new("examples").to_path_buf();
    if let Some(workspace) = workspace {
        base_dir = base_dir.join(workspace);
    }
    base_dir = base_dir.join(crate_name);
    cargo_clean(&base_dir);
    exec(&base_dir, &opt);
    let mut html_path = base_dir.clone();
    if workspace.is_some() {
        html_path = html_path.parent().unwrap().to_path_buf();
    }
    html_path = html_path.join("target/wasm-bundle/debug");
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
    bundle_example(None, "wasm-counter", Opt::default())
}

#[test]
fn yew_counter() {
    bundle_example(None, "yew-counter", Opt::default())
}

#[test]
fn library() {
    let mut opt = Opt::default();
    opt.example = Some("demo".to_string());
    bundle_example(None, "library", opt)
}

#[test]
fn workspace() {
    bundle_example(Some("workspace"), "wasm-counter", Opt::default())
}
