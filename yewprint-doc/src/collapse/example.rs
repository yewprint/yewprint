use yew::prelude::*;
use yewprint::{Button, Collapse};

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
        let logs = include_str!("example.log");

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
                        {logs}
                    </pre>
                </Collapse>
            </div>
        }
    }
}
