use crate::{ConditionalClass, Icon, IconName};
use yew::prelude::*;

pub struct HtmlSelect {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub fill: ConditionalClass,
    #[prop_or_default]
    pub minimal: ConditionalClass,
    #[prop_or_default]
    pub large: ConditionalClass,
    #[prop_or_default]
    pub disabled: ConditionalClass,
    #[prop_or_default]
    pub icon: Option<IconName>,
    #[prop_or_default]
    pub title: Option<String>,
    #[prop_or_default]
    pub onchange: Callback<ChangeData>,
    #[prop_or_default]
    pub options: Vec<(String, String)>,
    #[prop_or_default]
    pub value: String,
}

impl Component for HtmlSelect {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        HtmlSelect {props}
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
        let option_children = self.props.options
            .iter()
            .map(|(value, label)| html!(<option value=value>{label}</option>))
            .collect::<Html>();

        html! {
            <div
                class=(
                    "bp3-html-select",
                    self.props.minimal.map_some("bp3-minimal"),
                    self.props.large.map_some("bp3-large"),
                    self.props.fill.map_some("bp3-fill"),
                    self.props.disabled.map_some("bp3-disabled"),
                )
            >
                <select
                    disabled=*self.props.disabled
                    onchange={self.props.onchange.clone()}
                >
                    {option_children}
                </select>
                <Icon icon=IconName::DoubleCaretVertical/>
            </div>
        }
    }
}