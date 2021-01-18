use yew::prelude::*;

pub struct ControlGroup {
    props: ControlGroupProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ControlGroupProps {
    #[prop_or_default]
    pub fill: bool,
    #[prop_or_default]
    pub vertical: bool,
    #[prop_or_default]
    pub children: html::Children,
}

impl Component for ControlGroup {
    type Message = ();
    type Properties = ControlGroupProps;

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
                class=classes!(
                    "bp3-control-group",
                    self.props.fill.then(|| "bp3-fill"),
                    self.props.vertical.then(|| "bp3-vertical"),
                )
            >
                {self.props.children.clone()}
            </div>
        }
    }
}
