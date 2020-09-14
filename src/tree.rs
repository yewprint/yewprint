use crate::collapse::Collapse;
use crate::icon::{Icon, IconName};
use crate::Intent;
pub use id_tree::*;
use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;
use yew::prelude::*;

#[derive(Clone)]
pub struct TreeData<T> {
    tree: Rc<RefCell<id_tree::Tree<NodeData<T>>>>,
    version: usize,
}

impl<T> PartialEq for TreeData<T> {
    fn eq(&self, other: &Self) -> bool {
        self.version == other.version
    }
}

impl<T> TreeData<T> {
    pub fn borrow(&self) -> Ref<id_tree::Tree<NodeData<T>>> {
        self.tree.borrow()
    }

    pub fn borrow_mut(&mut self) -> RefMut<id_tree::Tree<NodeData<T>>> {
        self.version = self.version.wrapping_add(1);
        self.tree.borrow_mut()
    }
}

impl<T> From<id_tree::Tree<NodeData<T>>> for TreeData<T> {
    fn from(tree: id_tree::Tree<NodeData<T>>) -> Self {
        TreeData {
            tree: Rc::new(RefCell::new(tree)),
            version: 0,
        }
    }
}

pub struct Tree<T: Clone> {
    props: Props<T>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props<T: Clone> {
    #[prop_or_default]
    pub is_expanded: bool,
    pub tree: TreeData<T>,
    #[prop_or_default]
    pub on_collapse: Option<Callback<(id_tree::NodeId, MouseEvent)>>,
    #[prop_or_default]
    pub on_expand: Option<Callback<(id_tree::NodeId, MouseEvent)>>,
    #[prop_or_default]
    pub onclick: Option<Callback<(id_tree::NodeId, MouseEvent)>>,
}

pub struct NodeData<T> {
    pub disabled: bool,
    pub has_caret: bool,
    pub icon: Option<IconName>,
    pub icon_color: Option<String>,
    pub icon_intent: Option<Intent>,
    pub is_expanded: bool,
    pub is_selected: bool,
    pub label: yew::virtual_dom::VNode,
    pub secondary_label: Option<yew::virtual_dom::VNode>,
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
        let tree = self.props.tree.borrow();

        let nodes = if let Some(root_id) = tree.root_node_id() {
            self.render_children(root_id, 0)
        } else {
            Default::default()
        };

        html! {
            <div class="bp3-tree">
                {nodes}
            </div>
        }
    }
}

impl<T: Clone> Tree<T> {
    fn render_children(&self, node_id: &NodeId, depth: u32) -> yew::virtual_dom::VNode {
        let tree = self.props.tree.borrow();
        let mut nodes = Vec::new();

        for node_id in tree.children_ids(node_id).unwrap() {
            let node = tree.get(node_id).unwrap();
            let data = node.data();
            let on_collapse = {
                let node_id = node_id.clone();
                self.props.on_collapse.clone().map(move |x| {
                    x.reform(move |event: MouseEvent| {
                        event.stop_propagation();
                        (node_id.clone(), event)
                    })
                })
            };
            let on_expand = {
                let node_id = node_id.clone();
                self.props.on_expand.clone().map(move |x| {
                    x.reform(move |event: MouseEvent| {
                        event.stop_propagation();
                        (node_id.clone(), event)
                    })
                })
            };
            let onclick = {
                let node_id = node_id.clone();
                self.props
                    .onclick
                    .clone()
                    .map(move |x| x.reform(move |event| (node_id.clone(), event)))
            };
            let inner_nodes = self.render_children(node_id, depth + 1);

            nodes.push(html! {
                <TreeNode
                    disabled=data.disabled
                    has_caret=data.has_caret
                    icon=data.icon
                    icon_color=data.icon_color.clone()
                    icon_intent=data.icon_intent
                    is_expanded=data.is_expanded
                    is_selected=data.is_selected
                    label=data.label.clone()
                    secondary_label=data.secondary_label.clone()
                    on_collapse=on_collapse
                    on_expand=on_expand
                    onclick=onclick
                    depth=depth
                >
                    {inner_nodes}
                </TreeNode>
            });
        }

        html! {
            <ul class="bp3-tree-node-list">
                {nodes}
            </ul>
        }
    }
}

pub struct TreeNode {
    props: TreeNodeProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct TreeNodeProps {
    pub disabled: bool,
    pub has_caret: bool,
    pub icon: Option<IconName>,
    pub icon_color: Option<String>,
    pub icon_intent: Option<Intent>,
    pub is_expanded: bool,
    pub is_selected: bool,
    pub label: yew::virtual_dom::VNode,
    pub secondary_label: Option<yew::virtual_dom::VNode>,
    pub on_collapse: Option<Callback<MouseEvent>>,
    pub on_expand: Option<Callback<MouseEvent>>,
    pub onclick: Option<Callback<MouseEvent>>,
    pub children: html::Children,
    pub depth: u32,
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
        let mut container_class = Classes::from("bp3-tree-node");
        if self.props.is_selected {
            container_class.push("bp3-tree-node-selected");
        }
        let mut content_class = Classes::from("bp3-tree-node-content");
        content_class.push(&format!("bp3-tree-node-content-{}", self.props.depth));

        html! {
            <li class=container_class>
                <div class=content_class onclick?={self.props.onclick.clone()}>
                    {
                        if self.props.has_caret {
                            let mut class = Classes::from("bp3-tree-node-caret");
                            class.push(if self.props.is_expanded {
                                "bp3-tree-node-caret-open"
                            } else {
                                "bp3-tree-node-caret-closed"
                            });

                            html! {
                                <Icon
                                    class=class.to_string()
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
                    <Icon
                        class="bp3-tree-node-icon"
                        icon=self.props.icon.unwrap_or_default()
                        color=self.props.icon_color.clone(),
                        intent=self.props.icon_intent,
                    />
                    <span class="bp3-tree-node-label">{self.props.label.clone()}</span>
                    {
                        if let Some(label) = self.props.secondary_label.clone() {
                            html!(<span class="bp3-tree-node-secondary-label">{label}</span>)
                        } else {
                            html!()
                        }
                    }
                </div>
                <Collapse is_open=self.props.is_expanded>
                    {self.props.children.clone()}
                </Collapse>
            </li>
        }
    }
}
