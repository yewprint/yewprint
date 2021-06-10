use yew::prelude::*;
use yewprint::{PanelStack, Text};

pub struct Example {
    link: ComponentLink<Self>,
    props: ExampleProps,
    selected: Civilization,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ExampleProps {
    pub animate: bool,
    pub vertical: bool,
}

impl Component for Example {
    type Message = Civilization;
    type Properties = ExampleProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Example {
            link,
            props,
            selected: Civilization::Minoan,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        if self.selected != msg {
            self.selected = msg;
            true
        } else {
            false
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <PanelStack
                    title=html! {
                        <Text class=classes!("bp3-heading") ellipsize=true>
                            {"Hello World"}
                        </Text>
                    }
                >
                    {"Content"}
                </PanelStack>
            </div>
        }
    }
}

#[derive(Clone, Copy, PartialEq, Hash)]
pub enum Civilization {
    Sumer,
    Minoan,
    AncientEgypt,
    IndusValley,
}
