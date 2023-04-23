mod example;

use crate::ExampleContainer;
use example::*;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewprint::{Switch, H1, H5};

pub struct TextDoc {
    callback: Callback<ExampleProps>,
    state: ExampleProps,
}

impl Component for TextDoc {
    type Message = ExampleProps;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        TextDoc {
            callback: ctx.link().callback(|x| x),
            state: ExampleProps {
                ellipsize: false,
                text: "Hello, world!".into(),
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
                <H1 class={classes!("docs-title")}>{"Text"}</H1>
                <SourceCodeUrl />
                <div>
                    <ExampleContainer
                        source={source}
                        props={Some(html! {
                            <TextProps
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
    TextProps for ExampleProps =>
        fn view(&self, ctx: &Context<Self>) -> Html {
            html! {
                <div>
                    <H5>{"Props"}</H5>
                    <Switch
                        onclick={self.update_props(ctx, |props, _| ExampleProps {
                            ellipsize: !props.ellipsize,
                            ..props
                        })}
                        checked={ctx.props().example_props.ellipsize}
                        label={html!("Ellipsize")}
                    />
                    <p
                        style="margin-top: 5px;"
                    >
                        {"Value:"}
                    </p>
                    <input
                        class="bp3-input"
                        onchange={self.update_props(ctx, |props, e: Event| {
                                if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                                    ExampleProps {
                                        text: input.value().into(),
                                        ..props
                                    }
                                } else {
                                    ExampleProps {
                                        text: "Hello, world!".into(),
                                        ..props
                                    }
                                }
                            }
                        )}
                        type="text"
                        value={ctx.props().example_props.text.clone()}
                    />
                </div>
            }
        }
}

crate::build_source_code_component!();
