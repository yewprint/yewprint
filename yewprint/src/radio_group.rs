use yew::prelude::*;

pub struct RadioGroup {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub onchange: Callback<ChangeData>,
    #[prop_or_default]
    pub label: Option<yew::virtual_dom::VNode>,
    #[prop_or_default]
    pub label_class: Option<String>,
    #[prop_or_default]
    pub children: html::Children,
    #[prop_or_default]
    pub name: String,
    // selected_value
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
                class=classes!(
                    "bp3-radio-group",
                )
                onchange={self.props.onchange.clone()}
                name={self.props.name.clone()}
            >
            {
                if let Some(label) = self.props.label.clone() {
                    html! {
                        <label
                            class=classes!(
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
