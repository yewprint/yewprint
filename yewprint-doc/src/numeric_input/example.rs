use yew::prelude::*;
use yewprint::{Button, IconName, NumericInput};

pub struct Example {
    props: ExampleProps,
    link: ComponentLink<Self>,
    value: i32,
    value_two: i32,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ExampleProps {
    pub fill: bool,
    pub disabled: bool,
    pub large: bool,
}

pub enum Msg {
    Reset,
}

impl Component for Example {
    type Message = Msg;
    type Properties = ExampleProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Example {
            props,
            link,
            value: 0,
            value_two: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Reset => {
                self.value = 4;
                self.value_two = 4;
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
            <NumericInput<i32>
                disabled=self.props.disabled
                fill=self.props.large
                value=self.value_two
                bounds=i32::MIN..=i32::MAX
                increment=10
                placeholder=String::from("Enter an integer...")
            />
            <NumericInput<i32>
                disabled=self.props.disabled
                fill=self.props.fill
                large=self.props.large
                value=self.value
                bounds=-10..=10
                increment=1
                placeholder=String::from("Integer between -10 and 10")
            />
            <Button
                icon=IconName::Refresh
                onclick=self.link.callback(|_| Msg::Reset)
            >
                {"Reset at 4"}
            </Button>
            </>
        }
    }
}
