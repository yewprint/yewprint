mod example;

use crate::ExampleContainer;
use example::*;
use yew::prelude::*;
use yewprint::{Switch, H1, H5};

pub struct InputGroupDoc {
    callback: Callback<ExampleProps>,
    state: ExampleProps,
}

impl Component for InputGroupDoc {
    type Message = ExampleProps;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        InputGroupDoc {
            callback: link.callback(|x| x),
            state: ExampleProps {
                disabled: false,
                fill: false,
                large: false,
                small: false,
                round: false,
            },
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        self.state = msg;
        true
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        true
    }

    fn view(&self) -> Html {
        let example_props = self.state.clone();
        let source = crate::include_raw_html!(
            concat!(env!("OUT_DIR"), "/", file!(), ".html"),
            "bp3-code-block"
        );

        html! {
            <div>
                <H1 class={classes!("docs-title")}>{"InputGroup"}</H1>
                <SourceCodeUrl />
                <ExampleContainer
                    source={source}
                    props={Some(html! {
                        <InputGroupProps
                            callback={self.callback.clone()}
                            props=example_props.clone()
                        >
                        </InputGroupProps>
                    })}
                >
                    <Example with example_props />
                </ExampleContainer>
            </div>
        }
    }
}

crate::build_example_prop_component! {
    InputGroupProps for ExampleProps =>
    fn view(&self) -> Html {
        html! {
            <div>
                <H5>{"Props"}</H5>
                <Switch
                    onclick={self.update_props(|props, _| ExampleProps {
                        disabled: !props.disabled,
                        ..props
                    })}
                    checked={self.props.disabled}
                    label={html!("Disabled")}
                />
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
                <Switch
                    onclick={self.update_props(|props, _| ExampleProps {
                        round: !props.round,
                        ..props
                    })}
                    checked={self.props.round}
                    label={html!("Round")}
                />
            </div>
        }
    }
}

crate::build_source_code_component!();
