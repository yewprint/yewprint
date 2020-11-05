use yew::prelude::*;
use yewprint::ControlGroup;

pub struct Example {
    props: ExampleProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ExampleProps {
    pub fill: bool,
    pub vertical: bool,
}

impl Component for Example {
    type Message = ();
    type Properties = ExampleProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Example { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShoudRender {
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
            <ControlGroup
                fill=self.props.fill
                vertical=self.props.fill
            >
            </ControlGroup>
        }
    }
}