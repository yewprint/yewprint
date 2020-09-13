use crate::collapse::Collapse;
use crate::icon::{Icon, IconName};
use yew::prelude::*;

pub struct Tree {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    is_expanded: bool,
}

impl Component for Tree {
    type Message = ();
    type Properties = Props;

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
        html! {
            <div class="bp3-tree">
                <ul class="bp3-tree-node-list">
                    <TreeNode is_expanded=self.props.is_expanded has_caret=true>
                        {"Example Stuff"}
                    </TreeNode>
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
    pub is_expanded: bool,
    #[prop_or_default]
    pub has_caret: bool,
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
                            html! {
                                <Icon class="bp3-tree-node-caret" icon=IconName::ChevronRight />
                            }
                        } else {
                            html! {
                                <span class="bp3-tree-node-caret-none" />
                            }
                        }
                    }
                    {"content"}
                </div>
                <Collapse is_open=self.props.is_expanded>
                    {self.props.children.clone()}
                </Collapse>
            </li>
        }
    }
}
