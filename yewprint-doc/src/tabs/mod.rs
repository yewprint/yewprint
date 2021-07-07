mod example;

use crate::ExampleContainer;
use example::*;
use yew::prelude::*;
use yewprint::{Switch, H1, H5};

pub struct TabsDoc {
    callback: Callback<ExampleProps>,
    state: ExampleProps,
}

impl Component for TabsDoc {
    type Message = ExampleProps;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        TabsDoc {
            callback: link.callback(|x| x),
            state: ExampleProps {
                animate: true,
                vertical: false,
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
                <H1 class=classes!("docs-title")>{"Tabs"}</H1>
                <SourceCodeUrl />
                <div>
                    <ExampleContainer
                        source=source
                        props=Some(html! {
                            <TabsProps
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
    TabsProps for ExampleProps =>
        fn view(&self) -> Html {
            html! {
                <div>
                    <H5>{"Props"}</H5>
                    <Switch
                        onclick=self.update_props(|props, _| ExampleProps {
                            animate: !props.animate,
                            ..props
                        })
                        checked=self.props.animate
                        label=html!("Animate indicator")
                    />
                    <Switch
                        onclick=self.update_props(|props, _| ExampleProps {
                            vertical: !props.vertical,
                            ..props
                        })
                        checked=self.props.vertical
                        label=html!("Use vertical tabs")
                    />
                </div>
            }
        }
}

crate::build_source_code_component!("main", "tabs");
