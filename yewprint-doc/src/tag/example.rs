use yew::prelude::*;
use yewprint::{Tag, IconName,Intent, ConditionalClass};

pub struct Example {
    props: ExampleProps,
    link: ComponentLink<Self>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ExampleProps {
    pub initial_tags: Vec<String>,
    pub tags: Vec<String>,
    pub active: bool,
    pub fill: bool,
    pub icon: ConditionalClass,
    pub intent: Option<Intent>,
    pub interactive: bool,
    pub large: bool,
    pub minimal: bool,
    pub multiline: bool,
    //onClick,
    pub removable: ConditionalClass,
    pub right_icon: ConditionalClass,
    pub round: bool,
}

pub enum ExampleMsg {
    Remove(String),
}


impl Component for Example {
    type Message = ExampleMsg;
    type Properties = ExampleProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Example { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let ExampleMsg::Remove(label) = msg;
        self.props.tags = self.props.tags.clone().into_iter().filter(|l| *l != label).collect();
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
        let tags = self.props.tags.clone().into_iter().map(|label| {
            let label2 = label.clone();
            let remove = self.props.removable.map_some(self.link.callback(move |_| ExampleMsg::Remove(label2.clone()))); 
            html! {
                <Tag
                    active=self.props.active
                    fill=self.props.fill
                    icon=self.props.icon.map_some(IconName::Print)
                    intent=self.props.intent
                    interactive=self.props.interactive
                    large=self.props.large
                    minimal=self.props.minimal
                    multiline=self.props.multiline
                    right_icon=self.props.right_icon.map_some(IconName::Star)
                    round=self.props.round
                    onremove=remove
                >
                    {yew::virtual_dom::vtext::VText::new(label.clone())}
                </Tag>
            }
        });

        html!{
            <>
                {tags.collect::<Html>()}
            </>
        }
    }
}
