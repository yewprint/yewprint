mod example;

use crate::ExampleContainer;
use example::*;
use yew::prelude::*;
use yewprint::{Switch, H1, H5};

pub struct DividerDoc {
    callback: Callback<ExampleProps>,
    state: ExampleProps,
}

impl Component for DividerDoc {
    type Message = ExampleProps;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        DividerDoc {
            callback: link.callback(|x| x),
            state: ExampleProps { vertical: false },
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
                <H1 class=classes!("docs-title")>{"Divider"}</H1>
                <SourceCodeUrl />
                <ExampleContainer
                    source=source
                    props=Some(html! {
                        <DividerProps
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
    DividerProps for ExampleProps =>
    fn view(&self) -> Html {
        html! {
            <div>
                <H5>{"Props"}</H5>
                    <Switch
                        onclick=self.update_props(|props, _| ExampleProps {
                            vertical: !props.vertical
                        })
                        checked=self.props.vertical
                        label=html!("Vertical")
                    />
            </div>
        }
    }
}

crate::build_source_code_component!(
    "https://github.com/yewprint/yewprint/blob/main/yewprint/src/divider.rs",
    check_divider_url
);
