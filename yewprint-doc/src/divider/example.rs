use yew::prelude::*;
use yewprint::{Button, ButtonGroup, Divider};

#[derive(Clone, PartialEq, Eq, Properties)]
pub struct ExampleProps {
    pub vertical: bool,
}
#[function_component(Example)]
pub fn example(props: &ExampleProps) -> Html {
    html! {
        <ButtonGroup vertical={props.vertical}>
            <Button>{"File"}</Button>
            <Button>{"Edit"}</Button>
            <Divider vertical={props.vertical} />
            <Button>{"Create"}</Button>
            <Button>{"Delete"}</Button>
            <Divider vertical={props.vertical} />
            // <Button icon=IconName::Add />
            // <Button icon=IconName::Remove />
        </ButtonGroup>
    }
}
