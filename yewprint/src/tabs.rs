use crate::ConditionalClass;
use std::collections::{hash_map::DefaultHasher, HashMap};
use std::hash::{Hash, Hasher};
use web_sys::HtmlElement;
use yew::prelude::*;

pub struct Tabs<T: Clone + PartialEq + Hash + 'static> {
    link: ComponentLink<Self>,
    props: TabsProps<T>,
    tab_refs: HashMap<u64, NodeRef>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct TabsProps<T: Clone + PartialEq> {
    #[prop_or_default]
    pub animate: bool,
    #[prop_or_default]
    pub default_selected_tab_id: Option<T>,
    pub id: String,
    #[prop_or_default]
    pub large: ConditionalClass,
    #[prop_or_default]
    pub render_active_panel_only: bool,
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

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let tab_refs = props
            .tabs
            .iter()
            .map(|x| {
                let mut hasher = DefaultHasher::new();
                x.id.hash(&mut hasher);
                let id = hasher.finish();
                (id, NodeRef::default())
            })
            .collect::<HashMap<_, _>>();

        Tabs {
            props,
            tab_refs,
            link,
        }
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

    fn rendered(&mut self, first_render: bool) {
        if first_render && self.props.animate {
            self.link.send_message(());
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
                let selected = self.props.selected_tab_id == x.id;
                (x, id, title_id, panel_id, selected)
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
                            let mut hasher = DefaultHasher::new();
                            self.props.selected_tab_id.hash(&mut hasher);
                            let id = hasher.finish();

                            if let Some(element) = self.tab_refs[&id].cast::<HtmlElement>()
                            {
                                let indicator_style = format!(
                                    "height: {}px; width: {}px; \
                                    transform: translateX({}px) translateY({}px);",
                                    element.client_height(),
                                    element.client_width(),
                                    element.offset_left(),
                                    element.offset_top(),
                                );

                                html! {
                                    <div class="bp3-tab-indicator-wrapper" style=indicator_style>
                                        <div class="bp3-tab-indicator" />
                                    </div>
                                }
                            } else {
                                html!()
                            }
                        } else {
                            html!()
                        }
                    }
                    {
                        tabs
                            .iter()
                            .map(|(props, id, title_id, panel_id, selected)| html! {
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
                                    onclick?={
                                        if props.disabled {
                                            None
                                        } else {
                                            let tab_id = props.id.clone();
                                            Some(self
                                                .props
                                                .onchange
                                                .reform(move |_| tab_id.clone()))
                                        }
                                    }
                                    key=*id
                                    ref=self.tab_refs[&id].clone()
                                >
                                    { props.title.clone() }
                                </div>
                            })
                            .collect::<Html>()
                    }
                </div>
                {
                    tabs
                        .iter()
                        .filter(|(_, _, _, _, selected)| {
                            !self.props.render_active_panel_only || *selected
                        })
                        .map(|(props, id, title_id, panel_id, selected)| html! {
                            <div
                                class=(
                                    "bp3-tab-panel",
                                    props.panel_class.clone(),
                                )
                                aria-labelledby=title_id
                                aria-hidden=!selected
                                role="tabpanel"
                                id=panel_id
                                key=*id
                            >
                                { props.panel.clone() }
                            </div>
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
    pub title_class: Option<String>,
    pub panel_class: Option<String>,
}
