mod example;

use crate::ExampleContainer;
use example::*;
use yew::prelude::*;
use yewprint::H1;

pub struct MenuDoc;

impl Component for MenuDoc {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        MenuDoc
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let source = crate::include_raw_html!(
            concat!(env!("OUT_DIR"), "/", file!(), ".html"),
            "bp3-code-block"
        );

        html! {
            <div>
                <H1 class={classes!("docs-title")}>{"Menu"}</H1>
                <SourceCodeUrl />
                <ExampleContainer source={source}>
                    <Example />
                </ExampleContainer>
            </div>
        }
    }
}

crate::build_source_code_component!();
