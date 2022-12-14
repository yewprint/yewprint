mod example;

use crate::ExampleContainer;
use example::*;
use implicit_clone::unsync::{IArray, IString};
use yew::prelude::*;
use yewprint::{Button, ButtonGroup, HtmlSelect, IconName, Intent, Switch, H1, H5};

pub struct TagDoc {
    callback: Callback<ExampleProps>,
    state: ExampleProps,
}

fn initial_tags() -> IArray<AttrValue> {
    [
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
    .into_iter()
    .collect::<IArray<_>>()
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

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        self.state = msg;
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
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
                            example_props={example_props.clone()}
                        />
                    })}
                >
                    <Example ..example_props/>
                </ExampleContainer>
            </div>
        }
    }
}

crate::build_example_prop_component! {
    TagProps for ExampleProps =>
        fn view(&self, ctx: &Context<Self>) -> Html {
            html! {
                <div>
                    <H5>{"Props"}</H5>
                    <div>
                        <Switch
                            onclick={self.update_props(ctx, |props, _| ExampleProps {
                                active: !props.active,
                                ..props
                            })}
                            checked={ctx.props().example_props.active}
                            label={html!("Active")}
                        />
                        <Switch
                            onclick={self.update_props(ctx, |props, _| ExampleProps {
                                fill: !props.fill,
                                ..props
                            })}
                            checked={ctx.props().example_props.fill}
                            label={html!("Fill")}
                        />
                        <Switch
                            onclick={self.update_props(ctx, |props, _| ExampleProps {
                                interactive: !props.interactive,
                                ..props
                            })}
                            checked={ctx.props().example_props.interactive}
                            label={html!("Interactive")}
                        />
                        <Switch
                            onclick={self.update_props(ctx, |props, _| ExampleProps {
                                large: !props.large,
                                ..props
                            })}
                            checked={ctx.props().example_props.large}
                            label={html!("Large")}
                        />
                        <Switch
                            onclick={self.update_props(ctx, |props, _| ExampleProps {
                                minimal: !props.minimal,
                                ..props
                            })}
                            checked={ctx.props().example_props.minimal}
                            label={html!("Minimal")}
                        />
                        <Switch
                            onclick={self.update_props(ctx, |props, _| ExampleProps {
                                multiline: !props.multiline,
                                ..props
                            })}
                            checked={ctx.props().example_props.multiline}
                            label={html!("Multiline")}
                        />
                        <Switch
                            onclick={self.update_props(ctx, |props, _| ExampleProps {
                                round: !props.round,
                                ..props
                            })}
                            checked={ctx.props().example_props.round}
                            label={html!("Round")}
                        />
                        <Switch
                            onclick={self.update_props(ctx, |props, _| ExampleProps {
                                removable: !props.removable,
                                ..props
                            })}
                            checked={ctx.props().example_props.removable}
                            label={html!("Removable")}
                        />
                        <Switch
                            onclick={self.update_props(ctx, |props, _| ExampleProps {
                                icon: !props.icon,
                                ..props
                            })}
                            checked={ctx.props().example_props.icon}
                            label={html!("Icon")}
                        />
                        <Switch
                            onclick={self.update_props(ctx, |props, _| ExampleProps {
                                right_icon: !props.right_icon,
                                ..props
                            })}
                            checked={ctx.props().example_props.right_icon}
                            label={html!("Right icon")}
                        />
                        <p>{"Select intent:"}</p>
                        <ButtonGroup
                            vertical=true
                        >
                            <HtmlSelect<Option<Intent>>
                                options={IArray::Static(&[
                                    (None, IString::Static("None")),
                                    (Some(Intent::Primary), IString::Static("Primary")),
                                    (Some(Intent::Success), IString::Static("Success")),
                                    (Some(Intent::Warning), IString::Static("Warning")),
                                    (Some(Intent::Danger), IString::Static("Danger")),
                                ])}
                                onchange={self.update_props(ctx, |props, intent| ExampleProps {
                                    intent,
                                    ..props
                                })}
                                value={ctx.props().example_props.intent}
                            />
                            <Button
                                icon={IconName::Refresh}
                                onclick={self.update_props(ctx, |props, _| ExampleProps {
                                    reset_tags: props.reset_tags + 1,
                                    ..props.clone()
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
