use yew::prelude::*;
use yewprint::{Intent, Slider};

pub struct Example {
    props: ExampleProps,
    value1: f64,
    value2: i32,
    link: ComponentLink<Self>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ExampleProps {
    pub vertical: bool,
    pub intent: Option<Intent>,
}

pub enum Msg {
    Value1Update(f64),
    Value2Update(i32),
}

impl Component for Example {
    type Message = Msg;
    type Properties = ExampleProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Example {
            props,
            value1: Default::default(),
            value2: Default::default(),
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Value1Update(value) => {
                self.value1 = value;
            }
            Msg::Value2Update(value) => {
                self.value2 = value;
            }
        }
        true
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
                <Slider<f64>
                    min=0.0
                    max=1.0
                    step_size=0.1
                    value=self.value1
                    label_values=Some(vec![
                        (0.0, String::from("0.0")),
                        (0.1, String::from("0.1")),
                        (0.2, String::from("0.2")),
                        (0.3, String::from("0.3")),
                        (0.4, String::from("0.4")),
                        (0.5, String::from("0.5")),
                        (0.6, String::from("0.6")),
                        (0.7, String::from("0.7")),
                        (0.8, String::from("0.8")),
                        (0.9, String::from("0.9")),
                        (1.0, String::from("1.0")),
                    ])
                    intent=self.props.intent
                    onchange=self.link.callback(|x| Msg::Value1Update(x))
                    vertical=self.props.vertical
                />
                <Slider<i32>
                    min=0
                    max=100
                    step_size=1
                    value=self.value2
                    label_values=Some(vec![
                        (0, String::from("0%")),
                        (10, String::from("10%")),
                        (20, String::from("10%")),
                        (30, String::from("10%")),
                        (40, String::from("10%")),
                        (50, String::from("10%")),
                        (60, String::from("10%")),
                        (70, String::from("10%")),
                        (80, String::from("10%")),
                        (90, String::from("10%")),
                        (100, String::from("100%")),
                    ])
                    intent=self.props.intent
                    onchange=self.link.callback(|x| Msg::Value2Update(x))
                    vertical=self.props.vertical
                />
            </>
        }
    }
}
