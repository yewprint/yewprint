use std::fmt::Display;
use std::hash::Hash;
use yew::prelude::*;

pub struct Radio<T: Clone + PartialEq + Display + Default + Hash + 'static> {
    props: RadioProps<T>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct RadioProps<T: Clone + PartialEq + Display + Default + Hash + 'static> {
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub inline: bool,
    #[prop_or_default]
    pub large: bool,
    #[prop_or_default]
    pub checked: bool,
    #[prop_or_default]
    pub onchange: Callback<T>,
    #[prop_or_default]
    pub label: yew::virtual_dom::VNode,
    pub value: T,
}

impl<T: Clone + PartialEq + Display + Default + Hash + 'static> Component for Radio<T> {
    type Message = ();
    type Properties = RadioProps<T>;
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
                class=classes!(
                    "bp3-control",
                    "bp3-radio",
                    self.props.disabled.then(|| "bp3-disabled"),
                    self.props.inline.then(|| "bp3-inline"),
                    self.props.large.then(|| "bp3-large"),
                )
            >
                <input
                    type="radio"
                    checked={self.props.checked}
                    onchange={self.props.onchange.reform(move |_| self.props.value)}
                    disabled=self.props.disabled
                    value={self.props.value.clone()}
                />
                <span
                    class=classes!("bp3-control-indicator")
                >
                </span>
                {self.props.label.clone()}
            </label>
        }
    }
}
