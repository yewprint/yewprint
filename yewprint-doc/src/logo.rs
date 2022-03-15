use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct LogoProps {
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(Logo)]
pub fn logo(props: &LogoProps) -> Html {
    crate::include_raw_html!("logo.svg", props.class.to_string())
}
