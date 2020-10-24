use crate::ConditionalClass;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use yew::prelude::*;

pub struct Tabs<T: Clone + PartialEq> {
    props: TabsProps<T>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct TabsProps<T: Clone + PartialEq> {
    #[prop_or_default]
    pub animate: bool,
    #[prop_or_default]
    pub default_selected_tab_id: Option<T>,
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub large: ConditionalClass,
    #[prop_or_default]
    pub render_active_panel_only: ConditionalClass,
    pub selected_tab_id: T,
    #[prop_or_default]
    pub vertical: ConditionalClass,
    #[prop_or_default]
    pub onchange: Callback<T>,
    #[prop_or_default]
    pub class: String,
    pub tabs: Vec<Tab<T>>,
}

impl<T: Clone + PartialEq + Hash + 'static> Component for Tabs<T> {
    type Message = ();
    type Properties = TabsProps<T>;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Tabs { props }
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
        let tabs = self
            .props
            .tabs
            .iter()
            .map(|x| {
                let mut hasher = DefaultHasher::new();
                x.id.hash(&mut hasher);
                let id = hasher.finish();
                let title_id = format!("bp3-tab-title_{}_{}", self.props.id, id);
                let panel_id = format!("bp3-tab-panel_{}_{}", self.props.id, id);
                (x, id, title_id, panel_id)
            })
            .collect::<Vec<_>>();

        html! {
            <div
                class=(
                    "bp3-tabs",
                    self.props.vertical.map_some("bp3-vertical"),
                    self.props.class.clone(),
                )
            >
                <div
                    class=(
                        "bp3-tab-list",
                        self.props.large.map_some("bp3-large"),
                    )
                >
                    {
                        if self.props.animate {
                            html! {
                                <div class="bp3-tab-indicator-wrapper">
                                    <div class="bp3-tab-indicator" />
                                </div>
                            }
                        } else {
                            html!()
                        }
                    }
                    {
                        tabs
                            .iter()
                            .map(|(props, id, title_id, panel_id)| {
                                let selected = self.props.selected_tab_id == props.id;

                                html! {
                                    <div
                                        class=(
                                            "bp3-tab",
                                            props.title_class.clone(),
                                        )
                                        aria-disabled=props.disabled
                                        aria-expanded=selected
                                        aria-selected=selected
                                        role="tab"
                                        tabIndex?={
                                            if props.disabled {
                                                Some("0")
                                            } else {
                                                None
                                            }
                                        }
                                        id=title_id
                                        aria-controls=panel_id
                                        data-tab-id=id
                                    >
                                        { props.title.clone() }
                                    </div>
                                }
                            })
                            .collect::<Html>()
                    }
                </div>
                {
                    tabs
                        .iter()
                        .map(|(props, _id, title_id, panel_id)| {
                            let selected = self.props.selected_tab_id == props.id;

                            html! {
                                <div
                                    class=(
                                        "bp3-tab-panel",
                                        props.panel_class.clone(),
                                    )
                                    aria-labelledby=title_id
                                    aria-hidden=!selected
                                    role="tabpanel"
                                    id=panel_id
                                    key=panel_id.clone()
                                >
                                    { props.panel.clone() }
                                </div>
                            }
                        })
                        .collect::<Html>()
                }
            </div>
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Tab<T> {
    pub disabled: bool,
    pub id: T,
    pub title: Html,
    pub panel: Html,
    pub title_class: String,
    pub panel_class: String,
}
