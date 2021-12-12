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
    pub inner_label_checked: Option<String>,
    #[prop_or_default]
    pub inner_label: Option<String>,
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
            if self.props.inner_label.is_some() || self.props.inner_label_checked.is_some() {
                let inner_label = self.props.inner_label.as_ref().unwrap_or_default();
                let inner_label_checked = self.props.inner_label_checked.as_ref();
                html! {
                    <>
                        <div class=classes!("bp3-control-indicator-child")>
                            <div class=classes!("bp3-switch-inner-text")>
                                {
                                    if let Some(label_checked) = inner_label_checked {
                                        label_checked.clone()
                                    } else {
                                        inner_label.clone()
                                    }
                                }
                            </div>
                        </div>
                        <div class=classes!("bp3-control-indicator-child")>
                            <div class=classes!("bp3-switch-inner-text")>{inner_label.clone()}</div>
                        </div>
                    </>
                }
            } else {
                Html::default()
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
