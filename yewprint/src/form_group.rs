use crate::{ConditionalClass, Intent};
use yew::prelude::*;

pub struct FormGroup {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub disabled: ConditionalClass,
    #[prop_or_default]
    pub helper_text: String,
    #[prop_or_default]
    pub inline: ConditionalClass,
    #[prop_or_default]
    pub intent: Option<Intent>,
    #[prop_or_default]
    pub label: String,
    #[prop_or_default]
    pub label_for: String,
    #[prop_or_default]
    pub label_info: String,
    #[prop_or_default]
    pub children: Children,
}

impl Component for FormGroup {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        FormGroup { props }
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
            <div
                class=(
                    "bp3-form-group",
                    self.props.disabled.map_some("bp3-disabled"),
                    self.props.inline.map_some("bp3-inline"),
                    self.props.intent.map(|intent| match intent {
                        Intent::Primary => "bp3-intent-primary",
                        Intent::Success => "bp3-intent-success",
                        Intent::Warning => "bp3-intent-warning",
                        Intent::Danger => "bp3-intent-danger",
                    })
                )
            >
                <label class="bp3-label" for={self.props.label_for.clone()}>
                    {self.props.label.clone()}
                    <span class="bp3-text-muted">{self.props.label_info.clone()}</span>
                </label>
                <div class="bp3-form-content">
                    {self.props.children.clone()}
                    <div class="bp3-form-helper-text">{self.props.helper_text.clone()}</div>
                </div>
            </div>
        }
    }
}
