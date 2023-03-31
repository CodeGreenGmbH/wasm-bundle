use gloo::timers::future::sleep;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{HtmlButtonElement, HtmlParagraphElement};

pub fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    wasm_logger::init(wasm_logger::Config::new(log::Level::Info));
    wasm_bindgen_futures::spawn_local(run())
}

static COUNTER: AtomicUsize = AtomicUsize::new(0);

async fn run() {
    log::info!("start");

    let document = web_sys::window().unwrap().document().unwrap();
    let button: HtmlButtonElement = document
        .create_element("button")
        .unwrap()
        .dyn_into()
        .unwrap();
    let p: HtmlParagraphElement = document.create_element("p").unwrap().dyn_into().unwrap();
    document.body().unwrap().append_child(&p).unwrap();
    document.body().unwrap().append_child(&button).unwrap();

    button.set_text_content(Some("+1"));
    inc(&p);

    let rc_p = Rc::new(p.clone());
    let cb = move || {
        inc(&rc_p);
    };
    let cb = Closure::wrap(Box::new(cb) as Box<dyn FnMut()>);
    button
        .add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())
        .unwrap();
    cb.forget();

    for _ in 0..3 {
        sleep(Duration::from_secs(5)).await;
        log::info!("auto-inc");
        inc(&p);
    }
}

fn inc(p: &HtmlParagraphElement) {
    log::info!("inc");
    let v = COUNTER.fetch_add(1, Ordering::SeqCst);
    p.set_text_content(Some(&format!("{}", v)));
}
