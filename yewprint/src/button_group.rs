use std::borrow::Cow;
use yew::prelude::*;

pub struct ButtonGroup {
    props: ButtonGroupProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ButtonGroupProps {
    #[prop_or_default]
    pub minimal: bool,
    #[prop_or_default]
    pub vertical: bool,
    #[prop_or_default]
    pub fill: bool,
    #[prop_or_default]
    pub large: bool,
    #[prop_or_default]
    pub style: Option<Cow<'static, str>>,
    #[prop_or_default]
    pub children: html::Children,
    #[prop_or_default]
    pub class: Classes,
}

impl Component for ButtonGroup {
    type Message = ();
    type Properties = ButtonGroupProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            props: *ctx.props(),
        }
    }

    fn update(&mut self, _: &Context<Self>, _: Self::Message) -> bool {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div
                class={classes!(
                    "bp3-button-group",
                    self.props.minimal.then(|| "bp3-minimal"),
                    self.props.fill.then(|| "bp3-fill"),
                    self.props.large.then(|| "bp3-large"),
                    self.props.vertical.then(|| "bp3-vertical"),
                    self.props.class.clone(),
                )}
                style={self.props.style.clone()}
            >
                {self.props.children.clone()}
            </div>
        }
    }
}
