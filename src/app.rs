use crate::buttons::Button;
use crate::collapse::Collapse;
use crate::forms::controls::Switch;
use crate::icon::*;
use crate::tree::*;
use crate::Intent;
use yew::prelude::*;

const DARK_BG_COLOR: &str = "#30404d";
const LIGHT_BG_COLOR: &str = "#f5f8fa";
const DARK_FG_COLOR: &str = "#f5f8fa";
const LIGHT_FG_COLOR: &str = "#182026";

pub struct App {
    link: ComponentLink<Self>,
    counter: i64,
    dark_theme: bool,
    collapsed: bool,
    tree: TreeData<i32>,
    callback_expand_node: Callback<(NodeId, MouseEvent)>,
    callback_select_node: Callback<(NodeId, MouseEvent)>,
}

pub enum Msg {
    AddOne,
    ToggleLight,
    ToggleCollapse,
    ExpandNode(NodeId),
    SelectNode(NodeId),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
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
                secondary_label: Some(html!(<Icon icon=IconName::EyeOpen />)),
                data: 3,
                ..Default::default()
            }),
            InsertBehavior::UnderNode(&root_id),
        )
        .unwrap();

        App {
            counter: 0,
            dark_theme: true,
            collapsed: true,
            tree: tree.into(),
            callback_expand_node: link.callback(|(node_id, _)| Msg::ExpandNode(node_id)),
            callback_select_node: link.callback(|(node_id, _)| Msg::SelectNode(node_id)),
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => self.counter += 1,
            Msg::ToggleLight => self.dark_theme ^= true,
            Msg::ToggleCollapse => self.collapsed ^= true,
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

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let (fg_color, bg_color) = if self.dark_theme {
            (DARK_FG_COLOR, DARK_BG_COLOR)
        } else {
            (LIGHT_FG_COLOR, LIGHT_BG_COLOR)
        };
        let style = format!(
            "min-height: 100vh; padding: 10px; background-color: {}; color: {}",
            bg_color, fg_color
        );
        let class = if self.dark_theme { "bp3-dark" } else { "" };

        html! {
            <div class={class} style={style}>
                <p> {"Counter: "} { self.counter }</p>
                <div>
                    <Button onclick=self.link.callback(|_| Msg::AddOne)>{ "Add 1" }</Button>
                </div>
                <div>
                    <Switch
                        onclick=self.link.callback(|_| Msg::ToggleLight)
                        checked=self.dark_theme
                        label="Dark theme"
                    />
                </div>
                <div>
                    <Button onclick=self.link.callback(|_| Msg::ToggleCollapse)>
                        {"Toggle collapse"}
                    </Button>
                    <Collapse
                        is_open=!self.collapsed
                        keep_children_mounted=true
                    >
                        <pre class="bp3-code-block">
                            <div>{"[INFO]: Installing wasm-bindgen..."}</div>
                            <div>{"[INFO]: Optional fields missing from Cargo.toml: 'description', 'repository', and 'license'. These are not necessary, but recommended"}</div>
                            <div>{"[INFO]: :-) Done in 0.69s"}</div>
                            <div>{"[INFO]: :-) Your wasm pkg is ready to publish at /home/cecile/repos/blueprint-rs/./static."}</div>
                            <div>{"     Index: enabled, Upload: disabled, Cache: disabled, Cors: enabled, Range: enabled, Sort: enabled, Threads: 3"}</div>
                            <div>{"          Auth: disabled, Compression: disabled"}</div>
                            <div>{"         https: disabled, Cert: , Cert-Password: "}</div>
                            <div>{"          Root: /home/cecile/repos/blueprint-rs,"}</div>
                            <div>{"    TryFile404: "}</div>
                            <div>{"       Address: http://0.0.0.0:8000"}</div>
                            <div>{"    ======== [2020-09-07 20:39:46] ========"}</div>
                            <div>{"[2020-09-07 20:39:46] - 127.0.0.1 - 200 - GET /"}</div>
                            <div>{"[2020-09-07 20:39:46] - 127.0.0.1 - 200 - GET /static/blueprint.css"}</div>
                            <div>{"[2020-09-07 20:39:46] - 127.0.0.1 - 200 - GET /static/wasm.js"}</div>
                            <div>{"[2020-09-07 20:39:46] - 127.0.0.1 - 200 - GET /static/wasm_bg.wasm"}</div>
                        </pre>
                    </Collapse>
                </div>
                <div>
                    <Tree<i32>
                        tree=self.tree.clone()
                        on_collapse=Some(self.callback_expand_node.clone())
                        on_expand=Some(self.callback_expand_node.clone())
                        onclick=Some(self.callback_select_node.clone())
                    />
                </div>
            </div>
        }
    }
}
