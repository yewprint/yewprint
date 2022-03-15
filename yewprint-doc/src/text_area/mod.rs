mod example;

use crate::ExampleContainer;
use example::*;
use yew::prelude::*;
use yewprint::{HtmlSelect, Intent, Switch, H1, H5};

pub struct TextAreaDoc {
    callback: Callback<ExampleProps>,
    state: ExampleProps,
}

impl Component for TextAreaDoc {
    type Message = ExampleProps;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        TextAreaDoc {
            callback: ctx.link().callback(|x| x),
            state: ExampleProps {
                intent: None,
                large: false,
                small: false,
                fill: false,
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
                <H1 class={classes!("docs-title")}>{"Text"}</H1>
                <SourceCodeUrl />
                <div>
                    <ExampleContainer
                        source={source}
                        props={Some(html! {
                            <TextAreaProps
                                callback={self.callback.clone()}
                            />
                        })}
                    >
                        <Example ..example_props />
                    </ExampleContainer>
                </div>
            </div>
        }
    }
}

crate::build_example_prop_component! {
    TextAreaProps for ExampleProps =>
        fn view(&self, _ctx: &Context<Self>) -> Html {
            html! {
                <div>
                    <H5>{"Props"}</H5>
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
                            large: !props.large,
                            ..props
                        })}
                        checked={self.props.large}
                        label={html!("Large")}
                    />
                    <Switch
                        onclick={self.update_props(|props, _| ExampleProps {
                            small: !props.small,
                            ..props
                        })}
                        checked={self.props.small}
                        label={html!("Small")}
                    />
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
                </div>
            }
        }
}

crate::build_source_code_component!();
