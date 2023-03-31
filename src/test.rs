use std::{fs, path::Path};

use crate::{exec, Opt};

fn bundle_example(example: &str) {
    let base_path = Path::new("examples").join(example);
    exec(&base_path, Opt::default());
    let html_path = base_path.join("target/wasm-bundle/debug").join(format!("{example}.html"));
    fs::copy(html_path, base_path.join("index.html")).unwrap();
}

#[test]
fn wasm_counter() {
    bundle_example("wasm-counter")
}

#[test]
fn yew_counter() {
    bundle_example("yew-counter")
}
