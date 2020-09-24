use yew::prelude::*;
use yew::Classes;
use crate::elevation::Elevation;

#[derive(Clone, PartialEq, Properties)]
pub struct CardProps {
    #[prop_or_default]
    pub class: Option<String>,
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
        let mut class = Classes::from("bp3-card")
            .extend(&self.props.class);
        class.push(self.props.elevation.as_css_class());
        if self.props.interactive {
            class.push("bp3-interactive");
        }

        html! {
            <div class=class onclick={self.props.onclick.clone()}>
                {self.props.children.clone()}
            </div>
        }
    }
}

#[cfg(feature = "doc")]
pub mod doc {
    use super::*;

    pub struct CardDoc {}

    impl Component for CardDoc {
        type Message = ();
        type Properties = ();

        fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
            CardDoc {}
        }

        fn update(&mut self, _msg: Self::Message) -> ShouldRender {
            true
        }

        fn change(&mut self, _props: Self::Properties) -> ShouldRender {
            true
        }

        fn view(&self) -> Html {
            let source = crate::include_example!("example.rs");

            html! {
                <div>
                    <h1>{"Card"}</h1>
                    <div>{source}</div>
                </div>
            }
        }
    }
}
