mod example;

use crate::ExampleContainer;
use example::*;
use yew::prelude::*;
use yewprint::{Intent,Button, Menu, MenuItem, Switch, H1,H5};

pub struct TagDoc {
    callback: Callback<ExampleProps>,
    state: ExampleProps,
}

pub enum TagDocMsg {
    Props(ExampleProps),
    RemoveTag(String),
    Nothing,
}

fn initial_tags() -> Vec<String> {
    vec!["Landscape".into(), "Bird".into(), "City".into(), "Bridge".into(),"Street".into(),
    "Why do you go away? So that you can come back. So that you can see the place you came from with new eyes and extra colors. And the people there see you differently, too. Coming back to where you started is not the same as never leaving.".into()]
}

impl Component for TagDoc {
    type Message = TagDocMsg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        TagDoc {
            callback: link.callback(|x| TagDocMsg::Props(x)),
            state: ExampleProps {
                parent: link.callback(|msg| {
                    match msg {
                        // FIXME do something interesting with `onclick`.
                        None => TagDocMsg::Nothing,
                        Some(l) => TagDocMsg::RemoveTag(l),
                    }
                }),
                tags: initial_tags(),
                active: false,
                fill: false,
                icon: false.into(),
                intent: None,
                interactive: false,
                large: false,
                minimal: false,
                multiline: false,
                removable: false.into(),
                right_icon: false.into(),
                round: false,
            },
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            TagDocMsg::Props(props) => self.state = props,
            TagDocMsg::RemoveTag(label) => self.state.tags = self.state.tags.clone().into_iter().filter(|l| *l != label).collect(),
            TagDocMsg::Nothing => (),
        }
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
                            onclick=self.update_props(|props, _| ExampleProps {
                                active: !props.active,
                                ..props
                            })
                            checked=self.props.active
                            label="active"
                        />
                        <Switch
                            onclick=self.update_props(|props, _| ExampleProps {
                                fill: !props.fill,
                                ..props
                            })
                            checked=self.props.fill
                            label="fill"
                        />
                        <Switch
                            onclick=self.update_props(|props, _| ExampleProps {
                                interactive: !props.interactive,
                                ..props
                            })
                            checked=self.props.interactive
                            label="interactive"
                        />
                        <Switch
                            onclick=self.update_props(|props, _| ExampleProps {
                                large: !props.large,
                                ..props
                            })
                            checked=self.props.large
                            label="large"
                        />
                        <Switch
                            onclick=self.update_props(|props, _| ExampleProps {
                                minimal: !props.minimal,
                                ..props
                            })
                            checked=self.props.minimal
                            label="minimal"
                        />
                        <Switch
                            onclick=self.update_props(|props, _| ExampleProps {
                                multiline: !props.multiline,
                                ..props
                            })
                            checked=self.props.multiline
                            label="multiline"
                        />
                        <Switch
                            onclick=self.update_props(|props, _| ExampleProps {
                                round: !props.round,
                                ..props
                            })
                            checked=self.props.round
                            label="round"
                        />
                        <Switch
                            onclick=self.update_props(|props, _| ExampleProps {
                                removable: !props.removable,
                                ..props
                            })
                            checked=self.props.removable
                            label="removable"
                        />
                        <Switch
                            onclick=self.update_props(|props, _| ExampleProps {
                                icon: !props.icon,
                                ..props
                            })
                            checked=self.props.icon
                            label="icon"
                        />
                        <Switch
                            onclick=self.update_props(|props, _| ExampleProps {
                                right_icon: !props.right_icon,
                                ..props
                            })
                            checked=self.props.right_icon
                            label="right icon"
                        />
                        <Button
                            onclick=self.update_props(|props, _| ExampleProps {
                                tags: initial_tags(),
                                ..props
                            })
                            minimal=self.props.minimal
                            fill=self.props.fill
                        >
                            {"reset tags"}
                        </Button>
                        <p>{"Select intent:"}</p>
                        <Menu>
                            <MenuItem
                                onclick=self.update_props(|props, _| ExampleProps {
                                    intent: None,
                                    ..props
                                })
                                text=html!{"None"}
                            />
                            <MenuItem
                                onclick=self.update_props(|props, _| ExampleProps {
                                    intent: Some(Intent::Primary),
                                    ..props
                                })
                                text=html!{"Primary"}
                                intent=Intent::Primary
                            />
                            <MenuItem
                                onclick=self.update_props(|props, _| ExampleProps {
                                    intent: Some(Intent::Success),
                                    ..props
                                })
                                text=html!{"Success"}
                                intent=Intent::Success
                            />
                            <MenuItem
                                onclick=self.update_props(|props, _| ExampleProps {
                                    intent: Some(Intent::Warning),
                                    ..props
                                })
                                text=html!{"Warning"}
                                intent=Intent::Warning
                            />
                            <MenuItem
                                onclick=self.update_props(|props, _| ExampleProps {
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
