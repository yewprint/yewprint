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
    pub title: Option<String>,
    #[prop_or_default]
    pub style: Option<String>,
}

impl Component for Text {
    type Message = ();
    type Properties = TextProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Text { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <@{if self.props.inline { "span" } else { "div"}}
                class=classes!(
                    self.props.class.clone(),
                    self.props.ellipsize.then (|| "bp3-text-overflow-ellipsis"),
                )
                style?=self.props.style.clone()
                title?=self.props.title.clone()
            >
                {self.props.children.clone()}
            </@>
        }
    }
}
