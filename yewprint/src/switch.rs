use yew::prelude::*;

pub struct Switch {
    props: SwitchProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct SwitchProps {
    #[prop_or_default]
    pub checked: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub inline: bool,
    #[prop_or_default]
    pub large: bool,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub label: yew::virtual_dom::VNode,
    #[prop_or_default]
    pub class: Classes,
}

impl Component for Switch {
    type Message = ();
    type Properties = SwitchProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        true
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <label
                class={classes!(
                    "bp3-control",
                    "bp3-switch",
                    self.props.disabled.then(|| "bp3-disabled"),
                    self.props.inline.then(|| "bp3-inline"),
                    self.props.large.then(|| "bp3-large"),
                    self.props.class.clone(),
                )}
            >
            <input
                type="checkbox"
                checked={self.props.checked}
                onclick={self.props.onclick.clone()}
                disabled={self.props.disabled}
            />
            <span
                class={classes!("bp3-control-indicator")}
            >
            </span>
            {self.props.label.clone()}
            </label>
        }
    }
}
