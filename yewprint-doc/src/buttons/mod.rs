mod example;

use crate::ExampleContainer;
use example::*;
use yew::prelude::*;
use yewprint::{Switch, H1, H5};

pub struct ButtonDoc {
    callback: Callback<ExampleProps>,
    state: ExampleProps,
}

impl Component for ButtonDoc {
    type Message = ExampleProps;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ButtonDoc {
            callback: ctx.link().callback(|x| x),
            state: ExampleProps {
                minimal: false,
                fill: false,
                small: false,
                outlined: false,
                loading: false,
                large: false,
                active: false,
                disabled: false,
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
            "bp3-code-block"
        );

        html! {
            <div>
                <H1 class={classes!("docs-title")}>{"Button"}</H1>
                <SourceCodeUrl />
                <div>
                    <ExampleContainer
                        source={source}
                        props={Some(html! {
                            <ButtonProps
                                callback={self.callback.clone()}
                                props={example_props.clone()}
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
    ButtonProps for ExampleProps =>
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <H5>{"Props"}</H5>
                <Switch
                    onclick={self.update_props(ctx.props(), |props, _| ExampleProps {
                        minimal: !props.minimal,
                        ..props
                    })}
                    checked={ctx.props().minimal}
                    label={html!("Minimal")}
                />
                <Switch
                    onclick={self.update_props(ctx.props(), |props, _| ExampleProps {
                        fill: !props.fill,
                        ..props
                    })}
                    checked={ctx.props().fill}
                    label={html!("Fill")}
                />
                <Switch
                    onclick={self.update_props(ctx.props(), |props, _| ExampleProps {
                        small: !props.small,
                        ..props
                    })}
                    checked={ctx.props().small}
                    label={html!("Small")}
                />
                <Switch
                    onclick={self.update_props(ctx.props(), |props, _| ExampleProps {
                        outlined: !props.outlined,
                        ..props
                    })}
                    checked={ctx.props().outlined}
                    label={html!("Outlined")}
                />
                <Switch
                    onclick={self.update_props(ctx.props(), |props, _| ExampleProps {
                        loading: !props.loading,
                        ..props
                    })}
                    checked={ctx.props().loading}
                    label={html!("Loading")}
                />
                <Switch
                    onclick={self.update_props(ctx.props(), |props, _| ExampleProps {
                        large: !props.large,
                        ..props
                    })}
                    checked={ctx.props().large}
                    label={html!("Large")}
                />
                <Switch
                    onclick={self.update_props(ctx.props(), |props, _| ExampleProps {
                        active: !props.active,
                        ..props
                    })}
                    checked={ctx.props().active}
                    label={html!("Active")}
                />
                <Switch
                    onclick={self.update_props(ctx.props(), |props, _| ExampleProps {
                        disabled: !props.disabled,
                        ..props
                    })}
                    checked={ctx.props().disabled}
                    label={html!("Disabled")}
                />
            </div>
        }
    }
}

crate::build_source_code_component!();
