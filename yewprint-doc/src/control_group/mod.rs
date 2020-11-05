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

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        ControlGroupDoc {
            callback: link.callback(|x| x),
            state: ExampleProps {
                fill: false,
                vertical: false,
            },
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        self.state = msg;
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        let example_props = self.state.clone();
        let source = crate::include_raw_html!(
            concat!(env!("OUT_DIR"), "/", file!(), ".html"),
            "bp3-code_block"
        );

        html! {
            <div>
                <H1 class="docs-title">{"Control Group"}</H1>
                <ExampleContainer
                    source=source
                    props=Some(html! {
                        <ControlGroupProps
                            callback={self.callback.clone()}
                            props=example_props.clone()
                        >
                        </ControlGroupProps>
                    })
                >
                    <Example with example_props />
                </ExampleContainer>
            </div>
        }
    }
}

crate::build_example_prop_component! {
    ControlGroupProps for ExampleProps =>
    fn view(&self) -> Html {
        html! {
            <div>
                <H5>{"Props"}</H5>
                <Switch
                    onclick=self.update_props(|props, _| ExampleProps {
                        fill: !props.fill,
                        ..props
                    })
                    checked=self.props.fill
                    label=html!("Fill")
                />
                <Switch
                    onclick=self.update_props(|props, _| ExampleProps {
                        vertical: !props.vertical,
                        ..props
                    })
                    checked=self.props.vertical
                    label=html!("Vertical")
                />
            </div>
        }
    }
}