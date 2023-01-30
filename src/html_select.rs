use implicit_clone::{unsync::IArray, ImplicitClone};
use std::marker::PhantomData;

use crate::Icon;
use web_sys::HtmlSelectElement;
use yew::prelude::*;

#[derive(Debug)]
pub struct HtmlSelect<T: Clone + PartialEq + 'static> {
    select_element: NodeRef,
    update_selected: bool,
    phantom: PhantomData<T>,
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct HtmlSelectProps<T: ImplicitClone + PartialEq + 'static> {
    #[prop_or_default]
    pub fill: bool,
    #[prop_or_default]
    pub minimal: bool,
    #[prop_or_default]
    pub large: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub title: Option<AttrValue>,
    #[prop_or_default]
    pub onchange: Callback<T>,
    #[prop_or_default]
    pub value: Option<T>,
    pub options: IArray<(T, AttrValue)>,
    #[prop_or_default]
    pub class: Classes,
}

impl<T: ImplicitClone + PartialEq + 'static> Component for HtmlSelect<T> {
    type Message = Event;
    type Properties = HtmlSelectProps<T>;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            select_element: NodeRef::default(),
            update_selected: false,
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

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        self.update_selected = true;
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Self::Properties {
            fill,
            minimal,
            large,
            disabled,
            title,
            onchange: _,
            value,
            options,
            class,
        } = &ctx.props();

        let option_children = options
            .iter()
            .map(|(this_value, label)| {
                let selected = value.as_ref().map(|x| &this_value == x).unwrap_or_default();

                html! {
                    <option {selected}>
                        {label}
                    </option>
                }
            })
            .collect::<Html>();

        html! {
            <div
                class={classes!(
                    "bp3-html-select",
                    minimal.then_some("bp3-minimal"),
                    large.then_some("bp3-large"),
                    fill.then_some("bp3-fill"),
                    disabled.then_some("bp3-disabled"),
                    class.clone(),
                )}
            >
                <select
                    value={String::new()}
                    disabled={*disabled}
                    onchange={ctx.link().callback(|x| x)}
                    {title}
                    ref={self.select_element.clone()}
                >
                    {option_children}
                </select>
                <Icon icon={Icon::DoubleCaretVertical}/>
            </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        if self.update_selected {
            self.update_selected = false;
            if let Some(value) = ctx.props().value.as_ref() {
                if let Some(select) = self.select_element.cast::<HtmlSelectElement>() {
                    if let Some(i) = ctx.props().options.iter().position(|(x, _)| &x == value) {
                        if let Ok(i) = i.try_into() {
                            if select.selected_index() != i {
                                select.set_selected_index(i);
                            }
                        }
                    }
                }
            }
        }
    }
}
