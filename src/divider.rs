use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct DividerProps {
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(Divider)]
pub fn view(DividerProps { class }: &DividerProps) -> Html {
    html! {
        <div
            class={classes!(
                "bp3-divider",
                class.clone(),
            )}
        />
    }
}
