mod example;

use crate::ExampleContainer;
use example::*;
use yew::prelude::*;
use yewprint::{HtmlSelect, Intent, Switch, H1, H5};

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
