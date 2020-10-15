mod example;

use crate::ExampleContainer;
use example::*;
use yew::prelude::*;
use yewprint::{Button, Intent, Menu, MenuItem, Switch, H1, H5, IconName};

pub struct TagDoc {
    callback: Callback<ExampleProps>,
    state: ExampleProps,
}

fn initial_tags() -> Vec<String> {
    vec![
        "Landscape".into(),
        "Bird".into(),
        "City".into(),
        "Bridge".into(),
        "Street".into(),
        "Why do you go away? So that you can come back. So that you can see the place you \
        came from with new eyes and extra colors. And the people there see you differently, \
        too. Coming back to where you started is not the same as never leaving."
            .into(),
    ]
}

impl Component for TagDoc {
    type Message = ExampleProps;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        TagDoc {
            callback: link.callback(|x| x),
            state: ExampleProps {
                initial_tags: initial_tags(),
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
                reset_tags: 0,
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
                            label="Active"
                        />
                        <Switch
                            onclick=self.update_props(|props, _| ExampleProps {
                                fill: !props.fill,
                                ..props
                            })
                            checked=self.props.fill
                            label="Fill"
                        />
                        <Switch
                            onclick=self.update_props(|props, _| ExampleProps {
                                interactive: !props.interactive,
                                ..props
                            })
                            checked=self.props.interactive
                            label="Interactive"
                        />
                        <Switch
                            onclick=self.update_props(|props, _| ExampleProps {
                                large: !props.large,
                                ..props
                            })
                            checked=self.props.large
                            label="Large"
                        />
                        <Switch
                            onclick=self.update_props(|props, _| ExampleProps {
                                minimal: !props.minimal,
                                ..props
                            })
                            checked=self.props.minimal
                            label="Minimal"
                        />
                        <Switch
                            onclick=self.update_props(|props, _| ExampleProps {
                                multiline: !props.multiline,
                                ..props
                            })
                            checked=self.props.multiline
                            label="Multiline"
                        />
                        <Switch
                            onclick=self.update_props(|props, _| ExampleProps {
                                round: !props.round,
                                ..props
                            })
                            checked=self.props.round
                            label="Round"
                        />
                        <Switch
                            onclick=self.update_props(|props, _| ExampleProps {
                                removable: !props.removable,
                                ..props
                            })
                            checked=self.props.removable
                            label="Removable"
                        />
                        <Switch
                            onclick=self.update_props(|props, _| ExampleProps {
                                icon: !props.icon,
                                ..props
                            })
                            checked=self.props.icon
                            label="Icon"
                        />
                        <Switch
                            onclick=self.update_props(|props, _| ExampleProps {
                                right_icon: !props.right_icon,
                                ..props
                            })
                            checked=self.props.right_icon
                            label="Right icon"
                        />
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
                        <Button
                            icon=IconName::Refresh
                            onclick=self.update_props(|props, _| ExampleProps {
                                reset_tags: props.reset_tags + 1,
                                ..props
                            })
                        >
                            {"Reset tags"}
                        </Button>
                    </div>
                </div>
            }
        }
}
