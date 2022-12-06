mod example;

use crate::ExampleContainer;
use example::*;
use implicit_clone::sync::IArray;
use std::sync::Arc;
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
    ProgressBarProps for ExampleProps =>
        fn view(&self, ctx: &Context<Self>) -> Html {
            html! {
                <div>
                    <H5>{"Props"}</H5>
                    <div>
                        <Switch
                            onclick={self.update_props(ctx, |props, _| ExampleProps {
                                stripes: !props.stripes,
                                ..props
                            })}
                            checked={ctx.props().example_props.stripes}
                            label={html!("Stripes")}
                        />
                        <Switch
                            onclick={self.update_props(ctx, |props, _| ExampleProps {
                                animate: !props.animate,
                                ..props
                            })}
                            checked={ctx.props().example_props.animate}
                            label={html!("Animate")}
                        />
                        <p>{"Select intent:"}</p>
                        <HtmlSelect<Option<Intent>>
                            options={IArray::<(Option<Intent>, AttrValue)>::Rc(Arc::new([
                                (None, "None".into()),
                                (Some(Intent::Primary), "Primary".into()),
                                (Some(Intent::Success), "Success".into()),
                                (Some(Intent::Warning), "Warning".into()),
                                (Some(Intent::Danger), "Danger".into()),
                            ]))}
                            value={self.example_props.intent}
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
