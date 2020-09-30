use yew::prelude::*;
use yewprint::{Button, ButtonGroup, IconName};

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
            <ButtonGroup is_minimal=true>
                <Button icon=IconName::Database>{"Queries"}</Button>
                <Button icon=IconName::Function>{"Functions"}</Button>
                <Button icon=IconName::Cog>{"Options"}</Button>
            </ButtonGroup>
        }
    }
}
