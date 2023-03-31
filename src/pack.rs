use std::{
    fs::File,
    io::{copy, Write},
    path::PathBuf,
};

use base64::{engine::general_purpose::STANDARD, write::EncoderWriter};

pub fn pack(name: &str, wasm_bindgen_target_dir: &PathBuf, html_target_dir: &PathBuf) {
    let js_path = html_target_dir.join(format!("{name}.js"));
    let html_path = html_target_dir.join(format!("{name}.html"));
    {
        let mut wasm = File::open(wasm_bindgen_target_dir.join(format!("{name}_bg.wasm"))).unwrap();
        let mut jsin = File::open(wasm_bindgen_target_dir.join(format!("{name}.js"))).unwrap();

        let mut js = File::create(&js_path).unwrap();
        js.write_all("let js_base64 = \"".as_bytes()).unwrap();
        let mut enc = EncoderWriter::new(&mut js, &STANDARD);
        copy(&mut jsin, &mut enc).unwrap();
        enc.finish().unwrap();
        drop(enc);
        js.write_all("\";\n".as_bytes()).unwrap();

        copy(&mut jsin, &mut js).unwrap();
        js.write_all("let wasm_base64 = \"".as_bytes()).unwrap();
        let mut enc = EncoderWriter::new(&mut js, &STANDARD);
        copy(&mut wasm, &mut enc).unwrap();
        enc.finish().unwrap();
        drop(enc);
        js.write_all("\";\n".as_bytes()).unwrap();

        js.write_all(
            r#"
const mod = await import('data:application/javascript;base64,'+js_base64);
let wasm_buffer = Uint8Array.from(atob(wasm_base64), c => c.charCodeAt(0)).buffer;
async function init() {
    await mod.default(wasm_buffer);
}
export default init;
"#
            .as_bytes(),
        )
        .unwrap();
        js.sync_all().unwrap();
        eprintln!("Generated: {}", js_path.as_path().to_string_lossy())
    }
    {
        let mut js = File::open(&js_path).unwrap();
        let mut html = File::create(&html_path).unwrap();
        html.write_all(
            r#"<html>
<head>
    <meta charset="UTF-8">
    <title>"#
                .as_bytes(),
        )
        .unwrap();
        html.write_all(name.as_bytes()).unwrap();
        html.write_all(
            r#"</title>
</head>
<body>
    <script type="module">
"#
            .as_bytes(),
        )
        .unwrap();
        copy(&mut js, &mut html).unwrap();
        html.write_all(
            r#"
init();
        </script>
    </body>
</html>"#
                .as_bytes(),
        )
        .unwrap();
        html.sync_all().unwrap();
        eprintln!("Generated: {}", html_path.as_path().to_string_lossy())
    }
}
