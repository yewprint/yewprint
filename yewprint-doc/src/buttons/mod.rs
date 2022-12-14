mod example;

use crate::ExampleContainer;
use example::*;
use implicit_clone::unsync::IArray;
use yew::prelude::*;
use yewprint::{HtmlSelect, Intent, Switch, H1, H5};

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
                intent: None,
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

        let props_component = html! {
            <ButtonProps
                callback={self.callback.clone()}
                example_props={example_props.clone()}
            />
        };

        html! {
            <div>
                <H1 class={classes!("docs-title")}>{"Button"}</H1>
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
    ButtonProps for ExampleProps =>
        fn view(&self, ctx: &Context<Self>) -> Html {
            html! {
                <div>
                    <H5>{"Props"}</H5>
                    <Switch
                        onclick={self.update_props(ctx, |props, _| ExampleProps {
                            minimal: !props.minimal,
                            ..props
                        })}
                        checked={ctx.props().example_props.minimal}
                        label={html!("Minimal")}
                    />
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
                            small: !props.small,
                            ..props
                        })}
                        checked={ctx.props().example_props.small}
                        label={html!("Small")}
                    />
                    <Switch
                        onclick={self.update_props(ctx, |props, _| ExampleProps {
                            outlined: !props.outlined,
                            ..props
                        })}
                        checked={ctx.props().example_props.outlined}
                        label={html!("Outlined")}
                    />
                    <Switch
                        onclick={self.update_props(ctx, |props, _| ExampleProps {
                            loading: !props.loading,
                            ..props
                        })}
                        checked={ctx.props().example_props.loading}
                        label={html!("Loading")}
                    />
                    <Switch
                        onclick={self.update_props(ctx, |props, _| ExampleProps {
                            large: !props.large,
                            ..props
                        })}
                        checked={ctx.props().example_props.large}
                        label={html!("Large")}
                    />
                    <Switch
                        onclick={self.update_props(ctx, |props, _| ExampleProps {
                            active: !props.active,
                            ..props
                        })}
                        checked={ctx.props().example_props.active}
                        label={html!("Active")}
                    />
                    <Switch
                        onclick={self.update_props(ctx, |props, _| ExampleProps {
                            disabled: !props.disabled,
                            ..props
                        })}
                        checked={ctx.props().example_props.disabled}
                        label={html!("Disabled")}
                    />
                    <HtmlSelect<Option<Intent>>
                        options={[
                            (None, "None".into()),
                            (Some(Intent::Primary), "Primary".into()),
                            (Some(Intent::Success), "Success".into()),
                            (Some(Intent::Warning), "Warning".into()),
                            (Some(Intent::Danger), "Danger".into()),
                        ].into_iter().collect::<IArray<_>>()}
                        onchange={self.update_props(ctx, |props, intent| ExampleProps {
                            intent,
                            ..props
                        })}
                        value={ctx.props().example_props.intent}
                    />
                </div>
            }
        }
}

crate::build_source_code_component!();
