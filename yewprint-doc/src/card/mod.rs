mod example;

use crate::ExampleContainer;
use example::*;
use yew::prelude::*;
use yewprint::{Elevation, Menu, MenuItem, Switch, H1, H5};

pub struct CardDoc {
    callback: Callback<ExampleProps>,
    state: ExampleProps,
}

impl Component for CardDoc {
    type Message = ExampleProps;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        CardDoc {
            callback: link.callback(|x| x),
            state: ExampleProps {
                elevation: Elevation::Level0,
                interactive: false,
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
                <H1 class="docs-title">{"Card"}</H1>
                <ExampleContainer
                    source=source
                    props=Some(html! {
                        <CardProps
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
    CardProps for ExampleProps =>
        fn view(&self) -> Html {
            html! {
                <div>
                    <H5>{"Props"}</H5>
                    <div>
                        <Switch
                            onclick=self.update_props(|props, _| ExampleProps {
                                interactive: !props.interactive,
                                ..props
                            })
                            checked=self.props.interactive
                            label="Toggle interaction"
                        />
                        <p>{"Elevation:"}</p>
                        <Menu>
                            <MenuItem
                                onclick=self.update_props(|props, _| ExampleProps {
                                    elevation: Elevation::Level0,
                                    ..props
                                })
                                text=html!{"Level 0"}
                            />
                            <MenuItem
                                onclick=self.update_props(|props, _| ExampleProps {
                                    elevation: Elevation::Level1,
                                    ..props
                                })
                                text=html!{"Level 1"}
                            />
                            <MenuItem
                                onclick=self.update_props(|props, _| ExampleProps {
                                    elevation: Elevation::Level2,
                                    ..props
                                })
                                text=html!{"Level 2"}
                            />
                            <MenuItem
                                onclick=self.update_props(|props, _| ExampleProps {
                                    elevation: Elevation::Level3,
                                    ..props
                                })
                                text=html!{"Level 3"}
                            />
                            <MenuItem
                                onclick=self.update_props(|props, _| ExampleProps {
                                    elevation: Elevation::Level4,
                                    ..props
                                })
                                text=html!{"Level 4"}
                            />
                        </Menu>
                    </div>
                </div>
            }
        }
}
