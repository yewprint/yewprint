use yew::prelude::*;

pub struct Tree {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {}

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
                    <TreeNode/>
                </ul>
            </div>
        }
    }
}

pub struct TreeNode {
    props: TreeNodeProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct TreeNodeProps {}

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
                    {"content"}
                </div>
                // missing <Collapse/>
            </li>
        }
    }
}
