use boolinator::Boolinator;
use yew::prelude::*;

pub struct RadioGroup {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub inline: bool,
    #[prop_or_default]
    pub label: Option<yew::virtual_dom::VNode>,
    #[prop_or_default]
    pub label_class: Option<String>,
    #[prop_or_default]
    pub children: html::Children,
}

impl Component for RadioGroup {
    type Message = ();
    type Properties = Props;

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
                    "bp3-radio-group",
                    self.props.disabled.as_some("bp3-disabled"),
                    self.props.inline.as_some("bp3-inline"),
                )
            >
            {
                if let Some(label) = self.props.label.clone() {
                    html! {
                        <label
                            class=(
                                "bp3-label",
                                self.props.label_class.clone())
                        >
                            {label}
                        </label>
                    }
                } else {
                    html!()
                }
            }
                {self.props.children.clone()}
            </div>
        }
    }
}
