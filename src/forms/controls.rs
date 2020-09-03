use yew::prelude::*;

pub struct Switch {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub checked: bool,
    pub onclick: Callback<MouseEvent>,
    pub label: String,
}

impl Component for Switch {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Switch { props }
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
            <label class="bp3-control bp3-switch">
            <input
                type="checkbox"
                checked={self.props.checked}
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
