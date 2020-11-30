use yew::prelude::*;
use yewprint::{Button, IconName, InputGroup, Tag};

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
            <div style="
                display: flex;
                flex-wrap: wrap;
            ">
                <div style="
                    width: 50%;
                    padding: 10px;
                ">
                    <InputGroup
                        fill=self.props.fill
                        large=self.props.large
                        small=self.props.small
                        round=self.props.round
                        disabled=self.props.disabled
                        left_icon=IconName::Filter
                        placeholder={"Filter histogram..."}
                    />
                </div>
                <div style="
                    width: 50%;
                    padding: 10px;
                ">
                    <InputGroup
                        fill=self.props.fill
                        large=self.props.large
                        small=self.props.small
                        round=self.props.round
                        disabled=self.props.disabled
                        placeholder={"Enter your password..."}
                        right_element=html! {
                            <Button
                                icon=IconName::Lock
                                minimal=true
                                disabled=self.props.disabled
                            />
                        }
                    />
                </div>
                <div style="
                    width: 50%;
                    padding: 10px;
                ">
                    <InputGroup
                        fill=self.props.fill
                        large=self.props.large
                        small=self.props.small
                        round=self.props.round
                        disabled=self.props.disabled
                        left_icon=IconName::Tag
                        placeholder={"Find tags"}
                        right_element=html! {
                            <Tag
                                minimal=true
                            />
                        }
                    />
                </div>
                <div style="
                    width: 50%;
                    padding: 10px;
                ">
                    <InputGroup
                        fill=self.props.fill
                        large=self.props.large
                        small=self.props.small
                        round=self.props.round
                        disabled=self.props.disabled
                        placeholder={"Add people or groups..."}
                    />
                </div>
            </div>
        }
    }
}
