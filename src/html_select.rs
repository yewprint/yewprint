use crate::Icon;
use implicit_clone::{unsync::IArray, ImplicitClone};
use web_sys::HtmlSelectElement;
use yew::prelude::*;

#[derive(Debug)]
pub struct HtmlSelect<T: ImplicitClone + PartialEq + 'static> {
    select_element: NodeRef,
    cb: HtmlSelectMessageCallbacks<Self>,
    phantom: std::marker::PhantomData<T>,
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

#[derive(Debug, yew_callbacks::Callbacks)]
pub enum HtmlSelectMessage {
    OnChange(Event),
    // NOTE: the component becomes tainted when a value is selected because there is no way to
    //       prevent the selection of the option in an <input> HTML component. This means the
    //       actual value and the visible value might differ.
    Untaint,
}

impl<T: ImplicitClone + PartialEq + 'static> Component for HtmlSelect<T> {
    type Message = HtmlSelectMessage;
    type Properties = HtmlSelectProps<T>;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            select_element: NodeRef::default(),
            cb: ctx.link().into(),
            phantom: Default::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            HtmlSelectMessage::OnChange(event) => {
                let i = if let Some(select) = event.target_dyn_into::<HtmlSelectElement>() {
                    select.selected_index()
                } else {
                    unreachable!("unexpected Event: {:?}", event);
                };
                if i >= 0 {
                    let i = i as usize;
                    let variant = ctx.props().options[i].0.clone();
                    ctx.props().onchange.emit(variant);
                    // NOTE: register option selection update for later even if the parent
                    //       component is not going to re-render
                    ctx.link().send_message(HtmlSelectMessage::Untaint);
                }
                false
            }
            HtmlSelectMessage::Untaint => {
                self.select_option(ctx);
                false
            }
        }
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
                    onchange={self.cb.on_change()}
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
        // NOTE: ensure correct option is selected
        self.select_option(ctx);
    }
}

impl<T: ImplicitClone + PartialEq + 'static> HtmlSelect<T> {
    fn select_option(&self, ctx: &Context<Self>) {
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
