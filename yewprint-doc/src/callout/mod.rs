mod example;

use crate::ExampleContainer;
use example::*;
use yew::prelude::*;
use yewprint::{HtmlSelect, Intent, Switch, H1, H5};

pub struct CalloutDoc {
    callback: Callback<ExampleProps>,
    state: ExampleProps,
}

impl Component for CalloutDoc {
    type Message = ExampleProps;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        CalloutDoc {
            callback: ctx.link().callback(|x| x),
            state: ExampleProps {
                show_icon: false,
                intent: None,
                show_title: true,
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
                <H1 class={classes!("docs-title")}>{"Callout"}</H1>
                <SourceCodeUrl />
                <ExampleContainer
                    source={source}
                    props={Some(html! {
                        <CalloutProps
                            callback={self.callback.clone()}
                            example_props={example_props.clone()}
                        />
                    })}
                >
                    <Example ..example_props />
                </ExampleContainer>
            </div>
        }
    }
}

crate::build_example_prop_component! {
    CalloutProps for ExampleProps =>
        fn view(&self, ctx: &Context<Self>) -> Html {
            html! {
                <div>
                    <H5>{"Props"}</H5>
                    <div>
                        <Switch
                            onclick={self.update_props(ctx, |props, _| ExampleProps {
                                show_icon: !props.show_icon,
                                ..props
                            })}
                            checked={ctx.props().example_props.show_icon}
                            label={html!("Show/hide icon")}
                        />
                        <Switch
                            onclick={self.update_props(ctx, |props, _| ExampleProps {
                                show_title: !props.show_title,
                                ..props
                            })}
                            checked={ctx.props().example_props.show_title}
                            label={html!("Show/hide title")}
                        />
                        <p>{"Select intent:"}</p>
                        <HtmlSelect<Option<Intent>>
                            options={vec![
                                (None, "None".into()),
                                (Some(Intent::Primary), "Primary".into()),
                                (Some(Intent::Success), "Success".into()),
                                (Some(Intent::Warning), "Warning".into()),
                                (Some(Intent::Danger), "Danger".into()),
                            ]}
                            onchange={self.update_props(ctx, |props, intent| ExampleProps {
                                intent,
                                ..props
                            })}
                        />
                    </div>
                </div>
            }
        }
}

crate::build_source_code_component!();
