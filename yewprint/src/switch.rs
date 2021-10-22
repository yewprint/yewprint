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
    #[prop_or_default]
    pub innerLabelChecked: Option<String>,
    #[prop_or_default]
    pub innerLabel: Option<String>,
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
        let maybe_display_label = move || -> Html {
            if self.props.innerLabel.is_some() || self.props.innerLabelChecked.is_some() {
                html! {
                    <>
                        <div class=classes!("bp3-control-indicator-child")>
                            <div class=classes!("bp3-switch-inner-text")>
                                {
                                    if let Some(label_checked) = self.props.innerLabelChecked.as_ref() {
                                        label_checked.clone()
                                    } else {
                                        self.props.innerLabel.clone().unwrap_or_default()
                                    }
                                }
                            </div>
                        </div>
                        <div class=classes!("bp3-control-indicator-child")>
                            <div class=classes!("bp3-switch-inner-text")>{self.props.innerLabel.clone().unwrap_or_default()}</div>
                        </div>
                    </>
                }
            } else {
                html! {}
            }
        };


        html! {
            <label
                class=classes!(
                    "bp3-control",
                    "bp3-switch",
                    self.props.disabled.then(|| "bp3-disabled"),
                    self.props.inline.then(|| "bp3-inline"),
                    self.props.large.then(|| "bp3-large"),
                    self.props.class.clone(),
                )
            >
            <input
                type="checkbox"
                checked={self.props.checked}
                onclick={self.props.onclick.clone()}
                disabled=self.props.disabled
            />
            <span
                class=classes!("bp3-control-indicator")
            >
                {maybe_display_label()}
            </span>
            {self.props.label.clone()}
            </label>
        }
    }
}
