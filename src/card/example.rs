use yew::prelude::*;
use yewprint::{Card, Elevation};

pub struct Example {
    link: ComponentLink<Self>,
    elevation: Elevation,
}

pub enum Msg {
    IncreaseElevation,
}

impl Component for Example {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Example { link, elevation: Elevation::Level0 }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::IncreaseElevation => self.elevation = self.elevation.above(),
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
                <Card elevation={self.elevation} onclick=self.link.callback(|_| Msg::IncreaseElevation)
                        interactive=true>
                    <p>{format!(
                        "This is a card component with elevation {}. Click the card to increase the elevation.",
                        self.elevation as u8)}
                   </p>
                </Card>
            }
    }
}