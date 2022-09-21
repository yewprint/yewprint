mod example;

use crate::ExampleContainer;
use example::*;
use yew::prelude::*;
use yewprint::{HtmlSelect, Intent, H1, H5};

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
            state: ExampleProps { intent: None },
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
            </div>
        }
    }
}

crate::build_example_prop_component! {
    IconProps for ExampleProps =>
        fn view(&self, ctx: &Context<Self>) -> Html {
            html! {
                <div>
                    <H5>{"Props"}</H5>
                    <div>
                        <p>{"Select intent:"}</p>
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
                    </div>
                </div>
            }
        }
}

crate::build_source_code_component!();
