use yew::prelude::*;
use yewprint::NumericInput;

pub struct Example {
    props: ExampleProps,
    link: ComponentLink<Self>,
    value: String,
}

pub enum Msg {
    SetValue(String),
    Up,
    Down,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ExampleProps {
    pub fill: bool,
    pub disabled: bool,
    pub large: bool,
}

impl Component for Example {
    type Message = Msg;
    type Properties = ExampleProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Example {
            props,
            link,
            value: Default::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetValue(input) => {
                self.value = input;
            }
            Msg::Up => match self.value.trim().parse::<u32>() {
                Ok(value) => self.value = (value + 1).to_string(),
                Err(_) => (),
            },
            Msg::Down => match self.value.trim().parse::<u32>() {
                Ok(value) => self.value = (value - 1).to_string(),
                Err(_) => (),
            },
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
            <NumericInput
                disabled=self.props.disabled
                fill=self.props.fill
                large=self.props.large
                onclick_up=self.link.callback(|_| Msg::Up)
                onclick_down=self.link.callback(|_| Msg::Down)
                oninput=self.link.callback(|e: InputData| Msg::SetValue(e.value))
                max_value=10
                min_value=0
            />
        }
    }
}
