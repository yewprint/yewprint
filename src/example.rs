use crate::{Button, Collapse, IconName, Intent};
use yew::prelude::*;

pub struct ExampleContainer {
    collapsed: bool,
    props: Props,
    link: ComponentLink<Self>,
}

pub enum Msg {
    ToggleSource,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub source: &'static str,
    pub children: html::Children,
}

impl Component for ExampleContainer {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ExampleContainer {
            collapsed: true,
            props,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ToggleSource => self.collapsed ^= true,
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // TODO: never re-render this component? How to optimize this
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="docs-example-wrapper">
                <div class="docs-example">
                    {self.props.children.clone()}
                </div>
                <div class="docs-source">
                    <Button
                        icon=IconName::Code
                        fill={true}
                        intent={Intent::Primary}
                        minimal={true}
                        onclick=self.link.callback(|_| Msg::ToggleSource)
                    >
                        {"View source"}
                    </Button>
                    <Collapse
                        is_open=!self.collapsed
                        keep_children_mounted=true
                    >
                        <pre class="bp3-code-block">{self.props.source}</pre>
                    </Collapse>
                </div>
            </div>
        }
    }
}

#[macro_export]
macro_rules! include_example {
    ($file:expr) => {{
        use crate::ExampleContainer;

        let source = include_str!($file);

        mod source {
            // TODO: example.rs files are not formatted because of this include
            include!($file);
        }
        use source::Example;

        html! {
            <ExampleContainer source={source}>
                <Example />
            </ExampleContainer>
        }
    }};
}
