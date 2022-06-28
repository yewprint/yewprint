use yew::prelude::*;
use yewprint::{Checkbox, Label};

#[derive(Clone, PartialEq, Properties)]
pub struct ExampleProps {
    pub disabled: bool,
    pub inline: bool,
    pub large: bool,
}

#[function_component(Example)]
pub fn example(props: &ExampleProps) -> Html {
    html! {
        <div>
            <Label>{"Assign responsability"}</Label>
            <Checkbox
                disabled={props.disabled}
                inline={props.inline}
                large={props.large}
                label={html!("Gilad Gray")}
            />
            <Checkbox
                disabled={props.disabled}
                inline={props.inline}
                large={props.large}
                label={html!("Jason Killian")}
            />
            <Checkbox
                disabled={props.disabled}
                inline={props.inline}
                large={props.large}
                label={html!("Antoine Llorca")}
            />
        </div>
    }
}
