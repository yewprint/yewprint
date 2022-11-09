use crate::collapse::Collapse;
use crate::icon::{Icon, IconName};
use crate::Intent;
use id_tree::*;
use std::cell::{Ref, RefCell, RefMut};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
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

pub struct Tree<T: Clone + PartialEq> {
    previous_expanded_state: RefCell<HashMap<u64, bool>>,
    phantom: PhantomData<T>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct TreeProps<T: Clone + PartialEq> {
    #[prop_or_default]
    pub is_expanded: bool,
    pub tree: TreeData<T>,
    #[prop_or_default]
    pub on_collapse: Option<Callback<(NodeId, MouseEvent)>>,
    #[prop_or_default]
    pub on_expand: Option<Callback<(NodeId, MouseEvent)>>,
    #[prop_or_default]
    pub onclick: Option<Callback<(NodeId, MouseEvent)>>,
    #[prop_or_default]
    pub class: Classes,
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

impl<T: Default> Default for NodeData<T> {
    fn default() -> Self {
        Self {
            disabled: false,
            has_caret: false,
            icon: None,
            icon_color: None,
            icon_intent: None,
            is_expanded: false,
            is_selected: false,
            label: Default::default(),
            secondary_label: None,
            data: Default::default(),
        }
    }
}

impl<T: Clone> Clone for NodeData<T> {
    fn clone(&self) -> Self {
        Self {
            disabled: self.disabled,
            has_caret: self.has_caret,
            icon: self.icon,
            icon_color: self.icon_color.clone(),
            icon_intent: self.icon_intent,
            is_expanded: self.is_expanded,
            is_selected: self.is_selected,
            label: self.label.clone(),
            secondary_label: self.secondary_label.clone(),
            data: self.data.clone(),
        }
    }
}

impl<T: Clone + PartialEq + 'static> Component for Tree<T> {
    type Message = ();
    type Properties = TreeProps<T>;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            previous_expanded_state: Default::default(),
            phantom: PhantomData,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let tree = ctx.props().tree.borrow();

        let nodes = if let Some(root_id) = tree.root_node_id() {
            self.render_children(ctx, root_id, 0)
        } else {
            html!()
        };

        html! {
            <div class={classes!(
                "bp3-tree",
                ctx.props().class.clone(),
            )}>
                <ul class={classes!("bp3-tree-node-list")}>
                    {nodes}
                </ul>
            </div>
        }
    }
}

// FIXME: The 'static bound here is probably wrong. Fix this at the end of PR.
impl<T: 'static + Clone + PartialEq> Tree<T> {
    fn render_children(
        &self,
        ctx: &Context<Self>,
        node_id: &NodeId,
        depth: u32,
    ) -> yew::virtual_dom::VNode {
        let tree = ctx.props().tree.borrow();
        let node = tree.get(node_id).unwrap();
        let children = node.children();

        children
            .iter()
            .map(|node_id| {
                let key = {
                    let mut hasher = DefaultHasher::new();
                    node_id.hash(&mut hasher);
                    hasher.finish()
                };
                let node = tree.get(node_id).unwrap();
                let data = node.data();
                let previous_is_expanded = self
                    .previous_expanded_state
                    .borrow_mut()
                    .insert(key, data.is_expanded);
                let inner_nodes = if !data.is_expanded && !previous_is_expanded.unwrap_or(true) {
                    Default::default()
                } else {
                    self.render_children(ctx, node_id, depth + 1)
                };

                html! {
                    <TreeNode
                        disabled={data.disabled}
                        has_caret={data.has_caret}
                        icon={data.icon}
                        icon_color={data.icon_color.clone()}
                        icon_intent={data.icon_intent}
                        is_expanded={data.is_expanded}
                        is_selected={data.is_selected}
                        label={data.label.clone()}
                        secondary_label={data.secondary_label.clone()}
                        on_collapse={ctx.props().on_collapse.clone()}
                        on_expand={ctx.props().on_expand.clone()}
                        onclick={ctx.props().onclick.clone()}
                        depth={depth}
                        node_id={node_id.clone()}
                        key={key}
                    >
                        {inner_nodes}
                    </TreeNode>
                }
            })
            .collect::<Html>()
    }
}

struct TreeNode {
    handler_caret_click: Callback<MouseEvent>,
    handler_click: Callback<MouseEvent>,
}

#[derive(Clone, Properties)]
struct TreeNodeProps {
    node_id: NodeId,
    disabled: bool,
    has_caret: bool,
    icon: Option<IconName>,
    icon_color: Option<String>,
    icon_intent: Option<Intent>,
    is_expanded: bool,
    is_selected: bool,
    label: yew::virtual_dom::VNode,
    secondary_label: Option<yew::virtual_dom::VNode>,
    on_collapse: Option<Callback<(NodeId, MouseEvent)>>,
    on_expand: Option<Callback<(NodeId, MouseEvent)>>,
    onclick: Option<Callback<(NodeId, MouseEvent)>>,
    children: html::Children,
    depth: u32,
}

impl PartialEq for TreeNodeProps {
    fn eq(&self, other: &Self) -> bool {
        self.node_id == other.node_id
            && self.disabled == other.disabled
            && self.has_caret == other.has_caret
            && self.icon == other.icon
            && self.icon_color == other.icon_color
            && self.icon_intent == other.icon_intent
            && self.is_expanded == other.is_expanded
            && self.is_selected == other.is_selected
            && self.label == other.label
            && self.secondary_label == other.secondary_label
            && self.on_collapse == other.on_collapse
            && self.on_expand == other.on_expand
            && self.onclick == other.onclick
            && self.depth == other.depth
            && (!other.is_expanded || self.children == other.children)
    }
}

enum TreeNodeMessage {
    CaretClick(MouseEvent),
    Click(MouseEvent),
}

impl Component for TreeNode {
    type Message = TreeNodeMessage;
    type Properties = TreeNodeProps;

    fn create(ctx: &Context<Self>) -> Self {
        TreeNode {
            handler_caret_click: ctx.link().callback(TreeNodeMessage::CaretClick),
            handler_click: ctx.link().callback(TreeNodeMessage::Click),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        if ctx.props().disabled {
            return false;
        }

        match msg {
            TreeNodeMessage::CaretClick(event) => {
                if ctx.props().is_expanded {
                    if let Some(on_collapse) = ctx.props().on_collapse.as_ref() {
                        event.stop_propagation();
                        on_collapse.emit((ctx.props().node_id.clone(), event));
                    }
                } else if let Some(on_expand) = ctx.props().on_expand.as_ref() {
                    event.stop_propagation();
                    on_expand.emit((ctx.props().node_id.clone(), event));
                }
            }
            TreeNodeMessage::Click(event) => {
                if let Some(onclick) = ctx.props().onclick.as_ref() {
                    onclick.emit((ctx.props().node_id.clone(), event));
                }
            }
        }

        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let content_style = format!("padding-left: {}px;", 23 * ctx.props().depth);

        html! {
            <li class={classes!(
                "bp3-tree-node",
                ctx.props().is_selected.then_some("bp3-tree-node-selected")
            )}>
                <div
                    class="bp3-tree-node-content"
                    style={content_style}
                    onclick={self.handler_click.clone()}
                >
                    {
                        if ctx.props().has_caret {
                            let mut class = Classes::from("bp3-tree-node-caret");
                            class.push(if ctx.props().is_expanded {
                                "bp3-tree-node-caret-open"
                            } else {
                                "bp3-tree-node-caret-closed"
                            });

                            html! {
                                <Icon
                                    class={classes!(class.to_string())}
                                    icon={IconName::ChevronRight}
                                    onclick={self.handler_caret_click.clone()}
                                />
                            }
                        } else {
                            html! {
                                <span class="bp3-tree-node-caret-none" />
                            }
                        }
                    }
                    <Icon
                        class={classes!("bp3-tree-node-icon")}
                        icon={ctx.props().icon.unwrap_or(IconName::Blank)}
                        color={ctx.props().icon_color.clone()}
                        intent={ctx.props().icon_intent}
                    />
                    <span class={classes!("bp3-tree-node-label")}>{ctx.props().label.clone()}</span>
                    {
                        if let Some(label) = ctx.props().secondary_label.clone() {
                            html!(
                                <span class={classes!("bp3-tree-node-secondary-label")}>
                                    {label}
                                </span>
                            )
                        } else {
                            html!()
                        }
                    }
                </div>
                <Collapse is_open={ctx.props().is_expanded}>
                    <ul class={classes!("bp3-tree-node-list")}>
                        {ctx.props().children.clone()}
                    </ul>
                </Collapse>
            </li>
        }
    }
}
