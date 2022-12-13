use yew::prelude::*;
use yew::virtual_dom::AttrValue;

#[derive(Clone, PartialEq, Properties)]
pub struct TextProps {
    #[prop_or_default]
    pub ellipsize: bool,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    /// Wrap text in `span` instead of `div`.
    #[prop_or_default]
    pub inline: bool,
    #[prop_or_default]
    pub title: Option<AttrValue>,
    #[prop_or_default]
    pub style: Option<AttrValue>,
}

#[function_component(Text)]
pub fn text(
    TextProps {
        ellipsize,
        children,
        class,
        inline,
        title,
        style,
    }: &TextProps,
) -> Html {
    html! {
        <@{if *inline { "span" } else { "div"}}
            class={classes!(
                ellipsize.then_some("bp3-text-overflow-ellipsis"),
                class.clone(),
            )}
            {style}
            {title}
        >
            {children.clone()}
        </@>
    }
}
