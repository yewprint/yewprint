use yew::prelude::*;
use yewprint::id_tree::{InsertBehavior, Node, NodeId, TreeBuilder};
use yewprint::{Icon, IconName, Intent, NodeData, Tree, TreeData};

pub struct Example {
    tree: TreeData<i32>,
    callback_expand_node: Callback<(NodeId, MouseEvent)>,
    callback_select_node: Callback<(NodeId, MouseEvent)>,
}

pub enum Msg {
    ExpandNode(NodeId),
    SelectNode(NodeId),
}

impl Component for Example {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut tree = TreeBuilder::new().build();
        let root_id = tree
            .insert(
                Node::new(NodeData {
                    data: 0,
                    ..Default::default()
                }),
                InsertBehavior::AsRoot,
            )
            .unwrap();
        let dir1 = tree
            .insert(
                Node::new(NodeData {
                    icon: Some(IconName::FolderClose),
                    label: "Big directory".into(),
                    has_caret: true,
                    data: 1,
                    ..Default::default()
                }),
                InsertBehavior::UnderNode(&root_id),
            )
            .unwrap();
        for i in 0..10 {
            let dir2 = tree
                .insert(
                    Node::new(NodeData {
                        icon: Some(IconName::FolderClose),
                        label: format!("Directory {}", i + 1).into(),
                        has_caret: true,
                        data: 1,
                        ..Default::default()
                    }),
                    InsertBehavior::UnderNode(&dir1),
                )
                .unwrap();
            for i in 0..10 {
                tree.insert(
                    Node::new(NodeData {
                        icon: Some(IconName::Document),
                        label: format!("File {}", i + 1).into(),
                        data: i,
                        ..Default::default()
                    }),
                    InsertBehavior::UnderNode(&dir2),
                )
                .unwrap();
            }
        }
        tree.insert(
            Node::new(NodeData {
                icon: Some(IconName::Tag),
                icon_intent: Some(Intent::Primary),
                label: "Outer file".into(),
                secondary_label: Some(html!(<Icon icon={IconName::EyeOpen} />)),
                data: 3,
                ..Default::default()
            }),
            InsertBehavior::UnderNode(&root_id),
        )
        .unwrap();

        Self {
            tree: tree.into(),
            callback_expand_node: link.callback(|(node_id, _)| Msg::ExpandNode(node_id)),
            callback_select_node: link.callback(|(node_id, _)| Msg::SelectNode(node_id)),
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::ExpandNode(node_id) => {
                let mut tree = self.tree.borrow_mut();
                let node = tree.get_mut(&node_id).unwrap();
                let data = node.data_mut();
                data.is_expanded ^= true;
                data.icon = Some(if data.is_expanded {
                    IconName::FolderOpen
                } else {
                    IconName::FolderClose
                })
            }
            Msg::SelectNode(node_id) => {
                let mut tree = self.tree.borrow_mut();
                let node = tree.get_mut(&node_id).unwrap();
                node.data_mut().is_selected ^= true;
            }
        }

        true
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        true
    }

    fn view(&self) -> Html {
        html! {
            <Tree<i32>
                tree={self.tree.clone()}
                on_collapse={Some(self.callback_expand_node.clone())}
                on_expand={Some(self.callback_expand_node.clone())}
                onclick={Some(self.callback_select_node.clone())}
            />
        }
    }
}
