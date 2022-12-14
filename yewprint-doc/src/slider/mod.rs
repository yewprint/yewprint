mod example;

use crate::ExampleContainer;
use example::*;
use implicit_clone::unsync::IArray;
use yew::prelude::*;
use yewprint::{HtmlSelect, Intent, Switch, H1, H2, H5};

pub struct SliderDoc {
    callback: Callback<ExampleProps>,
    state: ExampleProps,
}

impl Component for SliderDoc {
    type Message = ExampleProps;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        SliderDoc {
            callback: ctx.link().callback(|x| x),
            state: ExampleProps {
                vertical: false,
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

        html! {
            <div>
                <H1 class={classes!("docs-title")}>{"Slider"}</H1>
                <SourceCodeUrl />
                <ExampleContainer
                    source={source}
                    props={Some(html! {
                        <SliderProps
                            callback={self.callback.clone()}
                            example_props={example_props.clone()}
                        />
                    })}
                >
                    <Example ..example_props />
                </ExampleContainer>
                <H2>{"Edge Cases"}</H2>
                <div class={classes!("bp3-running-text")}>
                    <ul>
                        <li>
                            <p>
                                <strong>{"The "}
                                <code>{"values"}</code>
                                {" property is empty:"}</strong>
                            </p>
                            <p>
                                {
                                    "The slider will have no handle and the user won't be able \
                                    to interact with it."
                                }
                            </p>
                        </li>
                        <li>
                            <p>
                                <strong>{"The "}
                                <code>{"values"}</code>
                                {" property does not contain the "}
                                <code>{"selected"}</code>
                                {" property:"}</strong>
                            </p>
                            <p>
                                {
                                    "The handle will not be displayed until the \
                                    user clicks on the slider."
                                }
                            </p>
                        </li>
                        <li>
                            <p>
                                <strong>{"The "}
                                <code>{"values"}</code>
                                {" property contains only one value:"}</strong>
                            </p>
                            <p>
                                {
                                    "The handle and the label will be display at the center \
                                    of the slider and the user won't be able to interact with it."
                                }
                            </p>
                        </li>
                    </ul>
                </div>
            </div>
        }
    }
}

crate::build_example_prop_component! {
    SliderProps for ExampleProps =>
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <H5>{"Props"}</H5>
                <Switch
                    onclick={self.update_props(ctx, |props, _| ExampleProps {
                        vertical: !props.vertical,
                        ..props
                    })}
                    checked={ctx.props().example_props.vertical}
                    label={html!("Vertical")}
                    disabled=true
                />
                <p>{"Select intent:"}</p>
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
