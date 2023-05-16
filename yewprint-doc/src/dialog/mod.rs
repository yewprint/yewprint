mod example;

use crate::ExampleContainer;
use example::*;
use yew::prelude::*;
use yewprint::{H1, H5};

pub struct DialogDoc {
    callback: Callback<ExampleProps>,
    state: ExampleProps,
}

impl Component for DialogDoc {
    type Message = ExampleProps;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        DialogDoc {
            callback: ctx.link().callback(|x| x),
            state: ExampleProps {},
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        self.state = msg;
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let example_props = self.state.clone();
        let source = crate::include_raw_html!(
            concat!(env!("OUT_DIR"), "/", file!(), ".html"),
            "bp3-code-block"
        );

        let props_component = html! {
            <DialogProps
                callback={self.callback.clone()}
                example_props={example_props.clone()}
            />
        };

        html! {
            <div>
                <H1 class={classes!("docs-title")}>{"Dialog"}</H1>
                <SourceCodeUrl />
                <div>
                    <ExampleContainer
                        source={source}
                        props={Some(props_component)}
                    >
                        <Example ..example_props />
                    </ExampleContainer>
                </div>
            </div>
        }
    }
}

crate::build_example_prop_component! {
    DialogProps for ExampleProps =>
        fn view(&self, _ctx: &Context<Self>) -> Html {
            html! {
                <div>
                    <H5>{"Props"}</H5>
                </div>
            }
        }
}

crate::build_source_code_component!();
