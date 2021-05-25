use yew::prelude::*;
use yewprint::{Radio, RadioGroup};

pub struct Example {
    props: ExampleProps,
    link: ComponentLink<Self>,
    selected_value: Option<String>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ExampleProps {
    pub disabled: bool,
    pub inline: bool,
    pub large: bool,
    pub selected_value: Option<String>,
}

pub enum Msg {
    ValueUpdate(String),
}

impl Component for Example {
    type Message = Msg;
    type Properties = ExampleProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Example {
            props,
            selected_value: None,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ValueUpdate(value) => {
                    self.selected_value = Some(value);
                    true
                }
            }
        }
        false
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
                <RadioGroup<String>
                    option_children= vec![
                        (Lunch::Soup, "Soup".to_string()),
                        (Lunch::Salad, "Salad".to_string()),
                        (Lunch::Sandwich, "Sandwich".to_string()),
                    ]
                    value=self.selected_value.clone()
                />
            </div>
        }
    }
}

#[derive(Debug, Copy, Hash, PartialEq)]
pub enum Lunch {
    Soup,
    Salad,
    Sandwich,
}
