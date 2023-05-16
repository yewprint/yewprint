mod example;

use crate::ExampleContainer;
use example::*;
use yew::prelude::*;
use yewprint::H1;

pub struct DialogDoc {
    callback: Callback<ExampleProps>,
}

impl Component for DialogDoc {
    type Message = ExampleProps;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        DialogDoc {
            callback: ctx.link().callback(|x| x),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let source = crate::include_raw_html!(
            concat!(env!("OUT_DIR"), "/", file!(), ".html"),
            "bp3-code-block"
        );

        html! {
            <div>
                <H1 class={classes!("docs-title")}>{"Dialog"}</H1>
                <SourceCodeUrl />
                <div>
                    <ExampleContainer source={source}>
                        <Example />
                    </ExampleContainer>
                </div>
            </div>
        }
    }
}

crate::build_source_code_component!();
