use web_sys::Element;
use yew::prelude::*;

use wasm_bindgen::closure::Closure;
//pub use wasm_bindgen::{self, prelude::*, JsCast};
pub struct OverflowList {
    props: OverflowListProps,
    child_ref: NodeRef,
    //on_resize: Closure<dyn FnMut(ResizeEvent)>,
}
#[derive(Debug, Clone, PartialEq, Properties)]
pub struct OverflowListProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub visible_itens: Vec<yew::virtual_dom::VNode>,
    #[prop_or_default]
    pub collapsed_items: Vec<yew::virtual_dom::VNode>,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub style: Option<String>,
    #[prop_or_default]
    pub collapse_from_start: bool,
    #[prop_or_default]
    pub min_visible_itens: usize,
}

pub enum Msg {
    Resized(f64, f64),
}

impl OverflowList {
    fn repartition(&mut self, available_width: f64) {}
}

impl Component for OverflowList {
    type Properties = OverflowListProps;
    type Message = Msg;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let mut overflow = Self {
            props,
            child_ref: NodeRef::default(),
        };

        overflow.props.visible_itens = overflow.props.children.iter().collect::<Vec<_>>();

        overflow
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn rendered(&mut self, _first_render: bool) {
        let child_width = self.child_ref.cast::<Element>().unwrap().client_width();
        yew::services::ConsoleService::log(&format!("{}", child_width).to_string());
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            let child_rect = self.child_ref.cast::<Element>().expect("no child ref");
            let available_width = child_rect.client_width() as f64;
            self.props.visible_itens = self.props.children.iter().collect::<Vec<_>>();
            if !self.props.collapse_from_start {
                self.props.visible_itens.reverse();
            }

            self.repartition(available_width);
            return true;
        }
        false
    }

    fn view(&self) -> Html {
        let overflow_wrapper = html! {
            <li>
                <span class="bp3-popover-wrapper">
                    <span class="bp3-popover-target">
                        <span class="bp3-breadcrumbs-collapsed">
                        </span>
                    </span>
                </span>
            </li>
        };
        html! {
            <>
                <ul
                    ref=self.child_ref.clone()
                    class=classes!(
                        "bp3-breadcrumbs",
                        self.props.class.clone()
                )>
                {
                    if self.props.collapse_from_start {
                        overflow_wrapper.clone()
                    } else {
                        html! ()
                    }
                }
                {
                    self.props.visible_itens.clone()
                }
                {
                    if !self.props.collapse_from_start {
                        overflow_wrapper
                    } else {
                        html! ()
                    }
                }
                </ul>
            </>
        }
    }
}
pub trait ResizeAware {
    fn on_resize(&mut self, width: f64, height: f64) -> ();
}

impl ResizeAware for OverflowList {
    fn on_resize(&mut self, width: f64, height: f64) -> () {
        self.update(Msg::Resized(width, height));
    }
}

// ------ ------
//    Extern
// ------ ------

/* #[wasm_bindgen]
extern "C" {
    fn observeElementSize(element: &web_sys::Element, callback: &wasm_bindgen::JsValue);
}

pub struct ResizeSensor<T>
where
    T: ResizeAware,
{
    element: NodeRef,
    component: T,
}

impl<T: ResizeAware> ResizeSensor<T> {
    fn observe_element(&self) {
        let element_ref = self.element.cast::<Element>().expect("no element ref");
        let callback = move |width, height| self.component.on_resize(width, height);
        let closure = Closure::wrap(Box::new(callback) as Box<dyn Fn(f64, f64)>);
        let closure_as_js_value = closure.as_ref().clone();
        closure.forget();
        unsafe {
            observeElementSize(&element_ref, &closure_as_js_value);
        }
    }
} */
