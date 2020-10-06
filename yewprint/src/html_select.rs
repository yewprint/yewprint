use crate::{ConditionalClass, Icon, IconName}
use yew::prelude::*;

pub struct HtmlSelect {
    props: props,
}

pub struct Props {
    #[prop_or_default]
    pub fill: ConditionalClass,
    #[prop_or_default]
    pub minimal: ConditionalClass,
    #[prop_or_default]
    pub disable: ConditionalClass,
    #[prop_or_default]
    pub icon: Option<IconName>,
    #[prop_or_default]
    pub title: Option<String>,
    pub children: html::Children
    #[prop_or_default]
    // onChange
    //options
}

impl Component for HtmlSelect {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        HtmlSelect {props}
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

        }
    }
}