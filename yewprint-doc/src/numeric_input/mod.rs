mod example;

use crate::ExampleContainer;
use example::*;
use yew::prelude::*;
use yewprint::{Switch, H1, H5};

pub struct NumericInputDoc {
    callback: Callback<ExampleProps>,
    state: ExampleProps,
}

impl Component for NumericInputDoc {
    type Message = ExampleProps;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        NumericInputDoc {
            callback: ctx.link().callback(|x| x),
            state: ExampleProps {
                fill: false,
                disabled: false,
                large: false,
                disable_buttons: false,
                buttons_on_the_left: false,
                left_icon: false,
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
                <H1 class={classes!("docs-title")}>{"NumericInput"}</H1>
                <SourceCodeUrl />
                <div>
                    <ExampleContainer
                        source={source}
                        props={Some(html! {
                            <NumericInputProps
                                callback={self.callback.clone()}
                                example_props={example_props.clone()}
                            />
                        })}
                    >
                        <Example ..example_props />
                    </ExampleContainer>
                </div>
            </div>
        }
    }
}

crate::build_example_prop_component! {
    NumericInputProps for ExampleProps =>
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <H5>{"Props"}</H5>
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
                        disabled: !props.disabled,
                        ..props
                    })}
                    checked={ctx.props().example_props.disabled}
                    label={html!("Disabled")}
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
                        disable_buttons: !props.disable_buttons,
                        ..props
                    })}
                    checked={ctx.props().example_props.disable_buttons}
                    label={html!("Disable buttons")}
                />
                <Switch
                    onclick={self.update_props(ctx.props(), |props, _| ExampleProps {
                        buttons_on_the_left: !props.buttons_on_the_left,
                        ..props
                    })}
                    checked={ctx.props().example_props.buttons_on_the_left}
                    label={html!("Buttons on the left")}
                />
                <Switch
                    onclick={self.update_props(ctx.props(), |props, _| ExampleProps {
                        left_icon: !props.left_icon,
                        ..props
                    })}
                    checked={ctx.props().example_props.left_icon}
                    label={html!("Left icon")}
                />
            </div>
        }
    }
}

crate::build_source_code_component!();
