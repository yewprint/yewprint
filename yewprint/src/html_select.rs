use crate::{Icon, IconName};
use yew::prelude::*;

pub struct HtmlSelect<T: Clone + PartialEq + 'static> {
    props: HtmlSelectProps<T>,
    link: html::Scope<Self>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct HtmlSelectProps<T: Clone + PartialEq + 'static> {
    #[prop_or_default]
    pub fill: bool,
    #[prop_or_default]
    pub minimal: bool,
    #[prop_or_default]
    pub large: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub icon: Option<IconName>,
    #[prop_or_default]
    pub title: Option<String>,
    #[prop_or_default]
    pub onchange: Callback<T>,
    #[prop_or_default]
    pub value: Option<T>,
    pub options: Vec<(T, String)>,
    #[prop_or_default]
    pub class: Classes,
}

impl<T: Clone + PartialEq + 'static> Component for HtmlSelect<T> {
    type Message = Event;
    type Properties = HtmlSelectProps<T>;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            props: *ctx.props(),
            link: *ctx.link(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        let i = if let Event::Select(select) = msg {
            select.selected_index()
        } else {
            unreachable!("unexpected ChangeData variant: {:?}", msg);
        };
        if i >= 0 {
            let i = i as usize;
            let variant = self.props.options[i].0.clone();
            self.props.onchange.emit(variant);
        }
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let option_children = self
            .props
            .options
            .iter()
            .map(|(value, label)| {
                let selected = self
                    .props
                    .value
                    .as_ref()
                    .map(|x| value == x)
                    .unwrap_or_default();

                html! {
                    <option selected={selected}>
                        {label}
                    </option>
                }
            })
            .collect::<Html>();

        html! {
            <div
                class={classes!(
                    "bp3-html-select",
                    self.props.minimal.then(|| "bp3-minimal"),
                    self.props.large.then(|| "bp3-large"),
                    self.props.fill.then(|| "bp3-fill"),
                    self.props.disabled.then(|| "bp3-disabled"),
                    self.props.class.clone(),
                )}
            >
                <select
                    disabled={self.props.disabled}
                    onchange={self.link.callback(|x| x)}
                    title={self.props.title.clone()}
                    value={"".to_string()}
                >
                    {option_children}
                </select>
                <Icon icon={IconName::DoubleCaretVertical}/>
            </div>
        }
    }
}
