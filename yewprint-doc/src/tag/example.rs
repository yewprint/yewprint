use yew::prelude::*;
use yewprint::{Tag,Icon,Intent};

pub struct Example {
    props: ExampleProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ExampleProps {
    pub active: bool,
    pub fill: bool,
    pub icon: Option<Icon>,
    pub intent: Option<Intent>,
    pub interactive: bool,
    pub large: bool,
    pub minimal: bool,
    pub multiline: bool,
    //onClick,
    //onRemove,
    pub right_icon: Option<Icon>,
    pub round: bool,
}

impl Component for Example {
    type Message = ();
    type Properties = ExampleProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Example { props }
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
            <>
                <Tag
                    active=self.props.active
                    fill=self.props.fill
                    icon=self.props.icon.clone()
                    intent=self.props.intent
                    interactive=self.props.interactive
                    large=self.props.large
                    minimal=self.props.minimal
                    multiline=self.props.multiline
                    right_icon=self.props.right_icon.clone()
                    round=self.props.round
                >
                    {"Landscape"}
                </Tag>
                <Tag
                    active=self.props.active
                    fill=self.props.fill
                    icon=self.props.icon.clone()
                    intent=self.props.intent
                    interactive=self.props.interactive
                    large=self.props.large
                    minimal=self.props.minimal
                    multiline=self.props.multiline
                    right_icon=self.props.right_icon.clone()
                    round=self.props.round
                >
                    {"Bird"}
                </Tag>
                <Tag
                    active=self.props.active
                    fill=self.props.fill
                    icon=self.props.icon.clone()
                    intent=self.props.intent
                    interactive=self.props.interactive
                    large=self.props.large
                    minimal=self.props.minimal
                    multiline=self.props.multiline
                    right_icon=self.props.right_icon.clone()
                    round=self.props.round
                >
                    {"City"}
                </Tag>
            </>
        }
    }
}