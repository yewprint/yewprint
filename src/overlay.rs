use crate::Portal;
use gloo::timers::callback::Timeout;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement, NodeList};
use yew::prelude::*;

// TODO:
//  -  use without <Portal/>
//  -  enforce focus inside the overlay
//  -  CSS transitions

#[derive(Debug)]
pub struct Overlay {
    start_focus_trap: NodeRef,
    content_ref: NodeRef,
    callback_timeout: Option<Timeout>,
    initial_open: bool,
    #[allow(dead_code)]
    document_focus_closure: Closure<dyn FnMut(FocusEvent)>,
    last_active_element: Option<Element>,
    focus_first_element_closure: Closure<dyn FnMut()>,
    focus_last_element_closure: Closure<dyn FnMut()>,
}

#[derive(Debug, PartialEq, Properties)]
pub struct OverlayProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: Option<AttrValue>,
    #[prop_or_default]
    pub open: bool,
    #[prop_or(true)]
    pub backdrop: bool,
    #[prop_or_default]
    pub onclose: Callback<()>,
    #[prop_or_default]
    pub children: Children,
}

pub enum Msg {
    OnKeyDown(KeyboardEvent),
    OnClick(MouseEvent),
    FocusFirstElement,
    FocusLastElement,
    Close,
}

impl Component for Overlay {
    type Properties = OverlayProps;
    type Message = Msg;

    fn create(ctx: &Context<Self>) -> Self {
        let content_ref = NodeRef::default();

        let document_focus_closure = {
            let callback = ctx.link().callback(|_| Msg::FocusFirstElement);
            let content_ref = content_ref.clone();
            Closure::new(Box::new(move |_event| {
                let active_element_in_content = content_ref
                    .cast::<Element>()
                    .map(|content| {
                        content.contains(gloo::utils::document().active_element().as_deref())
                    })
                    .unwrap_or(false);
                if !active_element_in_content {
                    callback.emit(());
                }
            }) as Box<dyn FnMut(_)>)
        };
        gloo::utils::document().set_onfocus(Some(document_focus_closure.as_ref().unchecked_ref()));

        let focus_first_element_closure = {
            let content_ref = content_ref.clone();
            Closure::new(Box::new(move || {
                if let Some(element) = get_focusable_elements(&content_ref)
                    .and_then(|x| x.item(0))
                    .and_then(|x| x.dyn_into::<HtmlElement>().ok())
                {
                    element.focus().unwrap();
                }
            }) as Box<dyn FnMut()>)
        };

        let focus_last_element_closure = {
            let content_ref = content_ref.clone();
            Closure::new(Box::new(move || {
                if let Some(element) = get_focusable_elements(&content_ref)
                    .and_then(|x| x.item(x.length() - 1))
                    .and_then(|x| x.dyn_into::<HtmlElement>().ok())
                {
                    element.focus().unwrap();
                }
            }) as Box<dyn FnMut()>)
        };

        Self {
            start_focus_trap: NodeRef::default(),
            content_ref,
            callback_timeout: None,
            initial_open: false,
            document_focus_closure,
            last_active_element: None,
            focus_first_element_closure,
            focus_last_element_closure,
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        let new_props = ctx.props();

        if new_props.open != old_props.open {
            self.initial_open = false;
        }

        if new_props.open {
            self.last_active_element = gloo::utils::document().active_element();
            gloo::utils::body()
                .class_list()
                .add_1("bp3-overlay-open")
                .unwrap();
        } else {
            gloo::utils::body()
                .class_list()
                .remove_1("bp3-overlay-open")
                .unwrap();
            if let Some(element) = self
                .last_active_element
                .take()
                .and_then(|x| x.dyn_into::<HtmlElement>().ok())
            {
                let _ = element.focus();
            }
        }

        true
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::OnKeyDown(event) => {
                if event.key() == "Escape" {
                    ctx.props().onclose.emit(());
                }
                false
            }
            Msg::OnClick(_event) => {
                if self.callback_timeout.is_none() {
                    let callback = ctx.link().callback(|_| Msg::Close);
                    self.callback_timeout
                        .replace(Timeout::new(0, move || callback.emit(())));
                } else {
                    self.callback_timeout.take();
                }
                false
            }
            Msg::FocusFirstElement => {
                // always delay focus manipulation to just before repaint to prevent scroll jumping
                gloo::utils::window()
                    .request_animation_frame(
                        self.focus_first_element_closure.as_ref().unchecked_ref(),
                    )
                    .unwrap();
                false
            }
            Msg::FocusLastElement => {
                // always delay focus manipulation to just before repaint to prevent scroll jumping
                gloo::utils::window()
                    .request_animation_frame(
                        self.focus_last_element_closure.as_ref().unchecked_ref(),
                    )
                    .unwrap();
                false
            }
            Msg::Close => {
                self.callback_timeout.take();
                ctx.props().onclose.emit(());
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Self::Properties {
            class,
            style,
            open,
            backdrop,
            onclose: _,
            children,
        } = ctx.props();

        let backdrop = backdrop.then(|| {
            html! {
                <div class="bp3-overlay-backdrop" />
            }
        });
        let inner = open.then(|| {
            html! {
                <>
                <div
                    class="bp3-overlay-start-focus-trap"
                    ref={self.start_focus_trap.clone()}
                    tabindex=0
                    // NOTE: I am not 100% sure this is correct. In Blueprint they capture the
                    //       Shift+Tab combination but it looks like it's more for historic
                    //       reason... well, it seems to work on current Chrome and Firefox so...
                    onfocus={ctx.link().callback(|_| Msg::FocusLastElement)}
                />
                {backdrop}
                <div
                    class={classes!("bp3-overlay-content", class.clone())}
                    {style}
                    ref={self.content_ref.clone()}
                    onclick={ctx.link().callback(Msg::OnClick)}
                >
                    {for children.iter()}
                </div>
                <div
                    class="bp3-overlay-end-focus-trap"
                    ref={self.start_focus_trap.clone()}
                    tabindex=0
                    onfocus={ctx.link().callback(|_| Msg::FocusFirstElement)}
                />
                </>
            }
        });

        html! {
            <Portal>
                <div
                    class={classes!(
                        "bp3-overlay",
                        "bp3-overlay-scroll-container",
                        open.then_some("bp3-overlay-open"),
                    )}
                    aria-live="polite"
                    onkeydown={ctx.link().callback(Msg::OnKeyDown)}
                    onclick={ctx.link().callback(Msg::OnClick)}
                >
                    {inner}
                </div>
            </Portal>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        let Self::Properties { open, .. } = ctx.props();

        if *open && !self.initial_open {
            self.initial_open = true;
            ctx.link().send_message(Msg::FocusFirstElement);
        }
    }
}

impl Drop for Overlay {
    fn drop(&mut self) {
        gloo::utils::document().set_onfocus(None);
    }
}

fn get_focusable_elements(node_ref: &NodeRef) -> Option<NodeList> {
    node_ref.cast::<Element>().and_then(|element| {
        element
            .query_selector_all(
                "a[href]:not([tabindex=\"-1\"]),button:not([disabled]):not([tabindex=\"-1\"]),\
                details:not([tabindex=\"-1\"]),input:not([disabled]):not([tabindex=\"-1\"]),\
                select:not([disabled]):not([tabindex=\"-1\"]),\
                textarea:not([disabled]):not([tabindex=\"-1\"]),[tabindex]:not([tabindex=\"-1\"])",
            )
            .ok()
    })
}
