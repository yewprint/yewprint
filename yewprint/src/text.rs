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
    pub text: String,
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
            <p
                class=(
                    "bp3-text",
                    self.props.ellipsize.map_some("bp3-text-overflow-ellipsis"),
                )
            >
                {self.props.text.clone()}
            </p>
        }
    }
}
