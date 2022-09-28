mod example;

use crate::ExampleContainer;
use example::*;
use std::borrow::Cow;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewprint::{HtmlSelect, Icon, IconName, InputGroup, Intent, Slider, H1, H5};

pub struct IconDoc {
    callback: Callback<ExampleProps>,
    state: ExampleProps,
}

impl Component for IconDoc {
    type Message = ExampleProps;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        IconDoc {
            callback: ctx.link().callback(|x| x),
            state: ExampleProps {
                icon_name: "Print".to_string(),
                intent: None,
                icon_size: 16,
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

        let icon_list = IconName::iter()
            .map(|x| {
                html! {
                    <div>
                        <Icon
                            icon={x}
                        />
                    </div>
                }
            })
            .collect::<Vec<_>>();

        html! {
            <div>
                <H1 class={classes!("docs-title")}>{"Icon"}</H1>
                <SourceCodeUrl />
                <ExampleContainer
                    source={source}
                    props={Some(html! {
                        <IconProps
                            callback={self.callback.clone()}
                            example_props={example_props.clone()}
                        />
                    })}
                >
                        <Example ..example_props />
                </ExampleContainer>
                <div>
                    <InputGroup
                        fill={true}
                        round={true}
                        left_icon={IconName::Search}
                        placeholder={"Search for icons..."}
                    />
                    {icon_list}
                </div>
            </div>
        }
    }
}

crate::build_example_prop_component! {
    IconProps for ExampleProps =>
        fn view(&self, ctx: &Context<Self>) -> Html {
            let option_labels = (0..=100)
            .step_by(1)
            .map(|x| (x, (x % 20 == 0).then(|| format!("{}", x).into())))
            .collect::<Vec<_>>();

            html! {
                <div>
                    <H5>{"Props"}</H5>
                    <div>
                        <p>{"Icon:"}</p>
                        <input
                            class="bp3-input"
                            onchange={self.update_props(ctx, |props, e: Event| {
                                if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                                    ExampleProps {
                                        icon_name: input.value(),
                                        ..props
                                    }
                                } else {
                                    ExampleProps {
                                        icon_name: "Blank".to_string(),
                                        ..props
                                    }
                                }
                            })}
                            type="text"
                            value={ctx.props().example_props.icon_name.clone()}
                        />
                        <p
                            style="margin-top: 5px;"
                        >
                            {"Select intent:"}
                        </p>
                        <HtmlSelect<Option<Intent>>
                            options={vec![
                                (None, "None".to_string()),
                                (Some(Intent::Primary), "Primary".to_string()),
                                (Some(Intent::Success), "Success".to_string()),
                                (Some(Intent::Warning), "Warning".to_string()),
                                (Some(Intent::Danger), "Danger".to_string()),
                            ]}
                            value={self.example_props.intent}
                            onchange={self.update_props(ctx, |props, intent| ExampleProps {
                                intent,
                                ..props
                            })}
                        />
                        <p
                            style="margin-top: 5px;"
                        >
                            {"Select icon size"}
                        </p>
                        <Slider<i32>
                            selected={ctx.props().example_props.icon_size}
                            values={option_labels}
                            onchange={self.update_props(ctx, |props, icon_size| ExampleProps {
                                icon_size,
                                ..props
                            })}
                            value_label={
                                Cow::Owned(format!("{}", ctx.props().example_props.icon_size))
                            }
                        />
                    </div>
                </div>
            }
        }
}

crate::build_source_code_component!();
