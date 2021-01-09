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
    pub intent: Option<Intent>,
    #[prop_or_default]
    pub onchange: Callback<i32>,
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
                class="bp3-numeric-input"
                fill=self.props.fill
                large=self.props.large
            >
                <InputGroup
                    placeholder="Enter a number..."
                    large=self.props.large
                    disabled=self.props.disabled
                />
                <ButtonGroup vertical=true class="bp3-fixed">
                    <Button
                        icon=IconName::ChevronUp
                        disabled=self.props.disabled
                    />
                    <Button
                        icon=IconName::ChevronDown
                        disabled=self.props.disabled
                    />
                </ButtonGroup>
            </ControlGroup>
        }
    }
}
