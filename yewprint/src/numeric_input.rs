use crate::Intent;
use boolinator::Boolinator;
use yew::prelude::*;

pub struct NumericInput {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub fill: bool,
    #[prop_or_default]
    pub large: bool,
    #[prop_or_default]
    pub minValue: i32,
    #[prop_or_default]
    pub maxValue: i32,
    #[prop_or_default]
    pub intent: Option<Intent>,
    #[prop_or_default]
    pub onchange: Callback<T>
}

impl Component for NumericInput {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        NumericInput { props }
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
            <InputGroup
                class=(
                    "bp3-numeric-input",
                    self.props.disabled.as_some("bp3-disabled"),
                    self.props.fill.as_some("bp3-fill"),
                    self.props.large.as_some("bp3-large"),
                    self.props.intent,
                )
                onchange={self.props.onchange.clone()}
            />
        }
    }
}