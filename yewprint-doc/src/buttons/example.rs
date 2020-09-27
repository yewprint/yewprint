use yew::prelude::*;
use yewprint::Button;

pub struct Example {
    link: ComponentLink<Self>,
    counter: i64,
}

pub enum Msg {
    AddOne,
}

impl Component for Example {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Example { counter: 0, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => self.counter += 1,
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <p> {"Counter: "} { self.counter }</p>
                <div>
                    <Button onclick=self.link.callback(|_| Msg::AddOne)>{ "Add 1" }</Button>
                </div>
            </div>
        }
    }
}
