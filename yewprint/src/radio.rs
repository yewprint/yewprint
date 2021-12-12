use yew::prelude::*;

pub struct Radio {
    props: RadioProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct RadioProps {
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub inline: bool,
    #[prop_or_default]
    pub large: bool,
    #[prop_or_default]
    pub checked: Option<bool>,
    #[prop_or_default]
    pub name: Option<String>,
    #[prop_or_default]
    pub onchange: Option<Callback<Event>>,
    #[prop_or_default]
    pub label: yew::virtual_dom::VNode,
    #[prop_or_default]
    pub value: Option<String>,
}

impl Component for Radio {
    type Message = ();
    type Properties = RadioProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self { props: ctx.props() }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self) -> Html {
        html! {
            <label
                class={classes!(
                    "bp3-control",
                    "bp3-radio",
                    self.props.disabled.then(|| "bp3-disabled"),
                    self.props.inline.then(|| "bp3-inline"),
                    self.props.large.then(|| "bp3-large"),
                )}
            >
                <input
                    type="radio"
                    onchange={self.props.onchange.clone().unwrap_or_default()}
                    disabled={self.props.disabled}
                    value={self.props.value.clone().unwrap_or_default()}
                    checked={self.props.checked.unwrap_or(false)}
                    name={self.props.name.clone().unwrap_or_default()}
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
