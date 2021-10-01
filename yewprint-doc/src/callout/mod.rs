mod example;

use crate::ExampleContainer;
use example::*;
use yew::prelude::*;
use yewprint::{HtmlSelect, Intent, Switch, H1, H5};

pub struct CalloutDoc {
    callback: Callback<ExampleProps>,
    state: ExampleProps,
}

impl Component for CalloutDoc {
    type Message = ExampleProps;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        CalloutDoc {
            callback: link.callback(|x| x),
            state: ExampleProps {
                show_icon: false,
                intent: None,
                show_title: true,
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
                <H1 class=classes!("docs-title")>{"Callout"}</H1>
                <SourceCodeUrl />
                <ExampleContainer
                    source=source
                    props=Some(html! {
                        <CalloutProps
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
    CalloutProps for ExampleProps =>
        fn view(&self) -> Html {
            html! {
                <div>
                    <H5>{"Props"}</H5>
                    <div>
                        <Switch
                            onclick=self.update_props(|props, _| ExampleProps {
                                show_icon: !props.show_icon,
                                ..props
                            })
                            checked=self.props.show_icon
                            label=html!("Show/hide icon")
                        />
                        <Switch
                            onclick=self.update_props(|props, _| ExampleProps {
                                show_title: !props.show_title,
                                ..props
                            })
                            checked=self.props.show_title
                            label=html!("Show/hide title")
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
                </div>
            }
        }
}

crate::build_source_code_component!();
