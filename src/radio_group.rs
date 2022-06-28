use crate::Radio;
use yew::prelude::*;

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

// impl<T: Clone + PartialEq + 'static> Component for RadioGroup<T> {

#[function_component(RadioGroup)]
pub fn radio_group<T: Clone + PartialEq + 'static>(props: &RadioGroupProps<T>) -> Html {
    let option_children = props
        .options
        .iter()
        .map(|(value, label)| {
            let checked = props.value.as_ref().map(|x| value == x).unwrap_or_default();
            let value = value.clone();

            html! {
                <Radio
                    value={"".to_string()}
                    label={html!(label)}
                    checked={checked}
                    onchange={props.onchange.reform(move |_| value.clone())}
                    inline={props.inline}
                    disabled={props.disabled}
                    large={props.large}
                />
            }
        })
        .collect::<Html>();

    html! {
        <div
            class={classes!(
                "bp3-radio-group",
                props.class.clone(),
            )}
        >
            {
                if let Some(label) = props.label.clone() {
                    label
                } else {
                    html!()
                }
            }
            {option_children}
        </div>
    }
}
