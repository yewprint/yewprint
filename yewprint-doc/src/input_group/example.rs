use yew::prelude::*;
use yewprint::{Button, IconName, InputGroup};

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
                    disabled=self.props.disabled
                    left_icon=IconName::Filter
                    placeholder={"Filter histogram..."}
                />
                <InputGroup
                    fill=self.props.fill
                    large=self.props.large
                    small=self.props.small
                    round=self.props.round
                    disabled=self.props.disabled
                    placeholder={"Enter your password..."}
                    right_element=html!(
                        <Button
                            icon=IconName::Lock
                            minimal=true
                            disabled=self.props.disabled
                        />
                    )
                />
                <InputGroup
                    fill=self.props.fill
                    large=self.props.large
                    small=self.props.small
                    round=self.props.round
                    disabled=self.props.disabled
                    left_icon=IconName::Tag
                    placeholder={"Find tags"}
                />
                <InputGroup
                    fill=self.props.fill
                    large=self.props.large
                    small=self.props.small
                    round=self.props.round
                    disabled=self.props.disabled
                    placeholder={"Add people or groups..."}
                />
            </>
        }
    }
}
