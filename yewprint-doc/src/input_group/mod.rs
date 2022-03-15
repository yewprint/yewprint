mod example;

use crate::ExampleContainer;
use example::*;
use yew::prelude::*;
use yewprint::{Switch, H1, H5};

pub struct InputGroupDoc {
    callback: Callback<ExampleProps>,
    state: ExampleProps,
}

impl Component for InputGroupDoc {
    type Message = ExampleProps;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        InputGroupDoc {
            callback: ctx.link().callback(|x| x),
            state: ExampleProps {
                disabled: false,
                fill: false,
                large: false,
                small: false,
                round: false,
            },
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        self.state = msg;
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let example_props = self.state.clone();
        let source = crate::include_raw_html!(
            concat!(env!("OUT_DIR"), "/", file!(), ".html"),
            "bp3-code-block"
        );

        html! {
            <div>
                <H1 class={classes!("docs-title")}>{"InputGroup"}</H1>
                <SourceCodeUrl />
                <ExampleContainer
                    source={source}
                    props={Some(html! {
                        <InputGroupProps
                            callback={self.callback.clone()}
                            example_props={example_props.clone()}
                        >
                        </InputGroupProps>
                    })}
                >
                    <Example ..example_props />
                </ExampleContainer>
            </div>
        }
    }
}

crate::build_example_prop_component! {
    InputGroupProps for ExampleProps =>
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <H5>{"Props"}</H5>
                <Switch
                    onclick={self.update_props(ctx.props(), |props, _| ExampleProps {
                        disabled: !props.disabled,
                        ..props
                    })}
                    checked={ctx.props().example_props.disabled}
                    label={html!("Disabled")}
                />
                <Switch
                    onclick={self.update_props(ctx.props(), |props, _| ExampleProps {
                        fill: !props.fill,
                        ..props
                    })}
                    checked={ctx.props().example_props.fill}
                    label={html!("Fill")}
                />
                <Switch
                    onclick={self.update_props(ctx.props(), |props, _| ExampleProps {
                        large: !props.large,
                        ..props
                    })}
                    checked={ctx.props().example_props.large}
                    label={html!("Large")}
                />
                <Switch
                    onclick={self.update_props(ctx.props(), |props, _| ExampleProps {
                        small: !props.small,
                        ..props
                    })}
                    checked={ctx.props().example_props.small}
                    label={html!("Small")}
                />
                <Switch
                    onclick={self.update_props(ctx.props(), |props, _| ExampleProps {
                        round: !props.round,
                        ..props
                    })}
                    checked={ctx.props().example_props.round}
                    label={html!("Round")}
                />
            </div>
        }
    }
}

crate::build_source_code_component!();
