mod example;

use crate::ExampleContainer;
use example::*;
use yew::prelude::*;
use yewprint::{Switch, H1, H5};

pub struct ControlGroupDoc {
    callback: Callback<ExampleProps>,
    state: ExampleProps,
}

impl Component for ControlGroupDoc {
    type Message = ExampleProps;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ControlGroupDoc {
            callback: ctx.link().callback(|x| x),
            state: ExampleProps {
                fill: false,
                vertical: false,
            },
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
            "bp3-code_block"
        );

        html! {
            <div>
                <H1 class={classes!("docs-title")}>{"ControlGroup"}</H1>
                <SourceCodeUrl />
                <ExampleContainer
                    source={source}
                    props={Some(html! {
                        <ControlGroupProps
                            callback={self.callback.clone()}
                            example_props={example_props.clone()}
                        >
                        </ControlGroupProps>
                    })}
                >
                    <Example ..example_props />
                </ExampleContainer>
            </div>
        }
    }
}

crate::build_example_prop_component! {
    ControlGroupProps for ExampleProps =>
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <H5>{"Props"}</H5>
                <Switch
                    onclick={self.update_props(ctx, |props, _| ExampleProps {
                        fill: !props.fill,
                        ..props
                    })}
                    checked={ctx.props().example_props.fill}
                    label={html!("Fill")}
                />
                <Switch
                    onclick={self.update_props(ctx, |props, _| ExampleProps {
                        vertical: !props.vertical,
                        ..props
                    })}
                    checked={ctx.props().example_props.vertical}
                    label={html!("Vertical")}
                />
            </div>
        }
    }
}

crate::build_source_code_component!();
