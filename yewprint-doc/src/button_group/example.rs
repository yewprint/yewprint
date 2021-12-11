use std::borrow::Cow;
use yew::prelude::*;
use yewprint::{Button, ButtonGroup, IconName};

pub struct Example {
    props: ExampleProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ExampleProps {
    pub minimal: bool,
    pub fill: bool,
    pub large: bool,
    pub vertical: bool,
}

impl Component for Example {
    type Message = ();
    type Properties = ExampleProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Example { props }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self) -> Html {
        html! {
            <ButtonGroup
                minimal={self.props.minimal}
                fill={self.props.fill}
                large={self.props.large}
                vertical={self.props.vertical}
                style={Cow::Borrowed("margin:0;")}
            >
                <Button icon={IconName::Database>{"Queries"}</Button>}
                <Button icon={IconName::Function>{"Functions"}</Button>}
                <Button icon={IconName::Cog>{"Options"}</Button>}
            </ButtonGroup>
        }
    }
}
