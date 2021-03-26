use crate::Intent;
use std::fmt;
use std::iter;
use std::ops;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::Element;
use yew::prelude::*;

pub struct Slider<T>
where
    T: Clone
        + Copy
        + From<i32>
        + ops::Add<Output = T>
        + ops::Sub<Output = T>
        + ops::Mul<Output = T>
        + ops::Div<Output = T>
        + PartialOrd
        + fmt::Display
        + 'static,
{
    props: SliderProps<T>,
    mouse_move: Closure<dyn FnMut(MouseEvent)>,
    mouse_up: Closure<dyn FnMut(MouseEvent)>,
    link: ComponentLink<Self>,
    handle_ref: NodeRef,
    track_ref: NodeRef,
    tick_size: Option<T>,
    is_moving: bool,
}

trait Clamp: PartialOrd + Sized {
    fn clamp(self, min: Self, max: Self) -> Self {
        assert!(min <= max);
        if self < min {
            min
        } else if self > max {
            max
        } else {
            self
        }
    }
}

impl<T: PartialOrd> Clamp for T {}

#[derive(Clone, Properties)]
pub struct SliderProps<T: Clone> {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub vertical: bool,
    #[prop_or_default]
    pub intent: Option<Intent>,
    #[prop_or_default]
    pub onchange: Callback<T>,
    #[prop_or_default]
    pub label_values: Option<Vec<T>>,
    #[prop_or_default]
    pub label_step_size: Option<T>,
    pub label_renderer: Rc<Box<dyn Fn(T) -> String>>,
    pub value: T,
    pub step_size: T,
    pub min: T,
    pub max: T,
}

impl<T: Clone + PartialEq> PartialEq for SliderProps<T> {
    fn eq(&self, other: &Self) -> bool {
        self.class == other.class
            && self.vertical == other.vertical
            && self.intent == other.intent
            && self.onchange == other.onchange
            && self.label_values == other.label_values
            && self.label_step_size == other.label_step_size
            && self.value == other.value
            && self.step_size == other.step_size
            && self.min == other.min
            && self.max == other.max
            && Rc::ptr_eq(&self.label_renderer, &other.label_renderer)
    }
}

pub enum Msg<T> {
    StartChange,
    Change(T),
    StopChange,
    KeyDown(KeyboardEvent),
}

impl<T> Component for Slider<T>
where
    T: Clone
        + Copy
        + From<i32>
        + ops::Add<Output = T>
        + ops::Sub<Output = T>
        + ops::Mul<Output = T>
        + ops::Div<Output = T>
        + PartialOrd
        + fmt::Display
        + 'static,
{
    type Message = Msg<T>;
    type Properties = SliderProps<T>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mouse_move = {
            let link = link.clone();
            Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                link.send_message(Msg::Change(T::from(event.client_x())));
            }) as Box<dyn FnMut(_)>)
        };
        let mouse_up = {
            let link = link.clone();
            Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
                link.send_message(Msg::StopChange);
                yew::services::ConsoleService::log("mouseup")
            }) as Box<dyn FnMut(_)>)
        };
        Self {
            props,
            mouse_move,
            mouse_up,
            link,
            handle_ref: NodeRef::default(),
            track_ref: NodeRef::default(),
            tick_size: None,
            is_moving: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::StartChange => {
                let document = yew::utils::document();
                let event_target: &web_sys::EventTarget = document.as_ref();
                self.is_moving = true;
                event_target
                    .add_event_listener_with_callback(
                        "mousemove",
                        self.mouse_move.as_ref().unchecked_ref(),
                    )
                    .unwrap();
                event_target
                    .add_event_listener_with_callback(
                        "mouseup",
                        self.mouse_up.as_ref().unchecked_ref(),
                    )
                    .unwrap();
            }
            Msg::Change(value) => {
                let handle_rect = self
                    .handle_ref
                    .cast::<Element>()
                    .unwrap()
                    .get_bounding_client_rect();
                let pixel_delta =
                    value - T::from((handle_rect.left() + handle_rect.width() / 2.0) as i32);
                let value = self.props.value
                    + (pixel_delta
                        / (self
                            .tick_size
                            .expect("tick_size has been set in fn rendered()")
                            * self.props.step_size))
                        * self.props.step_size;
                let value =
                    iter::successors(Some(self.props.min), |x| Some(*x + self.props.step_size))
                        .take_while(|x| *x <= value && *x <= self.props.max)
                        .last()
                        .unwrap_or(self.props.min);
                if value != self.props.value {
                    self.props.onchange.emit(value);
                }
            }
            Msg::StopChange => {
                let document = yew::utils::document();
                let event_target: &web_sys::EventTarget = document.as_ref();
                self.is_moving = false;
                event_target
                    .remove_event_listener_with_callback(
                        "mousemove",
                        self.mouse_move.as_ref().unchecked_ref(),
                    )
                    .unwrap();
                event_target
                    .remove_event_listener_with_callback(
                        "mouseup",
                        self.mouse_up.as_ref().unchecked_ref(),
                    )
                    .unwrap();
            }
            Msg::KeyDown(event) => match event.key().as_str() {
                "ArrowDown" | "ArrowLeft" => self.props.onchange.emit(
                    (self.props.value - self.props.step_size).clamp(self.props.min, self.props.max),
                ),
                "ArrowUp" | "ArrowRight" => self.props.onchange.emit(
                    (self.props.value + self.props.step_size).clamp(self.props.min, self.props.max),
                ),
                x => yew::services::ConsoleService::log(&format!("keydown, {}", x)),
            },
        }
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
        let percentage = (T::from(100_i32) * (self.props.value - self.props.min)
            / (self.props.max - self.props.min))
            .clamp(T::from(0_i32), T::from(100_i32));
        let label_values = if let Some(value) = &self.props.label_values {
            value.clone()
        } else if let Some(step) = self.props.label_step_size {
            iter::successors(Some(self.props.min), move |x| Some(*x + step))
                .take_while(|x| *x <= self.props.max)
                .collect::<Vec<_>>()
        } else {
            vec![self.props.min, self.props.max]
        };
        let labels = label_values
            .into_iter()
            .map(|x| {
                let offset_percentage = ((x - self.props.min) * T::from(100_i32)
                    / (self.props.max - self.props.min))
                    .clamp(T::from(0_i32), T::from(100_i32));
                html! {
                    <div
                        class=classes!("bp3-slider-label")
                        style=format!("left: {}%;", offset_percentage)
                    >
                        {x}
                    </div>
                }
            })
            .collect::<Html>();

        html! {
            <div
                class=classes!(
                    "bp3-slider",
                    self.props.vertical.then(|| "bp3-vertical"),
                )
            >
                <div
                    class=classes!("bp3-slider-track")
                    ref={self.track_ref.clone()}
                >
                    <div
                        class=classes!("bp3-slider-progress")
                        style="top: 0px;"
                    >
                    </div>
                    <div
                        class=classes!("bp3-slider-progress", self.props.intent)
                        style=format!(
                            "left: 0%; right: {}%; top: 0px;",
                            (T::from(100_i32) - percentage),
                        )
                    >
                    </div>
                </div>
                <div class=classes!("bp3-slider-axis")>
                    {labels}
                </div>
                <span
                    class=classes!(
                        "bp3-slider-handle",
                        self.is_moving.then(|| "bp3-active"),
                    )
                    ref={self.handle_ref.clone()}
                    style=format!("left: calc({}% - 8px);", percentage)
                    onmousedown=self.link.callback(|_| Msg::StartChange)
                    onkeydown=self.link.callback(Msg::KeyDown)
                    tabindex=0
                >
                    <span class=classes!("bp3-slider-label")>
                        {(self.props.label_renderer)(self.props.value)}
                    </span>
                </span>
            </div>
        }
    }

    fn rendered(&mut self, _first_render: bool) {
        let track_size = T::from(self.track_ref.cast::<Element>().unwrap().client_width());
        self.tick_size = Some(track_size / (self.props.max - self.props.min));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_compiles_with_f64() {
        html! {
            <Slider<f64>
                min=-10.0
                max=10.0
                step_size=1.0
                label_step_size=2.0
                value=0.0
            />
        };
    }

    #[test]
    fn it_compiles_with_i64() {
        html! {
            <Slider<i64>
                min=-10
                max=10
                step_size=1
                label_step_size=2
                value=0
            />
        };
    }

    #[test]
    fn it_compiles_with_i32() {
        html! {
            <Slider<i32>
                min=-10
                max=10
                step_size=1
                label_step_size=2
                value=0
            />
        };
    }
}
