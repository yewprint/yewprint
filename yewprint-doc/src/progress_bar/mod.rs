mod example;

use crate::ExampleContainer;
use example::*;
use yew::prelude::*;
use yewprint::{HtmlSelect, Intent, Switch, H1, H5};

pub struct ProgressBarDoc {
    callback: Callback<ExampleProps>,
    state: ExampleProps,
}

impl Component for ProgressBarDoc {
    type Message = ExampleProps;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ProgressBarDoc {
            callback: ctx.link().callback(|x| x),
            state: ExampleProps {
                intent: None,
                animate: false,
                stripes: false,
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
                <H1 class={classes!("docs-title")}>{"ProgressBar"}</H1>
                <SourceCodeUrl />
                <ExampleContainer
                    source={source}
                    props={Some(html! {
                        <ProgressBarProps
                            callback={self.callback.clone()}
                            props={example_props.clone()}
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
    ProgressBarProps for ExampleProps =>
        fn view(&self, _ctx: &Context<Self>) -> Html {
            html! {
                <div>
                    <H5>{"Props"}</H5>
                    <div>
                        <Switch
                            onclick={self.update_props(ctx.props(), |props, _| ExampleProps {
                                stripes: !props.stripes,
                                ..props
                            })}
                            checked={ctx.props().stripes}
                            label={html!("Stripes")}
                        />
                        <Switch
                            onclick={self.update_props(ctx.props(), |props, _| ExampleProps {
                                animate: !props.animate,
                                ..props
                            })}
                            checked={ctx.props().animate}
                            label={html!("Animate")}
                        />
                        <p>{"Select intent:"}</p>
                        <HtmlSelect<Option<Intent>>
                            options={vec![
                                (None, "None".to_string()),
                                (Some(Intent::Primary), "Primary".to_string()),
                                (Some(Intent::Success), "Success".to_string()),
                                (Some(Intent::Warning), "Warning".to_string()),
                                (Some(Intent::Danger), "Danger".to_string()),
                            ]}
                            onchange={self.update_props(ctx.props(), |props, intent| ExampleProps {
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
