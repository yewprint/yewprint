use std::borrow::Cow;
use yew::prelude::*;
use yewprint::{Callout, Intent};

pub struct Example {
    props: ExampleProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ExampleProps {
    pub intent: Option<Intent>,
    pub show_icon: bool,
    pub show_title: bool,
}

impl Component for Example {
    type Message = ();
    type Properties = ExampleProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Example { props }
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
        html! {
            <Callout
                title={self.props.show_title.then(|| Cow::Borrowed("Visually important content"))}
                without_icon={!self.props.show_icon}
                intent={self.props.intent}
            >
                <p>{"The Callout element's background reflects its intent, if any."}</p>
            </Callout>
        }
    }
}
