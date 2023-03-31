use yew::prelude::*;

pub fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    wasm_logger::init(wasm_logger::Config::new(log::Level::Info));
    yew::Renderer::<App>::new().render();
}

struct App {
    counter: usize
}

enum Msg {
    Inc,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        log::info!("create");
        Self{ counter: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        log::info!("update");
        match msg {
            Msg::Inc => self.counter += 1,
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <p>{ self.counter }</p>
                <button onclick={ctx.link().callback(|_| Msg::Inc)}>{ "+1" }</button>
            </>
        }
    }
}
