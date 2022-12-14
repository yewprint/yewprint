use crate::Radio;
use implicit_clone::{unsync::IArray, ImplicitClone};
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct RadioGroupProps<T: ImplicitClone + PartialEq + 'static> {
    #[prop_or_default]
    pub label: Option<Html>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub inline: bool,
    #[prop_or_default]
    pub large: bool,
    pub options: IArray<(T, AttrValue)>,
    #[prop_or_default]
    pub value: Option<T>,
    #[prop_or_default]
    pub onchange: Callback<T>,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(RadioGroup)]
pub fn radio_group<T: ImplicitClone + PartialEq + 'static>(
    RadioGroupProps {
        label,
        disabled,
        inline,
        large,
        options,
        value,
        onchange,
        class,
    }: &RadioGroupProps<T>,
) -> Html {
    let option_children = options
        .iter()
        .map(|(this_value, label)| {
            let checked = value.as_ref().map(|x| &this_value == x).unwrap_or_default();

            html! {
                <Radio
                    value={String::new()}
                    label={html!(label)}
                    {checked}
                    onchange={onchange.reform(move |_| this_value.clone())}
                    {inline}
                    {disabled}
                    {large}
                />
            }
        })
        .collect::<Html>();

    html! {
        <div
            class={classes!(
                "bp3-radio-group",
                class.clone(),
            )}
        >
            {label.clone()}
            {option_children}
        </div>
    }
}
