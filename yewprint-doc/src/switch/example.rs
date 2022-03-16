use yew::prelude::*;
use yewprint::{Label, Switch};

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
            <Label>{"Privacy settings"}</Label>
            <Switch
                disabled={props.disabled}
                inline={props.inline}
                large={props.large}
                label={html!("Enabled")}
            />
            <Switch
                disabled={props.disabled}
                inline={props.inline}
                large={props.large}
                label={html!(<em>{"Public"}</em>)}
            />
            <Switch
                disabled={props.disabled}
                inline={props.inline}
                large={props.large}
                label={html!(<strong>{"Cooperative"}</strong>)}
            />
            <Switch
                disabled={props.disabled}
                inline={props.inline}
                large={props.large}
                label={html!(<u>{"Containing Text"}</u>)}
            />
        </div>
    }
}
