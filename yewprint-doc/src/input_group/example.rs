use yew::prelude::*;
use yewprint::{IconName, InputGroup};

pub struct Example {
    props: ExampleProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ExampleProps {
    pub disabled: bool,
    pub large: bool,
    pub small: bool,
    pub show_placeholder: bool,
}

impl Component for Example {
    type Message = ();
    type Properties = ExampleProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Example { props }
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
        let placeholder = if self.props.show_placeholder {
            Some("Test")
        } else {
            None
        };

        html! {
            <InputGroup
                disabled=self.props.disabled
                large=self.props.large
                small=self.props.small
                left_icon=IconName::Cog
            >
            </InputGroup>
        }
    }
}
