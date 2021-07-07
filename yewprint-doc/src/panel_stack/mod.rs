mod example;

use crate::ExampleContainer;
use example::*;
use yew::prelude::*;
use yewprint::H1;

pub struct PanelStackDoc {
    callback: Callback<ExampleProps>,
    state: ExampleProps,
}

impl Component for PanelStackDoc {
    type Message = ExampleProps;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        PanelStackDoc {
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
                <H1 class=classes!("docs-title")>{"PanelStack"}</H1>
                <SourceCodeUrl />
                <div>
                    <ExampleContainer
                        source=source
                        props=None
                    >
                        <Example with example_props />
                    </ExampleContainer>
                </div>
            </div>
        }
    }
}

crate::build_source_code_component!("main", "panel_stack");
