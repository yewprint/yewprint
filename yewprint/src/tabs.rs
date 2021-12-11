use std::collections::{hash_map::DefaultHasher, HashMap};
use std::hash::{Hash, Hasher};
use web_sys::HtmlElement;
use yew::prelude::*;

pub struct Tabs<T: Clone + PartialEq + Hash + 'static> {
    props: TabsProps<T>,
    tab_refs: HashMap<u64, NodeRef>,
    indicator_ref: NodeRef,
}

#[derive(Clone, PartialEq, Properties)]
pub struct TabsProps<T: Clone + PartialEq> {
    #[prop_or_default]
    pub animate: bool,
    #[prop_or_default]
    pub default_selected_tab_id: Option<T>,
    pub id: String,
    #[prop_or_default]
    pub large: bool,
    #[prop_or_default]
    pub render_active_panel_only: bool,
    pub selected_tab_id: T,
    #[prop_or_default]
    pub vertical: bool,
    #[prop_or_default]
    pub onchange: Callback<T>,
    #[prop_or_default]
    pub class: Classes,
    pub tabs: Vec<Tab<T>>,
}

impl<T: Clone + PartialEq + Hash + 'static> Component for Tabs<T> {
    type Message = ();
    type Properties = TabsProps<T>;

    fn create(ctx: &Context<Self>) -> Self {
        let tab_refs = ctx
            .props()
            .tabs
            .iter()
            .map(|x| {
                let mut hasher = DefaultHasher::new();
                x.id.hash(&mut hasher);
                let id = hasher.finish();
                (id, NodeRef::default())
            })
            .collect::<HashMap<_, _>>();

        Self {
            props: ctx.props(),
            tab_refs,
            indicator_ref: Default::default(),
        }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        true
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
                class={classes!(
                    "bp3-tabs",
                    self.props.vertical.then(|| "bp3-vertical"),
                    self.props.class.clone(),
                )}
            >
                <div
                    class={classes!(
                        "bp3-tab-list",
                        self.props.large.then(|| "bp3-large"),
                    )}
                >
                    {
                        if self.props.animate {
                            html! {
                                <div
                                    class="bp3-tab-indicator-wrapper"
                                    ref={self.indicator_ref.clone()}
                                >
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
                            .map(|(props, id, title_id, panel_id, selected)| html! {
                                <div
                                    class={classes!(
                                        "bp3-tab",
                                        props.title_class.clone(),
                                    )}
                                    aria-disabled=props.disabled.then(|| "true")
                                    aria-expanded=selected.to_string()
                                    aria-selected=selected.to_string()
                                    role="tab"
                                    tabIndex={(!props.disabled).then(|| "0")}
                                    id=title_id.to_string()
                                    aria-controls=panel_id.to_string()
                                    data-tab-id=id.to_string()
                                    onclick={(!props.disabled).then(|| {
                                        let tab_id = props.id.clone();
                                        self
                                            .props
                                            .onchange
                                            .reform(move |_| tab_id.clone())
                                    })}
                                    key=*id
                                    ref=self.tab_refs[id].clone()
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
                                class={classes!(
                                    "bp3-tab-panel",
                                    selected.then(|| props.panel_class.clone()),
                                )}
                                aria-labelledby={title_id.to_string()}
                                aria-hidden={(!selected).then(|| "true")}
                                role="tabpanel"
                                id={panel_id.to_string()}
                                key={*id}
                            >
                                { props.panel.clone() }
                            </div>
                        })
                        .collect::<Html>()
                }
            </div>
        }
    }

    fn rendered(&mut self, _first_render: bool) {
        if self.props.animate {
            let mut hasher = DefaultHasher::new();
            self.props.selected_tab_id.hash(&mut hasher);
            let id = hasher.finish();
            let indicator = self.indicator_ref.cast::<HtmlElement>().unwrap();

            if let Some(element) = self.tab_refs[&id].cast::<HtmlElement>() {
                let indicator_style = format!(
                    "height: {}px; width: {}px; \
                                    transform: translateX({}px) translateY({}px);",
                    element.client_height(),
                    element.client_width(),
                    element.offset_left(),
                    element.offset_top(),
                );
                let _ = indicator.set_attribute("style", &indicator_style);
            }
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Tab<T> {
    pub disabled: bool,
    pub id: T,
    pub title: Html,
    pub panel: Html,
    pub title_class: Classes,
    pub panel_class: Classes,
}
