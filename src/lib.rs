mod pack;
#[cfg(test)]
mod test;

use pack::pack;
use std::{
    env::var,
    ffi::OsStr,
    fs::create_dir_all,
    io::ErrorKind,
    path::{Path, PathBuf},
    process::{exit, Command, ExitStatus},
};

use clap::{Args, Parser};

#[derive(Args, Debug, Default)]
pub struct Opt {
    /// Build with the dev profile
    #[clap(long)]
    pub release: bool,
    #[clap(long)]
    pub example: Option<String>,
}

#[derive(Parser, Debug)]
#[clap(bin_name = "cargo")]
pub enum Cli {
    /// compile to self-contained `.js` and `.html` files using WASM
    #[clap(version)]
    WasmBundle(Opt),
}

pub fn exec(base_dir: &Path, opt: &Opt) {
    let base_dir = std::fs::canonicalize(base_dir).unwrap();
    let target_profile = match opt.release {
        true => "release",
        false => "debug",
    };
    let cargo_meta = cargo_metadata::MetadataCommand::new()
        .current_dir(base_dir.clone())
        .no_deps()
        .exec()
        .unwrap();
    let mut cargo_build_target_dir = cargo_meta
        .target_directory
        .as_std_path()
        .join("wasm32-unknown-unknown")
        .join(target_profile);
    let mut wasm_bindgen_target_dir = cargo_meta
        .target_directory
        .as_std_path()
        .join("wasm-bindgen")
        .join(target_profile);
    let mut html_target_dir = cargo_meta
        .target_directory
        .as_std_path()
        .join("wasm-bundle")
        .join(target_profile);

    let mut name = None;
    for package in cargo_meta.workspace_packages() {
        let manifest_path = package.manifest_path.clone().into_std_path_buf();
        let manifest_path = std::fs::canonicalize(manifest_path).unwrap();
        let manifest_dir = manifest_path.parent().unwrap();
        if manifest_dir == base_dir {
            name = Some(&package.name);
        }
    }

    let mut name = name.expect("cargo wasm-bundle must be executed in a package root");

    if let Some(example) = &opt.example {
        name = example;
        cargo_build_target_dir = cargo_build_target_dir.join("examples");
        wasm_bindgen_target_dir = wasm_bindgen_target_dir.join("examples");
        html_target_dir = html_target_dir.join("examples");
    }

    create_dir_all(&html_target_dir).unwrap();

    cargo_build(&base_dir, &opt);
    check_wasm_bindgen(&base_dir);
    wasm_bindgen(
        &base_dir,
        &cargo_build_target_dir,
        &name,
        &wasm_bindgen_target_dir,
    );
    pack(name, &wasm_bindgen_target_dir, &html_target_dir);
}

fn cargo_build(base_dir: &Path, opt: &Opt) {
    let mut cmd = Command::new(var("CARGO").unwrap_or("cargo".into()));
    cmd.arg("build")
        .args(["--target", "wasm32-unknown-unknown"]);
    if opt.release {
        cmd.arg("--release");
    }
    if let Some(example) = &opt.example {
        cmd.args(["--example", example]);
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

fn wasm_bindgen(
    base_dir: &Path,
    cargo_build_target_dir: &PathBuf,
    name: &str,
    wasm_bindgen_target_dir: &PathBuf,
) {
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

pub(crate) fn run(base_dir: &Path, mut cmd: Command) {
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
