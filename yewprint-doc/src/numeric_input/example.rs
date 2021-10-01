use yew::prelude::*;
use yewprint::NumericInput;

pub struct Example {
    props: ExampleProps,
}

pub enum Msg {}

#[derive(Clone, PartialEq, Properties)]
pub struct ExampleProps {
    pub fill: bool,
    pub disabled: bool,
    pub large: bool,
}

impl Component for Example {
    type Message = Msg;
    type Properties = ExampleProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Example { props }
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
            <NumericInput<i32>
                disabled=self.props.disabled
                fill=self.props.fill
                large=self.props.large
                min_value=0
                max_value=10
                increment=1
            />
        }
    }
}
