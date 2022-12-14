mod example;

use crate::ExampleContainer;
use example::*;
use implicit_clone::unsync::IArray;
use yew::prelude::*;
use yewprint::{HtmlSelect, Intent, Slider, H1, H5};

pub struct SpinnerDoc {
    callback: Callback<ExampleProps>,
    state: ExampleProps,
}

impl Component for SpinnerDoc {
    type Message = ExampleProps;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        SpinnerDoc {
            callback: ctx.link().callback(|x| x),
            state: ExampleProps {
                intent: None,
                size: 50,
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
                <H1 class={classes!("docs-title")}>{"Spinner"}</H1>
                <SourceCodeUrl />
                <div>
                    <ExampleContainer
                        source={source}
                        props={Some(html! {
                            <SpinnerProps
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
    SpinnerProps for ExampleProps =>
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <H5>{"Props"}</H5>
                <div>
                    <p>{"Select intent:"}</p>
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
                    <p
                        style="margin-top: 5px;"
                    >
                        {"Select Size:"}
                    </p>
                    <Slider<u32>
                        selected={ctx.props().example_props.size}
                        values={[
                            (10, Some("10".into())),
                            (20, None),
                            (30, None),
                            (40, None),
                            (50, Some("50".into())),
                            (60, None),
                            (70, None),
                            (80, None),
                            (90, None),
                            (100, Some("100".into())),
                        ].into_iter().collect::<IArray<_>>()}
                        onchange={self.update_props(ctx, |props, size| ExampleProps {
                            size,
                            ..props
                        })}
                    />
                </div>
            </div>
        }
    }
}

crate::build_source_code_component!();
