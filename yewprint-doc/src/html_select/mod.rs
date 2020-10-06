mod example;

use yew::prelude::*;
use crate::ExampleContainer;
use example::*;
use yewprint::{Switch, H1, H5};

pub struct HtmlSelectDoc {
    callback: Callback<ExampleProps>,
    state: ExampleProps,
}

impl Component for HtmlSelectDoc {
    type Message = ExampleProps;
    type Properties = ();


    fn create(_:Self::Properties, link: ComponentLink<Self>) -> Self {
        HtmlSelectDoc {
            callback: link.callback(|x| x),
            state: ExampleProps {
                minimal: false,
                fill: false,
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

        html!{
            <div>
                <H1 class="docs-title">{"HtmlSelect"}</H1>
                <div>
                    <ExampleContainer
                        source=source
                        props=Some(html! {
                            <ButtonProps
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
    ButtonProps for ExampleProps =>
        fn view(&self) -> Html {
            html! {
                <div>
                    <H5>{"Props"}</H5>
                <Switch
                    onclick=self.update_props(|props| ExampleProps {
                        minimal: !props.minimal,
                        ..props
                    })
                    checked=self.props.minimal
                    label="Minimal"
                />
                <Switch
                    onclick=self.update_props(|props| ExampleProps{
                        fill: !props.fill,
                        ..props
                    })
                    checked=self.props.fill
                    label="Fill"
                />
            </div>
            }
        }
}