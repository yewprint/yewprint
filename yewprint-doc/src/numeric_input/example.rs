use yew::prelude::*;
use yewprint::NumericInput;

pub struct Example {
    props: ExampleProps,
    link: ComponentLink<Self>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ExampleProps {
    pub fill: bool,
    pub disabled: bool,
    pub large: bool,
    pub integer_value: i32,
    pub integer_min_value: i32,
    pub integer_max_value: i32,
    pub integer_increment: i32,
    pub float_value: f32,
    pub float_min_value: f32,
    pub float_max_value: f32,
    pub float_increment: f32,
}

impl Component for Example {
    type Message = ();
    type Properties = ExampleProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Example { props, link }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
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
                <NumericInput<i32>
                    disabled=self.props.disabled
                    fill=self.props.fill
                    large=self.props.large
                    value=self.props.integer_value
                    min_value=self.props.integer_min_value
                    max_value=self.props.integer_max_value
                    increment=self.props.integer_increment
                    placeholder=String::from("Enter an integer...")
                />
                <NumericInput<f32>
                    disabled=self.props.disabled
                    fill=self.props.fill
                    large=self.props.large
                    value=self.props.float_value
                    min_value=self.props.float_min_value
                    max_value=self.props.float_max_value
                    increment=self.props.float_increment
                    placeholder=String::from("Enter a floating point number...")
                />
            </>
        }
    }
}
