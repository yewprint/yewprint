use crate::ConditionalClass;
use yew::prelude::*;

pub struct Switch {
    props: SwitchProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct SwitchProps {
    #[prop_or_default]
    pub checked: bool,
    #[prop_or_default]
    pub disabled: ConditionalClass,
    #[prop_or_default]
    pub inline: ConditionalClass,
    #[prop_or_default]
    pub large: ConditionalClass,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub label: String,
}

impl Component for Switch {
    type Message = ();
    type Properties = SwitchProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
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
            <label class=(
                "bp3-control bp3-switch",
                self.props.disabled.map_some("bp3-disabled"),
                self.props.inline.map_some("bp3-inline"),
                self.props.large.map_some("bp3-large"),
                )
            >
            <input
                type="checkbox"
                checked={self.props.checked},
                onclick={self.props.onclick.clone()}
            />
            <span
                class="bp3-control-indicator"
            >
            </span>
            {self.props.label.clone()}
            </label>
        }
    }
}
