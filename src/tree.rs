use crate::collapse::Collapse;
use crate::icon::Icon;
use crate::Intent;
use gloo::timers::callback::Timeout;
use id_tree::*;
use std::cell::{Ref, RefCell, RefMut};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::rc::Rc;
use yew::prelude::*;

#[derive(Debug)]
pub struct TreeData<T> {
    tree: Rc<RefCell<id_tree::Tree<NodeData<T>>>>,
    version: usize,
}

impl<T> Clone for TreeData<T> {
    fn clone(&self) -> Self {
        Self {
            tree: self.tree.clone(),
            version: self.version,
        }
    }
}

impl<T> yew::html::ImplicitClone for TreeData<T> {}

impl<T> PartialEq for TreeData<T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.tree, &other.tree) && self.version == other.version
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

#[derive(Debug)]
pub struct NodeData<T> {
    pub disabled: bool,
    pub has_caret: bool,
    pub icon: Icon,
    pub icon_color: Option<AttrValue>,
    pub icon_intent: Option<Intent>,
    pub is_expanded: bool,
    pub is_selected: bool,
    pub label: Html,
    pub secondary_label: Option<Html>,
    pub data: T,
}

impl<T: Default> Default for NodeData<T> {
    fn default() -> Self {
        Self {
            disabled: false,
            has_caret: false,
            icon: Default::default(),
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
            icon: self.icon.clone(),
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
    fn render_children(&self, ctx: &Context<Self>, node_id: &NodeId, depth: u32) -> Html {
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
                        icon={data.icon.clone()}
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
    // NOTE: to prevent event bubbling, see https://github.com/yewstack/yew/issues/3041
    callback_timeout: Option<Timeout>,
}

#[derive(Clone, Properties)]
struct TreeNodeProps {
    node_id: NodeId,
    disabled: bool,
    has_caret: bool,
    icon: Icon,
    icon_color: Option<AttrValue>,
    icon_intent: Option<Intent>,
    is_expanded: bool,
    is_selected: bool,
    label: Html,
    secondary_label: Option<Html>,
    on_collapse: Option<Callback<(NodeId, MouseEvent)>>,
    on_expand: Option<Callback<(NodeId, MouseEvent)>>,
    onclick: Option<Callback<(NodeId, MouseEvent)>>,
    children: yew::Children,
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
            callback_timeout: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        if ctx.props().disabled {
            return false;
        }

        let node_id = ctx.props().node_id.clone();
        match msg {
            TreeNodeMessage::CaretClick(event) => {
                if ctx.props().is_expanded {
                    if let Some(on_collapse) = ctx.props().on_collapse.clone() {
                        self.register_callback(on_collapse, (node_id, event));
                    }
                } else if let Some(on_expand) = ctx.props().on_expand.clone() {
                    self.register_callback(on_expand, (node_id, event));
                }
            }
            TreeNodeMessage::Click(event) => {
                if let Some(onclick) = ctx.props().onclick.clone() {
                    self.register_callback(onclick, (node_id, event));
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
                ctx.props().is_selected.then_some("bp3-tree-node-selected"),
                ctx.props().disabled.then_some("bp3-disabled"),
            )}>
                <div
                    class="bp3-tree-node-content"
                    style={content_style}
                    onclick={self.handler_click.clone()}
                >
                    {
                        if ctx.props().has_caret {
                            html! {
                                <Icon
                                    class={classes!(
                                        "bp3-tree-node-caret",
                                        if ctx.props().is_expanded {
                                            "bp3-tree-node-caret-open"
                                        } else {
                                            "bp3-tree-node-caret-closed"
                                        },
                                    )}
                                    icon={Icon::ChevronRight}
                                    onclick={self.handler_caret_click.clone()}
                                    aria_hidden=true
                                    tab_index={-1}
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
                        icon={ctx.props().icon.clone()}
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

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
        self.callback_timeout.take();
    }
}

impl TreeNode {
    fn register_callback<IN: 'static>(&mut self, callback: Callback<IN>, value: IN) {
        if self.callback_timeout.is_none() {
            self.callback_timeout
                .replace(Timeout::new(0, move || callback.emit(value)));
        }
    }
}
