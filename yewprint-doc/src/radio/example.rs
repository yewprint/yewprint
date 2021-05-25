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
                if let Some(selected) = self.props.selected_value {
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
                <RadioGroup
                    label=html!("Determine lunch")
                    name="group"
                    selected_value=self.selected_value
                >
                    <Radio
                        disabled=self.props.disabled
                        inline=self.props.inline
                        large=self.props.large
                        label=html!("Soup")
                        onchange=self.link.callback(|value| Msg::ValueUpdate(value))
                        value="one"
                    />
                    <Radio
                        disabled=self.props.disabled
                        inline=self.props.inline
                        large=self.props.large
                        onchange=self.link.callback(|value| Msg::ValueUpdate(value))
                        value="two"
                    />
                    <Radio
                        disabled=self.props.disabled
                        inline=self.props.inline
                        large=self.props.large
                        label=html!("Sandwich")
                        onchange=self.link.callback(|value| Msg::ValueUpdate(value))
                        value="three"
                    />
                </RadioGroup>
            </div>
        }
    }
}
