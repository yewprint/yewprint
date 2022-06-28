use yew::prelude::*;
use yewprint::{Button, ButtonGroup, IconName};

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
            <Button icon={IconName::Database}> {"Queries"}</Button>
            <Button icon={IconName::Function}>{"Functions"}</Button>
            <Button icon={IconName::Cog}>{"Options"}</Button>
        </ButtonGroup>
    }
}
