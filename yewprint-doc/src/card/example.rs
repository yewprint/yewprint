use yew::prelude::*;
use yewprint::{Card, Elevation};

#[derive(Clone, PartialEq, Eq, Properties)]
pub struct ExampleProps {
    pub elevation: Elevation,
    pub interactive: bool,
}

#[function_component(Example)]
pub fn example(props: &ExampleProps) -> Html {
    html! {
        <Card elevation={props.elevation} interactive={props.interactive}>
            <p>
                {
                    "This is a card component. The elevation of the card can be adjusted. \
                    An interactive card reacts to being moused over."
                }
            </p>
        </Card>
    }
}
