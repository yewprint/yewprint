use yew::prelude::*;

pub struct Divider {
    props: DividerProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct DividerProps {
    #[prop_or_default]
    pub vertical: bool,
    #[prop_or_default]
    pub children: html::Children,
    #[prop_or_default]
    pub class: Classes,
}

impl Component for Divider {
    type Message = ();
    type Properties = DividerProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _: Self::Message) -> bool {
        true
    }

    fn view(&self) -> Html {
        html! {
            <span
                class={classes!(
                    "bp3-divider",
                    self.props.vertical.then(|| "bp3-vertical"),
                    self.props.class.clone(),
                )}
            />
        }
    }
}
