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
    pub children: html::Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(ControlGroup)]
pub fn control_group(props: &ControlGroupProps) -> Html {
    html! {
        <div
            class={classes!(
                "bp3-control-group",
                props.fill.then_some("bp3-fill"),
                props.vertical.then_some("bp3-vertical"),
                props.class.clone(),
            )}
        >
            {props.children.clone()}
        </div>
    }
}
