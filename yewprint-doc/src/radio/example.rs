use yew::prelude::*;
use yewprint::{Label, Radio, RadioGroup};

pub struct Example {
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

    fn create(_ctx: &Context<Self>) -> Self {
        Example {
            selected_value: Lunch::Salad,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ValueUpdate(value) => {
                self.selected_value = value;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div>
                    <Radio
                        label={html!("Blue pill")}
                        inline={ctx.props().inline}
                        disabled={ctx.props().disabled}
                        large={ctx.props().large}
                        name={"group".to_string()}
                    />
                    <Radio
                        label={html!("Red pill")}
                        inline={ctx.props().inline}
                        disabled={ctx.props().disabled}
                        large={ctx.props().large}
                        name={"group".to_string()}
                    />
                </div>
                <div>
                    <RadioGroup<Lunch>
                        label={Some(html!(
                            <Label>
                                {"Determine Lunch"}
                            </Label>
                        ))}
                        options={vec![
                            (Lunch::Soup, "Soup".into()),
                            (Lunch::Salad, "Salad".into()),
                            (Lunch::Sandwich, "Sandwich".into()),
                        ]}
                        value={self.selected_value}
                        onchange={ctx.link().callback(|v| Msg::ValueUpdate(v))}
                        inline={ctx.props().inline}
                        disabled={ctx.props().disabled}
                        large={ctx.props().large}
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
