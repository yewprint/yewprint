use boolinator::Boolinator;
use yew::prelude::*;

pub struct ButtonGroup {
    props: ButtonGroupProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ButtonGroupProps {
    #[prop_or_default]
    pub minimal: bool,
    #[prop_or_default]
    pub vertical: bool,
    #[prop_or_default]
    pub fill: bool,
    #[prop_or_default]
    pub large: bool,
    #[prop_or_default]
    pub style: Option<String>,
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
                    self.props.minimal.as_some("bp3-minimal"),
                    self.props.fill.as_some("bp3-fill"),
                    self.props.large.as_some("bp3-large"),
                    self.props.vertical.as_some("bp3-vertical"),
                )
                style?=self.props.style.clone()
            >
                {self.props.children.clone()}
            </div>
        }
    }
}
