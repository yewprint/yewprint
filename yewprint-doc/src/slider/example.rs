use yew::prelude::*;
use yewprint::Slider;

pub struct Example {
    props: ExampleProps,
    value: f64,
    link: ComponentLink<Self>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ExampleProps {
    pub vertical: bool,
}

pub enum Msg {
    ValueUpdate(f64),
}

impl Component for Example {
    type Message = Msg;
    type Properties = ExampleProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Example {
            props,
            value: Default::default(),
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
            <Slider<f64>
                min=-10.0
                max=10.0
                step_size=1.0
                label_step_size=2.0
                value=self.value
                onchange=self.link.callback(|x| Msg::ValueUpdate(x))
                vertical=self.props.vertical
            />
        }
    }
}
