mod pack;
#[cfg(test)]
mod test;

use pack::pack;
use std::{
    env::var,
    ffi::OsStr,
    fs::create_dir_all,
    io::ErrorKind,
    path::{PathBuf, Path},
    process::{exit, Command, ExitStatus},
};

use clap::{Args, Parser};

#[derive(Args, Debug, Default)]
pub struct Opt {
    /// Build with the dev profile
    #[clap(long)]
    release: bool,
}

#[derive(Parser, Debug)]
#[clap(bin_name = "cargo")]
pub enum Cli {
    /// A cargo subcommand for generating flamegraphs, using inferno
    #[clap(version)]
    WasmBundle(Opt),
}

pub fn exec(base_dir: &Path, opt: Opt) {
    let target_profile = match opt.release {
        true => "release",
        false => "debug",
    };
    let cargo_meta = cargo_metadata::MetadataCommand::new()
        .current_dir(base_dir)
        .no_deps()
        .exec()
        .unwrap();
    let cargo_build_target_dir = cargo_meta
        .target_directory
        .as_std_path()
        .join("wasm32-unknown-unknown")
        .join(target_profile);
    let name = &cargo_meta.root_package().unwrap().name;
    let wasm_bindgen_target_dir = cargo_meta
        .target_directory
        .as_std_path()
        .join("wasm-bindgen")
        .join(target_profile);
    let html_target_dir = cargo_meta
        .target_directory
        .as_std_path()
        .join("wasm-bundle")
        .join(target_profile);
    create_dir_all(&html_target_dir).unwrap();

    cargo_build(base_dir, &opt);
    check_wasm_bindgen(base_dir);
    wasm_bindgen(base_dir, &cargo_build_target_dir, &name, &wasm_bindgen_target_dir);
    pack(name, &wasm_bindgen_target_dir, &html_target_dir);
}

fn cargo_build(base_dir: &Path, opt: &Opt) {
    let mut cmd = Command::new(var("CARGO").unwrap());
    cmd.arg("build")
        .args(["--target", "wasm32-unknown-unknown"]);
    if opt.release {
        cmd.arg("--release");
    }
    run(base_dir, cmd)
}

fn check_wasm_bindgen(base_dir: &Path) {
    let mut cmd = Command::new("wasm-bindgen");
    cmd.arg("--version");
    eprintln!("running {:?}", cmd);
    match cmd.status() {
        Ok(status) => check_status(status),
        Err(err) => match err.kind() {
            ErrorKind::NotFound => {
                let mut cmd = Command::new(var("CARGO").unwrap());
                cmd.args(["install", "wasm-bindgen-cli"]);
                run(base_dir, cmd);
            }
            _ => Err(err).unwrap(),
        },
    }
}

fn wasm_bindgen(base_dir: &Path, cargo_build_target_dir: &PathBuf, name: &str, wasm_bindgen_target_dir: &PathBuf) {
    let mut cmd = Command::new("wasm-bindgen");
    let infile = cargo_build_target_dir.join(format!("{name}.wasm"));
    cmd.args(["--target", "web"])
        .arg(infile.into_os_string())
        .args([
            &OsStr::new("--out-dir"),
            wasm_bindgen_target_dir.as_os_str(),
        ]);
    run(base_dir, cmd)
}

fn run(base_dir: &Path, mut cmd: Command) {
    cmd.current_dir(base_dir);
    eprintln!("running {:?}", cmd);
    check_status(cmd.status().unwrap());
}

fn check_status(status: ExitStatus) {
    let code = status.code().unwrap();
    if code != 0 {
        exit(code)
    }
}
