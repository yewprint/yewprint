use yew::prelude::*;
use yew::virtual_dom::AttrValue;

#[derive(Clone, PartialEq, Properties)]
pub struct ButtonGroupProps {
    #[prop_or_default]
    pub minimal: bool,
    #[prop_or_default]
    pub vertical: bool,
    #[prop_or_default]
    pub fill: bool,
    #[prop_or_default]
    pub large: bool,
    #[prop_or_default]
    pub style: Option<AttrValue>,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(ButtonGroup)]
pub fn button_group(props: &ButtonGroupProps) -> Html {
    let ButtonGroupProps {
        minimal,
        vertical,
        fill,
        large,
        style,
        children,
        class,
    } = props;

    html! {
        <div
            class={classes!(
                "bp3-button-group",
                minimal.then_some("bp3-minimal"),
                fill.then_some("bp3-fill"),
                large.then_some("bp3-large"),
                vertical.then_some("bp3-vertical"),
                class.clone(),
            )}
            {style}
        >
            {children.clone()}
        </div>
    }
}
