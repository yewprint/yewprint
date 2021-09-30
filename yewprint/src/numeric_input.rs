use crate::{Button, ButtonGroup, ControlGroup, IconName, InputGroup, Intent};
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
    pub class: String,
    #[prop_or_default]
    pub left_icon: Option<IconName>,
    #[prop_or_default]
    pub min_value: Option<i32>,
    #[prop_or_default]
    pub max_value: Option<i32>,
    #[prop_or_default]
    pub value: Option<i32>,
    #[prop_or_default]
    pub intent: Option<Intent>,
    #[prop_or_default]
    pub oninput: Callback<InputData>,
    #[prop_or_default]
    pub onclick_up: Callback<MouseEvent>,
    #[prop_or_default]
    pub onclick_down: Callback<MouseEvent>,
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
            <ControlGroup
                class=classes!("bp3-numeric-input")
                fill=self.props.fill
                large=self.props.large
            >
                <InputGroup
                    placeholder="Enter a number..."
                    large=self.props.large
                    disabled=self.props.disabled
                    oninput=self.props.oninput.clone()
                />
                <ButtonGroup vertical=true class=classes!("bp3-fixed")>
                    <Button
                        icon=IconName::ChevronUp
                        disabled=self.props.disabled
                        onclick=self.props.onclick_up.clone()
                    />
                    <Button
                        icon=IconName::ChevronDown
                        disabled=self.props.disabled
                        onclick=self.props.onclick_down.clone()
                    />
                </ButtonGroup>
            </ControlGroup>
        }
    }
}
