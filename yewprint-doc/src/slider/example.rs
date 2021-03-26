use yew::prelude::*;
use yewprint::{Intent, Slider};

pub struct Example {
    props: ExampleProps,
    value1: f64,
    value2: i32,
    value3: i64,
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
    Value3Update(i64),
}

impl Component for Example {
    type Message = Msg;
    type Properties = ExampleProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Example {
            props,
            value1: Default::default(),
            value2: Default::default(),
            value3: Default::default(),
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
            Msg::Value3Update(value) => {
                self.value3 = value;
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
        let percentage_labels = (0..=100)
            .step_by(10)
            .map(|x| (x, format!("{}%", x)))
            .collect::<Vec<(i32, String)>>();

        let pounds_labels = (-12_000..48_000)
            .step_by(10_000)
            .map(|x| (x, format!("£{}", x)))
            .collect::<Vec<(i64, String)>>();

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
                    label_values=Some(percentage_labels)
                    intent=self.props.intent
                    onchange=self.link.callback(|x| Msg::Value2Update(x))
                    vertical=self.props.vertical
                />
                <Slider<i64>
                    min=-12_000
                    max=48_000
                    step_size=6_000
                    value=self.value3
                    label_values=Some(pounds_labels)
                    onchange=self.link.callback(|x| Msg::Value3Update(x))
                    vertical=self.props.vertical
                />
            </>
        }
    }
}
