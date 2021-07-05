mod example;

use crate::ExampleContainer;
use example::*;
use yew::prelude::*;
use yewprint::{Switch, Text, H1, H5};

pub struct RadioDoc {
    callback: Callback<ExampleProps>,
    state: ExampleProps,
}

impl Component for RadioDoc {
    type Message = ExampleProps;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        RadioDoc {
            callback: link.callback(|x| x),
            state: ExampleProps {
                disabled: false,
                inline: false,
                large: false,
            },
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        self.state = msg;
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
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
                <H1 class=classes!("docs-title")>{"Radio"}</H1>
                <a
                    class=classes!("bp3-text-muted")
                    href="https://github.com/yewprint/yewprint/blob/main/yewprint/src/radio.rs"
                    target="_blank"
                >
                    <Text>{"Go to the source code"}</Text>
                </a>
                <ExampleContainer
                    source=source
                    props=Some(html! {
                        <RadioProps
                            callback={self.callback.clone()}
                            props=example_props.clone()
                        />
                    })
                >
                    <Example with example_props />
                </ExampleContainer>
            </div>
        }
    }
}

crate::build_example_prop_component! {
    RadioProps for ExampleProps =>
    fn view(&self) -> Html {
        html! {
            <div>
                <H5>{"Props"}</H5>
                <Switch
                    onclick=self.update_props(|props, _| ExampleProps {
                        disabled: !props.disabled,
                        ..props
                    })
                    checked=self.props.disabled
                    label=html!("Disabled")
                />
                <Switch
                    onclick=self.update_props(|props, _| ExampleProps {
                        inline: !props.inline,
                        ..props
                    })
                    checked=self.props.inline
                    label=html!("Inline")
                />
                <Switch
                    onclick=self.update_props(|props, _| ExampleProps {
                        large: !props.large,
                        ..props
                    })
                    checked=self.props.large
                    label=html!("Large")
                />
            </div>
        }
    }
}
