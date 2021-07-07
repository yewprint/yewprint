mod example;

use crate::ExampleContainer;
use example::*;
use yew::prelude::*;
use yewprint::H1;

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
                <SourceCodeUrl
                    url=COLLAPSE_URL
                />
                <ExampleContainer source=source>
                    <Example />
                </ExampleContainer>
            </div>
        }
    }
}

crate::build_source_code_component!(
    COLLAPSE_URL,
    "https://github.com/yewprint/yewprint/blob/main/yewprint/src/collapse.rs",
    check_collapse_url
);
