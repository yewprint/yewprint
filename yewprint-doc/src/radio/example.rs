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
                if let Some(mut selected) = self.props.selected_value.clone() {
                    selected = value;
                    true
                } else {
                    false
                }
            }
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
                <RadioGroup<String>
                    option_children= vec![
                        ("one".to_string(), "Soup".to_string()),
                        ("two".to_string(), "Salad".to_string()),
                        ("three".to_string(), "Sandwich".to_string()),
                    ]
                    value=self.selected_value.clone()
                />
            </div>
        }
    }
}
