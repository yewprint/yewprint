use crate::Elevation;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct CardProps {
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub elevation: Option<Elevation>,
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
        let mut classes = Classes::from("bp3-card")
            .extend(&self.props.class)
            .extend(self.props.elevation);
        if self.props.interactive {
            classes.push("bp3-interactive");
        }

        html! {
            <div class=classes onclick={self.props.onclick.clone()}>
                {self.props.children.clone()}
            </div>
        }
    }
}
