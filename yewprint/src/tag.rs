use crate::{ConditionalClass, Icon, Intent};
use yew::html::ChildrenRenderer;
use yew::prelude::*;
use yew::virtual_dom::VText;

pub struct Tag {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub children: ChildrenRenderer<VText>,
    #[prop_or_default]
    // FIXME Not clear that this field has any effect withut `interactive` on.
    pub active: ConditionalClass,
    #[prop_or_default]
    pub fill: ConditionalClass,
    #[prop_or_default]
    pub icon: Option<Icon>,
    #[prop_or_default]
    pub intent: Option<Intent>,
    #[prop_or_default]
    pub interactive: ConditionalClass,
    #[prop_or_default]
    pub large: ConditionalClass,
    #[prop_or_default]
    pub minimal: ConditionalClass,
    #[prop_or_default]
    // FIXME Should make sense once `Text` is implemented.
    pub multiline: ConditionalClass,
    #[prop_or_default]
    //onClick,
    //onRemove,
    #[prop_or_default]
    pub right_icon: Option<Icon>,
    #[prop_or_default]
    pub round: ConditionalClass,
}

impl Component for Tag {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Tag { props }
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
        // FIXME use Text when implemented
        html! {
            <span class=(
                "bp3-tag",
                self.props.intent,
                self.props.active.map_some("bp3-active"),
                self.props.fill.map_some("bp3-fill"),
                self.props.interactive.map_some("bp3-interactive"),
                self.props.large.map_some("bp3-large"),
                self.props.minimal.map_some("bp3-minimal"),
                self.props.multiline.map_some("bp3-multiline"),
                self.props.round.map_some("bp3-round"),
                )
            >
                {self.props.children.clone()}
            </span>
        }
    }
}
