mod example;

use crate::ExampleContainer;
use example::*;
use yew::prelude::*;
use yewprint::{Button, ButtonGroup, HtmlSelect, IconName, Intent, Switch, H1, H5};

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

    fn create(ctx: &Context<Self>) -> Self {
        TagDoc {
            callback: ctx.link().callback(|x| x),
            state: ExampleProps {
                initial_tags: initial_tags(),
                active: false,
                fill: false,
                icon: false,
                intent: None,
                interactive: false,
                large: false,
                minimal: false,
                multiline: false,
                removable: false,
                right_icon: false,
                round: false,
                reset_tags: 0,
            },
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        self.state = msg;
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
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
                <H1 class={classes!("docs-title")}>{"Tag"}</H1>
                <SourceCodeUrl />
                <ExampleContainer
                    source={source}
                    props={Some(html!{
                        <TagProps
                            callback={self.callback.clone()}
                            props=example_props.clone()
                        />
                    })}
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
                            onclick={self.update_props(|props, _| ExampleProps {
                                active: !props.active,
                                ..props
                            })}
                            checked={self.props.active}
                            label={html!("Active")}
                        />
                        <Switch
                            onclick={self.update_props(|props, _| ExampleProps {
                                fill: !props.fill,
                                ..props
                            })}
                            checked={self.props.fill}
                            label={html!("Fill")}
                        />
                        <Switch
                            onclick={self.update_props(|props, _| ExampleProps {
                                interactive: !props.interactive,
                                ..props
                            })}
                            checked={self.props.interactive}
                            label={html!("Interactive")}
                        />
                        <Switch
                            onclick={self.update_props(|props, _| ExampleProps {
                                large: !props.large,
                                ..props
                            })}
                            checked={self.props.large}
                            label={html!("Large")}
                        />
                        <Switch
                            onclick={self.update_props(|props, _| ExampleProps {
                                minimal: !props.minimal,
                                ..props
                            })}
                            checked={self.props.minimal}
                            label={html!("Minimal")}
                        />
                        <Switch
                            onclick={self.update_props(|props, _| ExampleProps {
                                multiline: !props.multiline,
                                ..props
                            })}
                            checked={self.props.multiline}
                            label={html!("Multiline")}
                        />
                        <Switch
                            onclick={self.update_props(|props, _| ExampleProps {
                                round: !props.round,
                                ..props
                            })}
                            checked={self.props.round}
                            label={html!("Round")}
                        />
                        <Switch
                            onclick={self.update_props(|props, _| ExampleProps {
                                removable: !props.removable,
                                ..props
                            })}
                            checked={self.props.removable}
                            label={html!("Removable")}
                        />
                        <Switch
                            onclick={self.update_props(|props, _| ExampleProps {
                                icon: !props.icon,
                                ..props
                            })}
                            checked={self.props.icon}
                            label={html!("Icon")}
                        />
                        <Switch
                            onclick={self.update_props(|props, _| ExampleProps {
                                right_icon: !props.right_icon,
                                ..props
                            })}
                            checked={self.props.right_icon}
                            label={html!("Right icon")}
                        />
                        <p>{"Select intent:"}</p>
                        <ButtonGroup
                            vertical=true
                        >
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
                            <Button
                                icon={IconName::Refresh}
                                onclick={self.update_props(|props, _| ExampleProps {
                                    reset_tags: ctx.props().reset_tags + 1,
                                    ..props
                                })}
                            >
                                {"Reset tags"}
                            </Button>
                        </ButtonGroup>
                    </div>
                </div>
            }
        }
}

crate::build_source_code_component!();
