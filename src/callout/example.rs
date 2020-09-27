use crate::{Callout, Intent, Menu, MenuItem, Switch};
use yew::prelude::*;

pub struct Example {
    link: ComponentLink<Self>,
    intent: Option<Intent>,
    show_icon: bool,
    show_title: bool,
}

pub enum Msg {
    ChangeIntent(Option<Intent>),
    ToggleIcon,
    ToggleTitle,
}

impl Component for Example {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Example {
            link,
            intent: None,
            show_icon: false,
            show_title: true,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ChangeIntent(new_intent) => self.intent = new_intent,
            Msg::ToggleIcon => self.show_icon = !self.show_icon,
            Msg::ToggleTitle => self.show_title = !self.show_title,
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let title = if self.show_title {
            Some("Visually important content")
        } else {
            None
        };
        html! {
            <Callout title=title without_icon=self.show_icon intent=self.intent>
                <p> {"The Callout element's background reflects its intent, if any."}</p>
                <div>
                    <Switch onclick=self.link.callback(|_| Msg::ToggleIcon) checked=self.show_icon label={ "Show/hide icon" } />
                    <Switch onclick=self.link.callback(|_| Msg::ToggleTitle) checked=self.show_title label={ "Show/hide title" } />
                    <p> {"Select intent:"}</p>
                    <Menu>
                        <MenuItem onclick=self.link.callback(|_| Msg::ChangeIntent(None)) text=html!{"None"}/>
                        <MenuItem onclick=self.link.callback(|_| Msg::ChangeIntent(Some(Intent::Primary))) text=html!{"Primary"}/>
                        <MenuItem onclick=self.link.callback(|_| Msg::ChangeIntent(Some(Intent::Success))) text=html!{"Success"}/>
                        <MenuItem onclick=self.link.callback(|_| Msg::ChangeIntent(Some(Intent::Warning))) text=html!{"Warning"}/>
                        <MenuItem onclick=self.link.callback(|_| Msg::ChangeIntent(Some(Intent::Danger))) text=html!{"Danger"}/>
                    </Menu>
                </div>
            </Callout>
        }
    }
}
