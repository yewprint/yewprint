use crate::Intent;
use yew::prelude::*;

pub struct TextArea {
    props: TextAreaProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct TextAreaProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub fill: bool,
    #[prop_or_default]
    pub grow_vertically: bool,
    #[prop_or_default]
    pub input_ref: NodeRef,
    #[prop_or_default]
    pub intent: Option<Intent>,
    #[prop_or_default]
    pub large: bool,
    #[prop_or_default]
    pub small: bool,
    #[prop_or_default]
    pub onchange: Option<Callback<ChangeData>>,
}

impl Component for TextArea {
    type Message = ();
    type Properties = TextAreaProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        TextArea { props }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self) -> Html {
        let classes = classes!(
            "bp3-input",
            self.props.intent,
            self.props.class.clone(),
            self.props.fill.then(|| "bp3-fill"),
            self.props.small.then(|| "bp3-small"),
            self.props.large.then(|| "bp3-large"),
        );
        html! {
            <textarea
                class={classes}
                onchange={self.props.onchange.clone()}
            />
        }
    }
}
