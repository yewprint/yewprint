use crate::ConditionalClass;
use yew::prelude::*;

pub struct ButtonGroup {
    props: ButtonGroupProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ButtonGroupProps {
    #[prop_or_default]
    pub minimal: ConditionalClass,
    #[prop_or_default]
    pub vertical: ConditionalClass,
    #[prop_or_default]
    pub fill: ConditionalClass,
    #[prop_or_default]
    pub large: ConditionalClass,
    #[prop_or_default]
    pub children: html::Children,
}

impl Component for ButtonGroup {
    type Message = ();
    type Properties = ButtonGroupProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
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
            <div
                class=(
                    "bp3-button-group",
                    self.props.minimal.map_some("bp3-minimal"),
                    self.props.fill.map_some("bp3-fill"),
                    self.props.large.map_some("bp3-large"),
                    self.props.vertical.map_some("bp3-vertical"),
                )
            >
                {self.props.children.clone()}
            </div>
        }
    }
}
