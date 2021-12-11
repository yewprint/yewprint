mod example;

use crate::ExampleContainer;
use example::*;
use yew::prelude::*;
use yewprint::{HtmlSelect, Intent, Slider, H1, H5};

pub struct SpinnerDoc {
    callback: Callback<ExampleProps>,
    state: ExampleProps,
}

impl Component for SpinnerDoc {
    type Message = ExampleProps;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        SpinnerDoc {
            callback: ctx.link().callback(|x| x),
            state: ExampleProps {
                intent: None,
                size: 50,
            },
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        self.state = msg;
        true
    }

    fn view(&self) -> Html {
        let example_props = self.state.clone();
        let source = crate::include_raw_html!(
            concat!(env!("OUT_DIR"), "/", file!(), ".html"),
            "bp3-code-block"
        );

        html! {
            <div>
                <H1 class={classes!("docs-title")}>{"Spinner"}</H1>
                <SourceCodeUrl />
                <div>
                    <ExampleContainer
                        source={source}
                        props={Some(html! {
                            <SpinnerProps
                                callback={self.callback.clone()}
                                {props=example_props.clone()}
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
    SpinnerProps for ExampleProps =>
    fn view(&self) -> Html {
        html! {
            <div>
                <H5>{"Props"}</H5>
                <div>
                    <p>{"Select intent:"}</p>
                    <HtmlSelect<Option<Intent>>
                        options={vec![
                            (None, "None".to_string()),
                            (Some(Intent::Primary), "Primary".to_string()),
                            (Some(Intent::Success), "Success".to_string()),
                            (Some(Intent::Warning), "Warning".to_string()),
                            (Some(Intent::Danger), "Danger".to_string()),
                        ]}
                        onchange={self.update_props(|props, intent| ExampleProps {
                            intent,
                            ..props
                        })}
                    />
                    <p
                        style="margin-top: 5px;"
                    >
                        {"Select Size:"}
                    </p>
                    <Slider<u32>
                        selected={self.props.size}
                        values={vec![
                            (10, Some("10".into())),
                            (20, None),
                            (30, None),
                            (40, None),
                            (50, Some("50".into())),
                            (60, None),
                            (70, None),
                            (80, None),
                            (90, None),
                            (100, Some("100".into())),
                        ]}
                        onchange={self.update_props(|props, size| ExampleProps {
                            size,
                            ..props
                        })}
                    />
                </div>
            </div>
        }
    }
}

crate::build_source_code_component!();
