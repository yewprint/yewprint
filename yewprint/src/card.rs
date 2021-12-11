use crate::Elevation;
use yew::prelude::*;
use yew::

#[derive(Clone, PartialEq, Properties)]
pub struct CardProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub elevation: Elevation,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or(false)]
    pub interactive: bool,
    pub children: html::Children,
}

pub struct Card {
    props: CardProps,
}

impl Component for Card {
    type Message = ();
    type Properties = CardProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        true
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <div class={classes!(
                "bp3-card",
                self.props.class.clone(),
                self.props.elevation,
                self.props.interactive.then(|| "bp3-interactive"),
            )}
            onclick={self.props.onclick.clone()}>
                {self.props.children.clone()}
            </div>
        }
    }
}
