use yew::prelude::*;

pub struct Checkbox {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub inline: bool,
    #[prop_or_default]
    pub large: bool,
    #[prop_or_default]
    pub checked: bool,
    #[prop_or_default]
    pub onchange: Callback<ChangeData>,
    #[prop_or_default]
    pub label: yew::virtual_dom::VNode,
    #[prop_or_default]
    pub indeterminate_state: bool,
}

impl Component for Checkbox {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self) -> Html {
        html! {
            <label
                class={classes!(
                    "bp3-control", "bp3-checkbox",
                    self.props.disabled.then(|| "bp3-disabled"),
                    self.props.inline.then(|| "bp3-inline"),
                    self.props.large.then(|| "bp3-large")
                )}
            >
                <input
                    type="checkbox"
                    checked={self.props.checked}
                    onchange={self.props.onchange.clone()}
                    disabled={self.props.disabled}
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
