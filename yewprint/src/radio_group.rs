use crate::Radio;
use yew::prelude::*;

pub struct RadioGroup<T: Clone + PartialEq + 'static> {
    props: RadioGroupProps<T>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct RadioGroupProps<T: Clone + PartialEq + 'static> {
    #[prop_or_default]
    pub label: Option<yew::virtual_dom::VNode>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub inline: bool,
    #[prop_or_default]
    pub large: bool,
    pub options: Vec<(T, String)>,
    #[prop_or_default]
    pub value: Option<T>,
    #[prop_or_default]
    pub onchange: Callback<T>,
    #[prop_or_default]
    pub class: Classes,
}

impl<T: Clone + PartialEq + 'static> Component for RadioGroup<T> {
    type Message = ();
    type Properties = RadioGroupProps<T>;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            props: *ctx.props(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let option_children = self
            .props
            .options
            .iter()
            .map(|(value, label)| {
                let checked = self
                    .props
                    .value
                    .as_ref()
                    .map(|x| value == x)
                    .unwrap_or_default();
                let value = value.clone();

                html! {
                    <Radio
                        value={"".to_string()}
                        label={html!(label)}
                        checked={checked}
                        onchange={self.props.onchange.reform(move |_| value.clone())}
                        inline={self.props.inline}
                        disabled={self.props.disabled}
                        large={self.props.large}
                    />
                }
            })
            .collect::<Html>();

        html! {
            <div
                class={classes!(
                    "bp3-radio-group",
                    self.props.class.clone(),
                )}
            >
                {
                    if let Some(label) = self.props.label.clone() {
                        label
                    } else {
                        html!()
                    }
                }
                {option_children}
            </div>
        }
    }
}
