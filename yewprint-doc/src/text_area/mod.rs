mod example;

use crate::ExampleContainer;
use example::*;
use yew::prelude::*;
use yewprint::{HtmlSelect, Intent, Switch, Text, H1, H5};

pub struct TextAreaDoc {
    callback: Callback<ExampleProps>,
    state: ExampleProps,
}

impl Component for TextAreaDoc {
    type Message = ExampleProps;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        TextAreaDoc {
            callback: link.callback(|x| x),
            state: ExampleProps {
                intent: None,
                large: false,
                small: false,
                fill: false,
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
                <SourceCodeUrl
                    url=TEXT_AREA_URL
                />
                <div>
                    <ExampleContainer
                        source=source
                        props=Some(html! {
                            <TextAreaProps
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
    TextAreaProps for ExampleProps =>
        fn view(&self) -> Html {
            html! {
                <div>
                    <H5>{"Props"}</H5>
                    <Switch
                        onclick=self.update_props(|props, _| ExampleProps {
                            fill: !props.fill,
                            ..props
                        })
                        checked=self.props.fill
                        label=html!("Fill")
                         />
                    <Switch
                        onclick=self.update_props(|props, _| ExampleProps {
                            large: !props.large,
                            ..props
                        })
                        checked=self.props.large
                        label=html!("Large")
                    />
                    <Switch
                        onclick=self.update_props(|props, _| ExampleProps {
                            small: !props.small,
                            ..props
                        })
                        checked=self.props.small
                        label=html!("Small")
                    />
                    <HtmlSelect<Option<Intent>>
                        options={vec![
                            (None, "None".to_string()),
                            (Some(Intent::Primary), "Primary".to_string()),
                            (Some(Intent::Success), "Success".to_string()),
                            (Some(Intent::Warning), "Warning".to_string()),
                            (Some(Intent::Danger), "Danger".to_string()),
                        ]}
                        onchange=self.update_props(|props, intent| ExampleProps {
                            intent,
                            ..props
                        })
                    />
                </div>
            }
        }
}

crate::build_source_code_component!(
    TEXT_AREA_URL,
    "https://github.com/yewprint/yewprint/blob/main/yewprint/src/text_area.rs",
    check_text_area_url
);
