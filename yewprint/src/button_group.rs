use yew::prelude::*;

pub struct ButtonGroup {
    props: ButtonGroupProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ButtonGroupProps {
    #[prop_or_default]
    pub is_minimal: bool,
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
        let mut classname = String::from("bp3-button-group");

        if self.props.is_minimal {
            classname.push_str(" bp3-minimal");
        }

        html! {
            <div class=classname>
              {self.props.children.clone()}
            </div>
        }
    }
}
