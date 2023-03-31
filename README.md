# cargo-wasm-bundle

Compile Rust code to self-contained `.js` and `.html` files using WASM.

## Installation

```
cargo install wasm-bundle
```
This makes the `cargo-wasm-bundle` binary available in your cargo binary directory (usually `~/.cargo/bin`).

## Usage

If `PATH` contains your cargo binary directory you may use:
```
cargo wasm-bundle
```
or
```
cargo wasm-bundle --release
```

The resulting `.html` and `.js` files are available in `target/wasm-bundle/<profile>/`. To run the program, open the `.html` file in a browser.

### JavaScript module

In addition to the `.html` file, a self-contained `.js` module is generated. Call the default export `init` to run the WASM program.
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

The `example` directory contains working crate layouts. Run `cargo test` in the crate root of `wasm-bundle` to create the corresponding `.html` files for all example crates (`examples/<example>/index.html`).

Crates with Rust main programs as generated by `cargo init` work. Libraries, examples, workspaces don't. If you need this functionality consider opening an issue on GitHub and contributing a failing example.

## Development

To test local changes with the `cargo wasm-bundle` command:
1. Clone the repository
2. Make changes & build
3. Run `PATH=../wasm-bundle/target/debug/:$PATH cargo wasm-bundle` in a test crate root.

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

#### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.