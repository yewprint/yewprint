use yew::prelude::*;
use yewprint::{Button, ButtonGroup, Divider, Icon};

#[derive(Clone, PartialEq, Properties)]
pub struct ExampleProps {
    pub vertical: bool,
}
#[function_component(Example)]
pub fn example(ExampleProps { vertical }: &ExampleProps) -> Html {
    html! {
        <ButtonGroup minimal=true {vertical} >
            <Button>{"File"}</Button>
            <Button>{"Edit"}</Button>
            <Divider />
            <Button>{"Create"}</Button>
            <Button>{"Delete"}</Button>
            <Divider />
            <Button icon={Icon::Add} />
            <Button icon={Icon::Remove} />
        </ButtonGroup>
    }
}
