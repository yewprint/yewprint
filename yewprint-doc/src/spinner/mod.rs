mod example;

use crate::ExampleContainer;
use example::*;
use yew::prelude::*;
use yewprint::{HtmlSelect, Intent, H1, H5};

pub struct SpinnerDoc {
    callback: Callback<ExampleProps>,
    state: ExampleProps,
}

impl Component for SpinnerDoc {
    type Message = ExampleProps;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        SpinnerDoc {
            callback: link.callback(|x| x),
            state: ExampleProps {
                intent: None,
                size: 10,
            },
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        self.state = msg;
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
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
                <H1 class=classes!("docs-title")>{"Spinner"}</H1>
                <div>
                    <ExampleContainer
                        source=source
                        props=Some(html! {
                            <SpinnerProps
                                callback={self.callback.clone()}
                                props=example_props.clone()
                            />
                        })
                    >
                        <Example with example_props />
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
                        onchange=self.update_props(|props, intent| ExampleProps {
                            intent,
                            ..props
                        })
                    />
                    <p>{"Select Size:"}</p>
                    <HtmlSelect<i32>
                        options={vec![
                            (20, "Small".to_string()),
                            (50, "Standard".to_string()),
                            (100, "Large".to_string()),
                        ]}
                        onchange=self.update_props(|props, size| ExampleProps {
                            size,
                            ..props
                        })
                    />
                </div>
            </div>
        }
    }
}
