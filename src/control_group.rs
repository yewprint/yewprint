use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct ControlGroupProps {
    #[prop_or_default]
    pub fill: bool,
    #[prop_or_default]
    pub vertical: bool,
    #[prop_or_default]
    pub large: bool,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(ControlGroup)]
pub fn control_group(
    ControlGroupProps {
        fill,
        vertical,
        large,
        class,
        children,
    }: &ControlGroupProps,
) -> Html {
    html! {
        <div
            class={classes!(
                "bp3-control-group",
                fill.then_some("bp3-fill"),
                vertical.then_some("bp3-vertical"),
                large.then_some("bp3-large"),
                class.clone(),
            )}
        >
            {children.clone()}
        </div>
    }
}
