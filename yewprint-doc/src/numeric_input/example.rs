use yew::prelude::*;
use yewprint::NumericInput;

pub struct Example {
    link: ComponentLink<Self>,
    props: ExampleProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ExampleProps {
    pub fill: bool,
    pub disabled: bool,
    pub large: bool,
}

impl Component for Example {
    type Message = ();
    type Properties = ExampleProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Example {
            link,
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
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
            <div>
                <NumericInput />
            </div>
        }
    }
}