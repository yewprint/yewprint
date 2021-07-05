mod example;

use crate::ExampleContainer;
use example::*;
use yew::prelude::*;
use yewprint::{Text, H1};

pub struct CollapseDoc;

impl Component for CollapseDoc {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        CollapseDoc
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let source = crate::include_raw_html!(
            concat!(env!("OUT_DIR"), "/", file!(), ".html"),
            "bp3-code-block"
        );

        html! {
            <div>
                <H1 class=classes!("docs-title")>{"Collapse"}</H1>
                <a
                class=classes!("bp3-text-muted")
                href="https://github.com/yewprint/yewprint/blob/main/yewprint/src/collapse.rs"
                target="_blank"
            >
                <Text>{"Go to the source code"}</Text>
            </a>
                <ExampleContainer source=source>
                    <Example />
                </ExampleContainer>
            </div>
        }
    }
}
