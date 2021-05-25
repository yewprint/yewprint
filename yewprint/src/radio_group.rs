use crate::Radio;
use std::collections::hash_map::DefaultHasher;
use std::fmt::Display;
use std::hash::{Hash, Hasher};
use yew::prelude::*;

pub struct RadioGroup<T: Clone + PartialEq + Hash + 'static> {
    props: RadioGroupProps<T>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct RadioGroupProps<T: Clone + PartialEq + Hash + 'static> {
    #[prop_or_default]
    pub label: Option<yew::virtual_dom::VNode>,
    #[prop_or_default]
    pub label_class: Option<String>,
    #[prop_or_default]
    pub option_children: Vec<(T, String)>,
    #[prop_or_default]
    pub name: String,
    #[prop_or_default]
    pub value: Option<T>,
    #[prop_or_default]
    pub class: Classes,
}

impl<T: Clone + PartialEq + Display + Default + Hash + 'static> Component for RadioGroup<T> {
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
        let option_children = self
            .props
            .option_children
            .iter()
            .map(|(value, name)| {
                let selected = self
                    .props
                    .value
                    .as_ref()
                    .map(|x| value == x)
                    .unwrap_or_default();
                let value = {
                    let mut hasher = DefaultHasher::new();
                    value.hash(&mut hasher);
                    hasher.finish()
                };

                html! {
                    <Radio<T>
                        value=value
                        label=html!(name)
                    />
                }
            })
            .collect::<Html>();

        html! {
            <div
                class=classes!(
                    "bp3-radio-group",
                    self.props.class.clone(),
                )
            >
            {
                if let Some(label) = self.props.label.clone() {
                    html! {
                        <label
                            class=classes!(
                                "bp3-label",
                                self.props.label_class.clone(),
                            )
                        >
                            {label}
                        </label>
                    }
                } else {
                    html!()
                }
            }
            {option_children}
            </div>
        }
    }
}
