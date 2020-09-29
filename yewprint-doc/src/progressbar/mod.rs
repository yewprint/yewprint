mod example;

use crate::ExampleContainer;
use example::*;
use yew::prelude::*;
use yewprint::{Intent, Menu, MenuItem, Switch, H1, H5};


pub struct ProgressBarDoc {
    callback: Callback<ExampleProps>,
    state: ExampleProps,
}

impl Component for ProgressBarDoc {
    type Message = ExampleProps;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        ProgressBarDoc {
            callback: link.callback(|x| x),
            state: ExampleProps::default(),
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
                <H1 class="docs-title">{"ProgressBar"}</H1>
                <ExampleContainer
                    source=source
                    props=Some(html! {
                        <ProgressBarProps
                            callback={self.callback.clone()}
                            props=example_props.clone()
                        />
                    })
                >
                    <Example with example_props />
                </ExampleContainer>
            </div>
        }
    }
}

crate::build_example_prop_component! {
    ProgressBarProps for ExampleProps =>
        fn view(&self) -> Html {
            html! {
                <div>
                    <H5>{"Props"}</H5>
                    <div>
                        <Switch
                            onclick=self.update_props(|props| ExampleProps {
                                stripes: !props.stripes,
                                ..props
                            })
                            checked=self.props.stripes
                            label="Stripes"
                        />
                        <Switch
                            onclick=self.update_props(|props| ExampleProps {
                                animate: !props.animate,
                                ..props
                            })
                            checked=self.props.animate
                            label="animate"
                        />
                        <p>{"Select intent:"}</p>
                        <Menu>
                            <MenuItem
                                onclick=self.update_props(|props| ExampleProps {
                                    intent: None,
                                    ..props
                                })
                                text=html!{"None"}
                            />
                            <MenuItem
                                onclick=self.update_props(|props| ExampleProps {
                                    intent: Some(Intent::Primary),
                                    ..props
                                })
                                text=html!{"Primary"}
                                intent=Intent::Primary
                            />
                            <MenuItem
                                onclick=self.update_props(|props| ExampleProps {
                                    intent: Some(Intent::Success),
                                    ..props
                                })
                                text=html!{"Success"}
                                intent=Intent::Success
                            />
                            <MenuItem
                                onclick=self.update_props(|props| ExampleProps {
                                    intent: Some(Intent::Warning),
                                    ..props
                                })
                                text=html!{"Warning"}
                                intent=Intent::Warning
                            />
                            <MenuItem
                                onclick=self.update_props(|props| ExampleProps {
                                    intent: Some(Intent::Danger),
                                    ..props
                                })
                                text=html!{"Danger"}
                                intent=Intent::Danger
                            />
                        </Menu>
                    </div>
                </div>
            }
        }
}
