use std::marker::PhantomData;

use crate::{Icon, IconName};
use web_sys::HtmlSelectElement;
use yew::prelude::*;

pub struct HtmlSelect<T: Clone + PartialEq + 'static> {
    select_element: NodeRef,
    phantom: PhantomData<T>,
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

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            select_element: NodeRef::default(),
            phantom: PhantomData,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let i = if let Some(select) = msg.target_dyn_into::<HtmlSelectElement>() {
            select.selected_index()
        } else {
            unreachable!("unexpected Event: {:?}", msg);
        };
        if i >= 0 {
            let i = i as usize;
            let variant = ctx.props().options[i].0.clone();
            ctx.props().onchange.emit(variant);
        }
        false
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        if let Some(value) = ctx.props().value.as_ref() {
            if let Some(select) = self.select_element.cast::<HtmlSelectElement>() {
                if let Some(i) = ctx.props().options.iter().position(|(x, _)| x == value) {
                    select.set_selected_index(i.try_into().unwrap());
                }
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let option_children = ctx
            .props()
            .options
            .iter()
            .map(|(value, label)| {
                let selected = ctx
                    .props()
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
                    ctx.props().minimal.then(|| "bp3-minimal"),
                    ctx.props().large.then(|| "bp3-large"),
                    ctx.props().fill.then(|| "bp3-fill"),
                    ctx.props().disabled.then(|| "bp3-disabled"),
                    ctx.props().class.clone(),
                )}
            >
                <select
                    disabled={ctx.props().disabled}
                    onchange={ctx.link().callback(|x| x)}
                    title={ctx.props().title.clone()}
                    value={"".to_string()}
                    ref={self.select_element.clone()}
                >
                    {option_children}
                </select>
                <Icon icon={IconName::DoubleCaretVertical}/>
            </div>
        }
    }
}
