use boolinator::Boolinator;
use yew::prelude::*;

pub struct Radio {
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
}

impl Component for Radio {
    type Message = ();
    type Properties = Props;
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
            <label
                class=(
                    "bp3-control bp3-radio",
                    self.props.disabled.as_some("bp3-disabled"),
                    self.props.inline.as_some("bp3-inline"),
                    self.props.large.as_some("bp3-large"),
                )
            >
                <input
                    type="radio"
                    checked={self.props.checked}
                    onchange={self.props.onchange.clone()}
                    disabled=self.props.disabled
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
