use yew::prelude::*;
use yewprint::{Card, Elevation};

pub struct CardDoc {
    link: ComponentLink<Self>,
    elevation: Elevation,
}

pub enum Msg {
    IncreaseElevation,
}

impl Component for CardDoc {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        CardDoc { link, elevation: Elevation::Level0 }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::IncreaseElevation => self.elevation = Elevation::from_value_clamped(self.elevation as u8 + 1),
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
                <Card elevation=Elevation::Level0 onclick=self.link.callback(|_| Msg::IncreaseElevation)>
                    <h1>{"Card"}</h1>
                    <p>{format!(
                        "This is a card component with elevation {}. Click the card to increase the elevation.",
                        self.elevation as u8)}
                   </p>
                </Card>
            }
    }
}