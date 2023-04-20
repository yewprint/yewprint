mod example;

use crate::ExampleContainer;
use example::*;
use implicit_clone::unsync::IArray;
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
                grow_vertically: true,
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
                                example_props={example_props.clone()}
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
        fn view(&self, ctx: &Context<Self>) -> Html {
            html! {
                <div>
                    <H5>{"Props"}</H5>
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
                            large: !props.large,
                            ..props
                        })}
                        checked={ctx.props().example_props.large}
                        label={html!("Large")}
                    />
                    <Switch
                        onclick={self.update_props(ctx, |props, _| ExampleProps {
                            small: !props.small,
                            ..props
                        })}
                        checked={ctx.props().example_props.small}
                        label={html!("Small")}
                    />
                    <Switch
                        onclick={self.update_props(ctx, |props, _| ExampleProps {
                            grow_vertically: !props.grow_vertically,
                            ..props
                        })}
                        checked={ctx.props().example_props.grow_vertically}
                        label={html!("Grow vertically")}
                    />
                    <HtmlSelect<Option<Intent>>
                        options={[
                            (None, "None".into()),
                            (Some(Intent::Primary), "Primary".into()),
                            (Some(Intent::Success), "Success".into()),
                            (Some(Intent::Warning), "Warning".into()),
                            (Some(Intent::Danger), "Danger".into()),
                        ].into_iter().collect::<IArray<_>>()}
                        onchange={self.update_props(ctx, |props, intent| ExampleProps {
                            intent,
                            ..props
                        })}
                        value={ctx.props().example_props.intent}
                    />
                </div>
            }
        }
}

crate::build_source_code_component!();
