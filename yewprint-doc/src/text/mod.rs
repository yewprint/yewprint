mod example;

use crate::ExampleContainer;
use example::*;
use yew::prelude::*;
use yewprint::{Switch, H1, H5};

pub struct TextDoc {
    callback: Callback<ExampleProps>,
    state: ExampleProps,
}

impl Component for TextDoc {
    type Message = ExampleProps;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        TextDoc {
            callback: link.callback(|x| x),
            state: ExampleProps {
                ellipsize: false,
                text: String::from("Hello, world!"),
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
                <H1 class=classes!("docs-title")>{"Text"}</H1>
                <SourceCodeUrl />
                <div>
                    <ExampleContainer
                        source=source
                        props=Some(html! {
                            <TextProps
                                callback={self.callback.clone()}
                                props=example_props.clone()
                            />
                        })
                    >
                        <Example with example_props />
                    </ExampleContainer>
                </div>
            </div>
        }
    }
}

crate::build_example_prop_component! {
    TextProps for ExampleProps =>
        fn view(&self) -> Html {
            html! {
                <div>
                    <H5>{"Props"}</H5>
                    <Switch
                        onclick=self.update_props(|props, _| ExampleProps {
                            ellipsize: !props.ellipsize,
                            ..props
                        })
                        checked=self.props.ellipsize
                        label=html!("Ellipsize")
                    />
                    <input
                        class="bp3-input"
                        onchange=self.update_props(|props, e|
                            match e {
                                ChangeData::Value(text) => {
                                    ExampleProps {
                                        text,
                                        ..props
                                    }
                                },
                                _ => {
                                    ExampleProps {
                                        text: "Hello, world!".to_string(),
                                        ..props
                                    }
                                }
                        })
                        type="text"
                        value={self.props.text.clone()}
                    />
                </div>
            }
        }
}

crate::build_source_code_component!(
    TEXT_URL,
    "https://github.com/yewprint/yewprint/blob/main/yewprint/src/text.rs",
    check_text_url
);
