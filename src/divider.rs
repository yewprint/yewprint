use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct DividerProps {
    #[prop_or_default]
    pub vertical: bool,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(Divider)]
pub fn view(DividerProps { vertical, class }: &DividerProps) -> Html {
    html! {
        <span
            class={classes!(
                "bp3-divider",
                vertical.then_some("bp3-vertical"),
                class.clone(),
            )}
        />
    }
}
