use yew::prelude::*;
use yewprint::{Button, ButtonGroup, Divider};

pub struct Example;

impl Component for Example {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <ButtonGroup>
                <Button>{"File"}</Button>
                <Button>{"Edit"}</Button>
                <Divider />
                <Button>{"Create"}</Button>
                <Button>{"Delete"}</Button>
                <Divider />
                // <Button icon=IconName::Add />
                // <Button icon=IconName::Remove />
            </ButtonGroup>
        }
    }
}
