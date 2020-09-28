mod example;

use crate::ExampleContainer;
use example::*;
use yew::prelude::*;
use yewprint::{Intent, Menu, MenuItem, Switch};

pub struct CalloutDoc {
    callback: Callback<ExampleProps>,
    state: ExampleProps,
}

impl Component for CalloutDoc {
    type Message = ExampleProps;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        CalloutDoc {
            callback: link.callback(|x| x),
            state: ExampleProps {
                show_icon: true,
                intent: None,
                show_title: true,
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
                <h1>{"Callout"}</h1>
                <ExampleContainer
                    source=source
                    props=Some(html! {
                        <CalloutProps
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
    CalloutProps for ExampleProps =>
        fn view(&self) -> Html {
            html! {
                <div>
                    <h5>{"Props"}</h5>
                    <div>
                        <Switch
                            onclick=self.update_props(|props| ExampleProps {
                                show_icon: !props.show_icon,
                                ..props
                            })
                            checked=self.props.show_icon
                            label="Show/hide icon"
                        />
                        <Switch
                            onclick=self.update_props(|props| ExampleProps {
                                show_title: !props.show_title,
                                ..props
                            })
                            checked=self.props.show_title
                            label="Show/hide title"
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
