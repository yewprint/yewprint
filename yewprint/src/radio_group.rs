use yew::prelude::*;

pub struct RadioGroup<T: Clone + PartialEq + 'static> {
    props: RadioGroupProps<T>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct RadioGroupProps<T: Clone + PartialEq + 'static> {
    #[prop_or_default]
    pub label: Option<yew::virtual_dom::VNode>,
    #[prop_or_default]
    pub label_class: Option<String>,
    #[prop_or_default]
    pub children: Vec<T>,
    #[prop_or_default]
    pub name: String,
    #[prop_or_default]
    pub selected_value: Option<T>,
}

impl<T: Clone + PartialEq + 'static> Component for RadioGroup<T>
where
    yew::Classes: From<T>,
{
    type Message = ();
    type Properties = RadioGroupProps<T>;

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
                    self.props.name.clone(),
                    self.props.children.clone(),
                    self.props.selected_value.clone()
                )
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
            </div>
        }
    }
}
