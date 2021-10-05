use yew::prelude::*;
use yewprint::{Button, NumericInput};

pub struct Example {
    props: ExampleProps,
    link: ComponentLink<Self>,
    first_value: i32,
    second_value: f32,
}

pub enum Msg {
    Reset,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ExampleProps {
    pub fill: bool,
    pub disabled: bool,
    pub large: bool,
    pub first_min_value: i32,
    pub first_max_value: i32,
    pub first_increment: i32,
    pub second_min_value: f32,
    pub second_max_value: f32,
    pub second_increment: f32,
}

impl Component for Example {
    type Message = Msg;
    type Properties = ExampleProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Example {
            props,
            link,
            first_value: 0,
            second_value: 0.0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Reset => {
                self.first_value = 0;
                self.second_value = 0.0;
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
                <NumericInput<i32>
                    disabled=self.props.disabled
                    fill=self.props.fill
                    large=self.props.large
                    min_value=self.props.first_min_value
                    max_value=self.props.first_max_value
                    increment=self.props.first_increment
                    value=self.first_value
                    placeholder=String::from("Enter an integer...")
                />
                <NumericInput<f32>
                    disabled=self.props.disabled
                    fill=self.props.fill
                    large=self.props.large
                    min_value=self.props.second_min_value
                    max_value=self.props.second_max_value
                    increment=self.props.second_increment
                    value=self.second_value
                    placeholder=String::from("Enter a floating point number...")
                />
                <Button
                    onclick=self.link.callback(|_| Msg::Reset)
                >
                    {"Reset"}
                </Button>
            </>
        }
    }
}
