mod example;

use crate::ExampleContainer;
use example::*;
use yew::prelude::*;
use yewprint::{Intent, Menu, MenuItem, Switch, H1,H5};

pub struct TagDoc {
    callback: Callback<ExampleProps>,
    state: ExampleProps,
}

impl Component for TagDoc {
    type Message = ExampleProps;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let initial_tags = vec!["Landscape".into(), "Bird".into(), "City".into(), "Bridge".into(),"Street".into()];
        TagDoc {
            callback: link.callback(|x| x),
            state: ExampleProps {
                initial_tags: initial_tags.clone(),
                tags: initial_tags.clone(),
                active: false,
                fill: false,
                icon: false.into(),
                intent: None,
                interactive: false,
                large: false,
                minimal: false,
                multiline: false,
                //onClick,
                removable: false.into(),
                right_icon: false.into(),
                round: false,
            }
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
                <H1 class="docs-title">{"Tag"}</H1>
                <ExampleContainer 
                    source=source
                    props=Some(html!{
                        <TagProps 
                            callback={self.callback.clone()}
                            props=example_props.clone()
                        />
                    })
                >
                    <Example with example_props/>
                </ExampleContainer>
            </div>
        }
    }
}

crate::build_example_prop_component! {
    TagProps for ExampleProps =>
        fn view(&self) -> Html {
            html! {
                <div>
                    <H5>{"Props"}</H5>
                    <div>
                        <Switch
                            onclick=self.update_props(|props| ExampleProps {
                                active: !props.active,
                                ..props
                            })
                            checked=self.props.active
                            label="active"
                        />
                        <Switch
                            onclick=self.update_props(|props| ExampleProps {
                                fill: !props.fill,
                                ..props
                            })
                            checked=self.props.fill
                            label="fill"
                        />
                        <Switch
                            onclick=self.update_props(|props| ExampleProps {
                                interactive: !props.interactive,
                                ..props
                            })
                            checked=self.props.interactive
                            label="interactive"
                        />
                        <Switch
                            onclick=self.update_props(|props| ExampleProps {
                                large: !props.large,
                                ..props
                            })
                            checked=self.props.large
                            label="large"
                        />
                        <Switch
                            onclick=self.update_props(|props| ExampleProps {
                                minimal: !props.minimal,
                                ..props
                            })
                            checked=self.props.minimal
                            label="minimal"
                        />
                        <Switch
                            onclick=self.update_props(|props| ExampleProps {
                                multiline: !props.multiline,
                                ..props
                            })
                            checked=self.props.multiline
                            label="multiline"
                        />
                        <Switch
                            onclick=self.update_props(|props| ExampleProps {
                                round: !props.round,
                                ..props
                            })
                            checked=self.props.round
                            label="round"
                        />
                        <Switch
                            onclick=self.update_props(|props| ExampleProps {
                                removable: !props.removable,
                                ..props
                            })
                            checked=self.props.removable
                            label="removable"
                        />
                        <Switch
                            onclick=self.update_props(|props| ExampleProps {
                                icon: !props.icon,
                                ..props
                            })
                            checked=self.props.icon
                            label="icon"
                        />
                        <Switch
                            onclick=self.update_props(|props| ExampleProps {
                                right_icon: !props.right_icon,
                                ..props
                            })
                            checked=self.props.right_icon
                            label="right icon"
                        />
                        // FIXME Switching off options resets removed tags, move the taglist change up
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
