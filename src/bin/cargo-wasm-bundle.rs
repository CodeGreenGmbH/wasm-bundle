use std::path::Path;

use clap::Parser;
use wasm_bundle::{exec, Cli};

fn main() {
    let Cli::WasmBundle(opt) = Cli::parse();
    exec(Path::new("."),opt);
}
