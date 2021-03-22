use yew::prelude::*;
use yewprint::Slider;

pub struct Example {
    props: ExampleProps,
    value: i32,
    link: ComponentLink<Self>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ExampleProps {
    pub vertical: bool,
}

pub enum Msg {
    ValueUpdate(i32),
}

impl Component for Example {
    type Message = Msg;
    type Properties = ExampleProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Example {
            props,
            value: 15,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ValueUpdate(value) => {
                self.value = value;
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
            <Slider
                min={0}
                max={10}
                step_size={1}
                value=self.value
                onchange=self.link.callback(|x| Msg::ValueUpdate(x))
                vertical=self.props.vertical
            />
        }
    }
}
