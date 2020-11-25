use yew::prelude::*;
use yewprint::{IconName, InputGroup};

pub struct Example {
    props: ExampleProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ExampleProps {
    pub disabled: bool,
    pub fill: bool,
    pub large: bool,
    pub small: bool,
    pub round: bool,
    pub placeholder: String,
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
        html! {
            <>
                <InputGroup
                    fill=self.props.fill
                    large=self.props.large
                    small=self.props.small
                    round=self.props.round
                    left_icon=IconName::Filter
                >
                    <input
                        class="bp3-input"
                        disabled=self.props.disabled
                        placeholder={"Filter histogram..."}
                    />
                </InputGroup>
                <InputGroup
                    fill=self.props.fill
                    large=self.props.large
                    small=self.props.small
                    round=self.props.round
                >
                    <input
                        class="bp3-input"
                        disabled=self.props.disabled
                        placeholder={"Enter your password..."}
                    />
                </InputGroup>
                <InputGroup
                    fill=self.props.fill
                    large=self.props.large
                    small=self.props.small
                    round=self.props.round
                    left_icon=IconName::Tag
                >
                    <input
                        class="bp3-input"
                        disabled=self.props.disabled
                        placeholder={"Find tags"}
                    />
                </InputGroup>
                <InputGroup
                    fill=self.props.fill
                    large=self.props.large
                    small=self.props.small
                    round=self.props.round
                >
                    <input
                        class="bp3-input"
                        disabled=self.props.disabled
                        placeholder={"Add people or groups..."}
                    />
                </InputGroup>
            </>
        }
    }
}
