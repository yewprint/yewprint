use yew::prelude::*;
use yewprint::{IconName, Intent, Tag};

pub struct Example {
    props: ExampleProps,
    link: &html::Scope<Self>,
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
        Example { props, link, tags }
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
        if self.props != props {
            if self.props.reset_tags != ctx.props().reset_tags {
                self.tags = ctx.props().initial_tags.clone();
            }
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        self.tags
            .iter()
            .map(|label| {
                let remove = {
                    let label = label.clone();
                    self.props.removable.then(|| {
                        self.link
                            .callback(move |_| ExampleMsg::Remove(label.clone()))
                    })
                };
                html! {
                    <Tag
                        active={self.props.active}
                        fill={self.props.fill}
                        icon={self.props.icon.then(|| IconName::Print)}
                        intent={self.props.intent}
                        interactive={self.props.interactive}
                        large={self.props.large}
                        minimal={self.props.minimal}
                        multiline={self.props.multiline}
                        right_icon={self.props.right_icon.then(|| IconName::Star)}
                        round={self.props.round}
                        onremove={remove}
                        onclick={self.link.callback(|_| ExampleMsg::Click)}
                    >
                        {label.clone()}
                    </Tag>
                }
            })
            .collect::<Html>()
    }
}
