use crate::ConditionalClass;
use yew::prelude::*;

pub struct Tabs {
    props: TabsProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct TabsProps {
    #[prop_or_default]
    pub animate: bool,
    #[prop_or_default]
    pub default_selected_tab_id: Option<usize>,
    #[prop_or_default]
    pub id: usize,
    #[prop_or_default]
    pub large: ConditionalClass,
    #[prop_or_default]
    pub render_active_panel_only: ConditionalClass,
    pub selected_tab_id: String,
    #[prop_or_default]
    pub vertical: ConditionalClass,
    #[prop_or_default]
    pub onchange: Callback<usize>,
    #[prop_or_default]
    pub class: String,
    pub tabs: Vec<Tab>,
}

impl Component for Tabs {
    type Message = ();
    type Properties = TabsProps;

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
        let title_ids = self
            .props
            .tabs
            .iter()
            .map(|x| format!("bp3-tab-title_{}_{}", self.props.id, x.id))
            .collect::<Vec<_>>();
        let panel_ids = self
            .props
            .tabs
            .iter()
            .map(|x| format!("bp3-tab-panel_{}_{}", self.props.id, x.id))
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
                        self.props.tabs
                            .iter()
                            .enumerate()
                            .map(|(i, props)| {
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
                                        id=title_ids[i]
                                        aria-controls=panel_ids[i]
                                        data-tab-id=props.id
                                    >
                                        { props.title.clone() }
                                    </div>
                                }
                            })
                            .collect::<Html>()
                    }
                </div>
                {
                    self.props.tabs
                        .iter()
                        .enumerate()
                        .map(|(i, props)| {
                            let selected = self.props.selected_tab_id == props.id;

                            html! {
                                <div
                                    class=(
                                        "bp3-tab-panel",
                                        props.panel_class.clone(),
                                    )
                                    aria-labelledby=title_ids[i]
                                    aria-hidden=!selected
                                    role="tabpanel"
                                    id=panel_ids[i]
                                    key=panel_ids[i].clone()
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
pub struct Tab {
    pub disabled: bool,
    pub id: String,
    pub parent_id: usize,
    pub title: Html,
    pub panel: Html,
    pub title_class: String,
    pub panel_class: String,
}
