use crate::{ConditionalClass, Icon, IconName};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::mem;
use yew::prelude::*;

pub struct HtmlSelect<T: Clone + PartialEq + Hash + 'static> {
    props: Props<T>,
    link: ComponentLink<Self>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props<T: Clone + PartialEq + 'static> {
    #[prop_or_default]
    pub fill: ConditionalClass,
    #[prop_or_default]
    pub minimal: ConditionalClass,
    #[prop_or_default]
    pub large: ConditionalClass,
    #[prop_or_default]
    pub disabled: ConditionalClass,
    #[prop_or_default]
    pub icon: Option<IconName>,
    #[prop_or_default]
    pub title: Option<String>,
    #[prop_or_default]
    pub onchange: Callback<T>,
    pub options: Vec<(T, String)>,
    #[prop_or_default]
    pub value: Option<T>,
}

impl<T: Clone + PartialEq + Hash + 'static> Component for HtmlSelect<T> {
    type Message = ChangeData;
    type Properties = Props<T>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let i = if let ChangeData::Select(select) = msg {
            select.selected_index()
        } else {
            unreachable!("unexpected ChangeData variant: {:?}", msg);
        };
        if i >= 0 {
            let i = i as usize;
            let variant = self.props.options[i].0.clone();
            self.props.onchange.emit(variant);
        }
        false
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
            .options
            .iter()
            .map(|(value, label)| {
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
                    <option selected=selected value=value>
                        {label}
                    </option>
                }
            })
            .collect::<Html>();

        html! {
            <div
                class=(
                    "bp3-html-select",
                    self.props.minimal.map_some("bp3-minimal"),
                    self.props.large.map_some("bp3-large"),
                    self.props.fill.map_some("bp3-fill"),
                    self.props.disabled.map_some("bp3-disabled"),
                )
            >
                <select
                    disabled=*self.props.disabled
                    onchange={self.link.callback(|x| x)}
                    value?={
                        self.props.value
                            .as_ref()
                            .map(|value| {
                                let mut hasher = DefaultHasher::new();
                                value.hash(&mut hasher);
                                hasher.finish()
                            })
                    }
                    title?={self.props.title.clone()}
                >
                    {option_children}
                </select>
                <Icon icon=IconName::DoubleCaretVertical/>
            </div>
        }
    }
}
