# cargo-wasm-bundle

Compile Rust code to self-contained `.js` and `.html` files using WASM.

## Installation

```
cargo install wasm-bundle
```
This makes the `cargo-wasm-bundle` binary available in your cargo binary directory (usually `~/.cargo/bin`). To use it as a cargo extension (`cargo wasm-bundle`), `cargo-wasm-bundle` must be present in any directory in `$PATH`.

## Usage

The CLI is modeled after `cargo build`, but only supports a subset of crate layouts and options at the moment. Please open an issue with your usecase, if this is not sufficient for you.

Build the default binary target (`src/main.rs`).
```
cargo wasm-bundle
```
The resulting `.html` and `.js` files are available in `target/wasm-bundle/<profile>/`. To run the program, open the `.html` file in a browser.

### Example targets

binary target (`/example/demo.rs`)
```
cargo wasm-bundle --example demo
```

### Release profile
You may add the `--release` flag to any `cargo wasm-bundle` command, to enable the release profile.
```
cargo wasm-bundle --release
```

### JavaScript module

In addition to the `.html` file, a self-contained `.js` module is generated. Call the default export to run the WASM program.
```
<html>
<body>
    <script type="module">
        import init from "./my_crate.js";
        init();
    </script>
</body>
</html>
```

## `wasm-bindgen`

This crate uses wasm-bindgen-cli. Mismatched versions of wasm-bindgen-cli and wasm-bindgen dependecies may lead to compilation errors. The simplest solution is to update both to the latest version:
```
cargo update -p wasm-bindgen
cargo install -f wasm-bindgen-cli
```

## What works, what doesn't

The `examples` directory in this crate contains working crates. Run `cargo test` in the crate root of `wasm-bundle` to create the corresponding `.html` files for all example crates (`examples/<example>/index.html`).

What works:
- Default binary target (`src/main.rs`)
- Example binary targets (`examples/demo.rs`)

Untested / doesn't work:
- Library targets
- Additional binary targets
- Customized binary targets are not tested
- Workspaces

If you need additional functionality consider opening an issue on GitHub and maybe contribute a matching example crate layout.

## Development

To test local changes with the `cargo wasm-bundle` command:
1. Clone the repository
2. Make changes & build
3. Run `PATH=<wasm_bundle>/target/debug/:$PATH cargo wasm-bundle` in a test crate root.

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

#### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
