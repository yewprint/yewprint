use yew::prelude::*;
use yewprint::{IconName, Intent, Tag};

pub struct Example {
    tags: Vec<String>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ExampleProps {
    pub initial_tags: Vec<String>,
    pub active: bool,
    pub fill: bool,
    pub icon: bool,
    pub intent: Option<Intent>,
    pub interactive: bool,
    pub large: bool,
    pub minimal: bool,
    pub multiline: bool,
    pub removable: bool,
    pub right_icon: bool,
    pub round: bool,
    pub reset_tags: u64,
}

pub enum ExampleMsg {
    Remove(String),
    Click,
}

impl Component for Example {
    type Message = ExampleMsg;
    type Properties = ExampleProps;

    fn create(ctx: &Context<Self>) -> Self {
        let tags = ctx.props().initial_tags.clone();
        Example { tags }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ExampleMsg::Remove(label) => {
                self.tags = self
                    .tags
                    .clone()
                    .into_iter()
                    .filter(|l| *l != label)
                    .collect()
            }
            ExampleMsg::Click => (),
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        if ctx.props().reset_tags != ctx.props().reset_tags {
            self.tags = ctx.props().initial_tags.clone();
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        self.tags
            .iter()
            .map(|label| {
                let remove = {
                    let label = label.clone();
                    ctx.props().removable.then(|| {
                        ctx.link()
                            .callback(move |_| ExampleMsg::Remove(label.clone()))
                    })
                };
                html! {
                    <Tag
                        active={ctx.props().active}
                        fill={ctx.props().fill}
                        icon={ctx.props().icon.then(|| IconName::Print)}
                        intent={ctx.props().intent}
                        interactive={ctx.props().interactive}
                        large={ctx.props().large}
                        minimal={ctx.props().minimal}
                        multiline={ctx.props().multiline}
                        right_icon={ctx.props().right_icon.then(|| IconName::Star)}
                        round={ctx.props().round}
                        onremove={remove}
                        onclick={ctx.link().callback(|_| ExampleMsg::Click)}
                    >
                        {label.clone()}
                    </Tag>
                }
            })
            .collect::<Html>()
    }
}
