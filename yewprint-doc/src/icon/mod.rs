mod example;

use crate::ExampleContainer;
use example::*;
use yew::prelude::*;
use yewprint::H1;

pub struct IconDoc;

impl Component for IconDoc {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        IconDoc
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }

    fn view(&self) -> Html {
        let source = crate::include_raw_html!(
            concat!(env!("OUT_DIR"), "/", file!(), ".html"),
            "bp3-code-block"
        );

        html! {
            <div>
                <H1 class={classes!("docs-title")}>{"Icon"}</H1>
                <SourceCodeUrl />
                <ExampleContainer source={source}>
                    <Example />
                </ExampleContainer>
            </div>
        }
    }
}

crate::build_source_code_component!();
