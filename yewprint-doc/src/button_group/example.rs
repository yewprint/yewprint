use yew::prelude::*;
use yewprint::{Button, ButtonGroup, Icon};

#[derive(Clone, PartialEq, Properties)]
pub struct ExampleProps {
    pub minimal: bool,
    pub fill: bool,
    pub large: bool,
    pub vertical: bool,
}

#[function_component(Example)]
pub fn example(props: &ExampleProps) -> Html {
    html! {
        <ButtonGroup
            minimal={props.minimal}
            fill={props.fill}
            large={props.large}
            vertical={props.vertical}
            style={"margin:0;"}
        >
            <Button icon={Icon::Database}> {"Queries"}</Button>
            <Button icon={Icon::Function}>{"Functions"}</Button>
            <Button icon={Icon::Cog}>{"Options"}</Button>
        </ButtonGroup>
    }
}
