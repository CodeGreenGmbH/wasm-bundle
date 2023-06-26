pub fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    wasm_logger::init(wasm_logger::Config::new(log::Level::Info));
    wasm_bindgen_futures::spawn_local(run())
}

async fn run() {
    let a = 3;
    let b = 17;
    let sum = library::add(a, b);
    log::info!("{a} + {b} = {sum}");
}
