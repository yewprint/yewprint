use std::borrow::Cow;
use yew::prelude::*;

pub struct Text {
    props: TextProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct TextProps {
    #[prop_or_default]
    pub ellipsize: bool,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    /// Wrap text in `span` instead of `div`.
    #[prop_or_default]
    pub inline: bool,
    #[prop_or_default]
    pub title: Option<Cow<'static, str>>,
    #[prop_or_default]
    pub style: Option<Cow<'static, str>>,
}

impl Component for Text {
    type Message = ();
    type Properties = TextProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            props: *ctx.props(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <@{if self.props.inline { "span" } else { "div"}}
                class={classes!(
                    self.props.class.clone(),
                    self.props.ellipsize.then (|| "bp3-text-overflow-ellipsis"),
                )}
                style={self.props.style.clone()}
                title={self.props.title.clone()}
            >
                {self.props.children.clone()}
            </@>
        }
    }
}
