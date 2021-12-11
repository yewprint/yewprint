use crate::{Icon, IconName, Intent, Spinner, ICON_SIZE_LARGE};
use std::borrow::Cow;
use yew::prelude::*;

pub struct Button {
    props: ButtonProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ButtonProps {
    #[prop_or_default]
    pub fill: bool,
    #[prop_or_default]
    pub minimal: bool,
    #[prop_or_default]
    pub small: bool,
    #[prop_or_default]
    pub outlined: bool,
    #[prop_or_default]
    pub loading: bool,
    #[prop_or_default]
    pub large: bool,
    #[prop_or_default]
    pub active: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub icon: Option<IconName>,
    #[prop_or_default]
    pub intent: Option<Intent>,
    #[prop_or_default]
    pub title: String,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: Option<Cow<'static, str>>,
    #[prop_or_default]
    pub children: html::Children,
}

impl Component for Button {
    type Message = ();
    type Properties = ButtonProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Button { props }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self) -> Html {
        html! {
            <button
                class={classes!(
                    "bp3-button",
                    self.props.fill.then(|| "bp3-fill"),
                    self.props.minimal.then(|| "bp3-minimal"),
                    self.props.small.then(|| "bp3-small"),
                    self.props.outlined.then(|| "bp3-outlined"),
                    self.props.loading.then(|| "bp3-loading"),
                    self.props.large.then(|| "bp3-large"),
                    (self.props.active && !self.props.disabled).then(|| "bp3-active"),
                    self.props.disabled.then(|| "bp3-disabled"),
                    self.props.intent,
                    self.props.class.clone(),
                )}
                style={self.props.style.clone()}
                onclick={(!self.props.disabled).then(|| self.props.onclick.clone())}
            >
                {
                    self
                        .props
                        .loading
                        .then(|| html! {
                            <Spinner
                                class={classes!("bp3-button-spinner")}
                                size={ICON_SIZE_LARGE as f32}
                            />
                        })
                        .unwrap_or_default()
                }
                {
                    if let Some(icon) = self.props.icon {
                        html! {
                            <Icon icon={icon} />
                        }
                    } else {
                        html!()
                    }
                }
                {
                    if self.props.children.is_empty() {
                        html! ()
                    } else {
                        html! {
                            <span class="bp3-button-text">
                                {for self.props.children.iter()}
                            </span>
                        }
                    }
                }
            </button>
        }
    }
}
