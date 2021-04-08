mod example;

use crate::ExampleContainer;
use example::*;
use yew::prelude::*;
use yewprint::{HtmlSelect, Intent, Switch, Text, H1, H2, H5};

pub struct SliderDoc {
    callback: Callback<ExampleProps>,
    state: ExampleProps,
}

impl Component for SliderDoc {
    type Message = ExampleProps;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        SliderDoc {
            callback: link.callback(|x| x),
            state: ExampleProps {
                vertical: false,
                intent: None,
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
                <H1 class=classes!("docs-title")>{"Slider"}</H1>
                <ExampleContainer
                    source=source
                    props=Some(html! {
                        <SliderProps
                            callback={self.callback.clone()}
                            props=example_props.clone()
                        />
                    })
                >
                    <Example with example_props />
                </ExampleContainer>
                <H2>{"Edge Cases"}</H2>
                <ul>
                    <li>
                        <div style="display: flex;">
                            <Text style="margin-right: 3px;">
                                {"The"}
                            </Text>
                            <code>{"options"}</code>
                            <Text style="margin-left: 3px;">
                                {"property is empty:"}
                            </Text>
                        </div>
                        <div style="display: flex; flex-wrap: wrap;">
                            <Text style="margin-right: 3px;">
                                {"If the component is provided an empty"}
                            </Text>
                            <code>{"Vec"}</code>
                            <Text style="margin-left: 3px;">
                                {"the slider will have no handle"}
                            </Text>
                            <Text style="margin-left: 3px;">
                                {"and the user won't be able to interact with it."}
                            </Text>
                        </div>
                    </li>
                    <li>
                        <div style="display: flex;">
                            <Text style="margin-right: 3px;">
                                {"The"}
                            </Text>
                            <code>{"options"}</code>
                            <Text style="margin-left: 3px;">
                                {"property does not contain the value property"}
                            </Text>
                        </div>
                        <div style="display: flex;">
                            <Text style="margin-right: 3px;">
                                {"If the"}
                            </Text>
                            <code>{"value"}</code>
                            <Text style="margin-left: 3px; margin-right: 3px;">
                                {"of the slider is not in the"}
                            </Text>
                            <code>{"options"}</code>
                            <Text>
                                {
                                    ", the handle will not be displayed until the user clicks \
                                    on the slider."
                                }
                            </Text>
                        </div>
                    </li>
                    <li>
                        <div style="display: flex;">
                            <Text style="margin-right: 3px;">
                                {"The"}
                            </Text>
                            <code>{"options"}</code>
                            <Text style="margin-left: 3px;">
                                {"property contains only one value:"}
                            </Text>
                        </div>
                        <div style="display: flex; flex-wrap: wrap;">
                            <Text style="margin-right: 3px;">
                                {"If the"}
                            </Text>
                            <code>{"options"}</code>
                            <Text style="margin-left: 3px;">
                                {"property contains only one value"}
                            </Text>
                            <Text style="margin-left: 3px;">
                                {
                                    "the handle and the label will be display \
                                    at the center of the slider"
                                }
                            </Text>
                            <Text>
                                {"and the user won't be able to interact with it."}
                            </Text>
                        </div>
                    </li>
                </ul>
            </div>
        }
    }
}

crate::build_example_prop_component! {
    SliderProps for ExampleProps =>
    fn view(&self) -> Html {
        html! {
            <div>
                <H5>{"Props"}</H5>
                <Switch
                    onclick=self.update_props(|props, _| ExampleProps {
                        vertical: !props.vertical,
                        ..props
                    })
                    checked=self.props.vertical
                    label=html!("Vertical")
                    disabled=true
                />
                <p>{"Select intent:"}</p>
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
