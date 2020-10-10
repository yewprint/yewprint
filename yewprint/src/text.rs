use crate::ConditionalClass;
use yew::prelude::*;

pub struct Text {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub ellipsize: ConditionalClass,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: String,
    /// Wrap text in `span` instead of `div`.
    #[prop_or("div".into())]
    pub tag: String,
    #[prop_or_default]
    pub title: Option<String>,
}

impl Component for Text {
    type Message = ();
    type Properties = Props;

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
            <@{self.props.tag.clone()}
                class=(
                    self.props.class.clone(),
                    self.props.ellipsize.map_some("bp3-text-overflow-ellipsis"),
                )
                title?=self.props.title.clone()
            >
                {self.props.children.clone()}
            </@>
        }
    }
}
