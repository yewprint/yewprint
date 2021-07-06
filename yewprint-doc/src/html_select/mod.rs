mod example;

use crate::ExampleContainer;
use example::*;
use yew::prelude::*;
use yewprint::{Switch, Text, H1, H5};

pub struct HtmlSelectDoc {
    callback: Callback<ExampleProps>,
    state: ExampleProps,
}

impl Component for HtmlSelectDoc {
    type Message = ExampleProps;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        HtmlSelectDoc {
            callback: link.callback(|x| x),
            state: ExampleProps {
                minimal: false,
                fill: false,
                disabled: false,
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
                <H1 class=classes!("docs-title")>{"HtmlSelect"}</H1>
                <SourceCodeUrl
                    url=HTML_SELECT_URL
                />
                <div>
                    <ExampleContainer
                        source=source
                        props=Some(html! {
                            <HtmlSelectProps
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
    HtmlSelectProps for ExampleProps =>
        fn view(&self) -> Html {
            html! {
                <div>
                    <H5>{"Props"}</H5>
                <Switch
                    onclick=self.update_props(|props, _| ExampleProps {
                        minimal: !props.minimal,
                        ..props
                    })
                    checked=self.props.minimal
                    label=html!("Minimal")
                />
                <Switch
                    onclick=self.update_props(|props, _| ExampleProps{
                        fill: !props.fill,
                        ..props
                    })
                    checked=self.props.fill
                    label=html!("Fill")
                />
                <Switch
                    onclick=self.update_props(|props, _| ExampleProps {
                        disabled: !props.disabled,
                        ..props
                    })
                    checked=self.props.disabled
                    label=html!("Disabled")
                />
                <Switch
                    onclick=self.update_props(|props, _| ExampleProps{
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

crate::build_source_code_component!(
    HTML_SELECT_URL,
    "https://github.com/yewprint/yewprint/blob/main/yewprint/src/html_select.rs",
    check_html_select_url
);
