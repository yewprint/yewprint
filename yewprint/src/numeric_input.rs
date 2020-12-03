use crate::{Button, ControlGroup, InputGroup, Intent};
use boolinator::Boolinator;
use yew::prelude::*;

pub struct NumericInput {
    props: NumericInputProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct NumericInputProps {
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub fill: bool,
    #[prop_or_default]
    pub large: bool,
    #[prop_or_default]
    pub min_value: i32,
    #[prop_or_default]
    pub max_value: i32,
    #[prop_or_default]
    pub intent: Option<Intent>,
}

impl Component for NumericInput {
    type Message = ();
    type Properties = NumericInputProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
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
            <ControlGroup >
                <InputGroup />
                <Button />
                <Button />
            </ControlGroup>
        }
    }
}
