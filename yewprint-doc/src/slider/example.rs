use std::rc::Rc;
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
                    label_step_size=1.0
                    label_renderer=Rc::new(Box::new(|value: f64| format!("{:.1}", value)))
                    value=self.value1
                    intent=self.props.intent
                    onchange=self.link.callback(|x| Msg::Value1Update(x))
                    vertical=self.props.vertical
                />
                <Slider<i32>
                    min=0
                    max=100
                    step_size=1
                    label_step_size=25
                    label_renderer=Rc::new(Box::new(|value: i32| format!("{}%", value)))
                    value=self.value2
                    intent=self.props.intent
                    onchange=self.link.callback(|x| Msg::Value2Update(x))
                    vertical=self.props.vertical
                />
            </>
        }
    }
}
