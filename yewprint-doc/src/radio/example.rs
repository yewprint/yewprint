use yew::prelude::*;
use yewprint::{Label, Radio, RadioGroup};

pub struct Example {
    props: ExampleProps,
    link: ComponentLink<Self>,
    selected_value: Lunch,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ExampleProps {
    pub disabled: bool,
    pub inline: bool,
    pub large: bool,
}

pub enum Msg {
    ValueUpdate(Lunch),
}

impl Component for Example {
    type Message = Msg;
    type Properties = ExampleProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Example {
            props,
            selected_value: Lunch::Salad,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ValueUpdate(value) => {
                self.selected_value = value;
                true
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
            <>
                <div>
                    <Radio
                        label=html!("Blue pill")
                        inline=self.props.inline
                        disabled=self.props.disabled
                        large=self.props.large
                        name="group"
                    />
                    <Radio
                        label=html!("Red pill")
                        inline=self.props.inline
                        disabled=self.props.disabled
                        large=self.props.large
                        name="group"
                    />
                </div>
                <div>
                    <RadioGroup<Lunch>
                        label=Some(html!(
                            <Label>
                                {"Determine Lunch"}
                            </Label>
                        ))
                        options=vec![
                            (Lunch::Soup, "Soup".to_string()),
                            (Lunch::Salad, "Salad".to_string()),
                            (Lunch::Sandwich, "Sandwich".to_string()),
                        ]
                        value=self.selected_value.clone()
                        onchange=self.link.callback(|v| Msg::ValueUpdate(v))
                        inline=self.props.inline
                        disabled=self.props.disabled
                        large=self.props.large
                    />
                </div>
            </>
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq)]
pub enum Lunch {
    Soup,
    Salad,
    Sandwich,
}
