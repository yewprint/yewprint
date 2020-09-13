use crate::collapse::Collapse;
use crate::icon::{Icon, IconName};
use yew::prelude::*;

pub struct Tree<T: Clone + PartialEq> {
    props: Props<T>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props<T: Clone + PartialEq> {
    #[prop_or_default]
    pub is_expanded: bool,
    pub tree: id_tree::Tree<NodeData<T>>,
}

#[derive(Clone, PartialEq)]
pub struct NodeData<T: Clone + PartialEq> {
    pub disabled: bool,
    pub has_caret: bool,
    pub icon: Option<IconName>,
    pub is_expanded: bool,
    pub is_selected: bool,
    pub label: yew::virtual_dom::VNode,
    pub on_collapse: Option<Callback<(id_tree::NodeId, MouseEvent)>>,
    pub on_expand: Option<Callback<(id_tree::NodeId, MouseEvent)>>,
    pub data: T,
}

impl<T: Clone + PartialEq + 'static> Component for Tree<T> {
    type Message = ();
    type Properties = Props<T>;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Tree { props }
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
        let mut nodes = Vec::new();

        if let Some(root_id) = self.props.tree.root_node_id() {
            for node_id in self.props.tree.children_ids(root_id).unwrap() {
                let node = self.props.tree.get(node_id).unwrap();
                let data = node.data();
                let on_collapse = {
                    let node_id = node_id.clone();
                    data.on_collapse
                        .clone()
                        .map(move |x| x.reform(move |event| (node_id.clone(), event)))
                };
                let on_expand = {
                    let node_id = node_id.clone();
                    data.on_expand
                        .clone()
                        .map(move |x| x.reform(move |event| (node_id.clone(), event)))
                };

                nodes.push(html! {
                    <TreeNode
                        disabled=data.disabled
                        has_caret=data.has_caret
                        icon=data.icon
                        is_expanded=data.is_expanded
                        is_selected=data.is_selected
                        label=data.label.clone()
                        on_collapse=on_collapse
                        on_expand=on_expand
                    >
                        {"Example Stuff"}
                    </TreeNode>
                });
            }
        }

        html! {
            <div class="bp3-tree">
                <ul class="bp3-tree-node-list">
                    {nodes}
                </ul>
            </div>
        }
    }
}

pub struct TreeNode {
    props: TreeNodeProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct TreeNodeProps {
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub has_caret: bool,
    #[prop_or_default]
    pub icon: Option<IconName>,
    #[prop_or_default]
    pub is_expanded: bool,
    #[prop_or_default]
    pub is_selected: bool,
    #[prop_or_default]
    pub label: yew::virtual_dom::VNode,
    #[prop_or_default]
    pub on_collapse: Option<Callback<MouseEvent>>,
    #[prop_or_default]
    pub on_expand: Option<Callback<MouseEvent>>,
    #[prop_or_default]
    pub children: html::Children,
}

impl Component for TreeNode {
    type Message = ();
    type Properties = TreeNodeProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        TreeNode { props }
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
        html! {
            <li class="bp3-tree-node">
                <div class="bp3-tree-node-content">
                    {
                        if self.props.has_caret {
                            let mut class = "bp3-tree-node-caret ".to_string();
                            class.push_str(if self.props.is_expanded {
                                "bp3-tree-node-caret-open"
                            } else {
                                "bp3-tree-node-caret-closed"
                            });

                            html! {
                                <Icon
                                    class=class
                                    icon=IconName::ChevronRight
                                    onclick={
                                        if self.props.disabled {
                                            None
                                        } else if self.props.is_expanded {
                                            self.props.on_collapse.clone()
                                        } else {
                                            self.props.on_expand.clone()
                                        }
                                    }
                                />
                            }
                        } else {
                            html! {
                                <span class="bp3-tree-node-caret-none" />
                            }
                        }
                    }
                    <Icon class="bp3-tree-node-icon" icon=self.props.icon.unwrap_or_default() />
                    <span class="bp3-tree-node-label">{self.props.label.clone()}</span>
                </div>
                <Collapse is_open=self.props.is_expanded>
                    {self.props.children.clone()}
                </Collapse>
            </li>
        }
    }
}
