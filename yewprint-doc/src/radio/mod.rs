mod example;

use crate::ExampleContainer;
use example::*;
use yew::prelude::*;
use yewprint::{Switch, H1, H5};

pub struct RadioDoc {
    callback: Callback<ExampleProps>,
    state: ExampleProps,
}

impl Component for RadioDoc {
    type Message = ExampleProps;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        RadioDoc {
            callback: ctx.link().callback(|x| x),
            state: ExampleProps {
                disabled: false,
                inline: false,
                large: false,
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
                <H1 class={classes!("docs-title")}>{"Radio"}</H1>
                <SourceCodeUrl />
                <ExampleContainer
                    source={source}
                    props={Some(html! {
                        <RadioProps
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
    RadioProps for ExampleProps =>
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <H5>{"Props"}</H5>
                <Switch
                    onclick={self.update_props(ctx.props(), |props, _| ExampleProps {
                        disabled: !props.disabled,
                        ..props
                    })}
                    checked={ctx.props().example_props.disabled}
                    label={html!("Disabled")}
                />
                <Switch
                    onclick={self.update_props(ctx.props(), |props, _| ExampleProps {
                        inline: !props.inline,
                        ..props
                    })}
                    checked={ctx.props().example_props.inline}
                    label={html!("Inline")}
                />
                <Switch
                    onclick={self.update_props(ctx.props(), |props, _| ExampleProps {
                        large: !props.large,
                        ..props
                    })}
                    checked={ctx.props().example_props.large}
                    label={html!("Large")}
                />
            </div>
        }
    }
}

crate::build_source_code_component!();
