use yew::prelude::*;
use yewprint::{Button,Collapse};

pub struct Example {
    link: ComponentLink<Self>,
    collapsed: bool,
}

pub enum Msg {
    ToggleCollapse,
}

impl Component for Example {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Example {
            collapsed: true,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ToggleCollapse => self.collapsed ^= true,
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <Button onclick=self.link.callback(|_| Msg::ToggleCollapse)>
                    {"Toggle collapse"}
                </Button>
                <Collapse
                    is_open=!self.collapsed
                    keep_children_mounted=true
                >
                    <pre class="bp3-code-block">
                        <div>{"[INFO]: Installing wasm-bindgen..."}</div>
                        <div>{"[INFO]: Optional fields missing from Cargo.toml: 'description', 'repository', and 'license'. These are not necessary, but recommended"}</div>
                        <div>{"[INFO]: :-) Done in 0.69s"}</div>
                        <div>{"[INFO]: :-) Your wasm pkg is ready to publish at /home/cecile/repos/blueprint-rs/./static."}</div>
                        <div>{"     Index: enabled, Upload: disabled, Cache: disabled, Cors: enabled, Range: enabled, Sort: enabled, Threads: 3"}</div>
                        <div>{"          Auth: disabled, Compression: disabled"}</div>
                        <div>{"         https: disabled, Cert: , Cert-Password: "}</div>
                        <div>{"          Root: /home/cecile/repos/blueprint-rs,"}</div>
                        <div>{"    TryFile404: "}</div>
                        <div>{"       Address: http://0.0.0.0:8000"}</div>
                        <div>{"    ======== [2020-09-07 20:39:46] ========"}</div>
                        <div>{"[2020-09-07 20:39:46] - 127.0.0.1 - 200 - GET /"}</div>
                        <div>{"[2020-09-07 20:39:46] - 127.0.0.1 - 200 - GET /static/blueprint.css"}</div>
                        <div>{"[2020-09-07 20:39:46] - 127.0.0.1 - 200 - GET /static/wasm.js"}</div>
                        <div>{"[2020-09-07 20:39:46] - 127.0.0.1 - 200 - GET /static/wasm_bg.wasm"}</div>
                    </pre>
                </Collapse>
            </div>
        }
    }
}
