mod example;

use crate::ExampleContainer;
use example::*;
use yew::prelude::*;
use yewprint::H1;

pub struct ButtonGroupDoc;

impl Component for ButtonGroupDoc {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self
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
                <H1 class="docs-title">{"Button Group"}</H1>
                <ExampleContainer source=source>
                    <Example />
                </ExampleContainer>
            </div>
        }
    }
}
