use crate::Intent;
use std::fmt;
use std::iter;
use std::ops;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::Element;
use yew::prelude::*;

pub struct Slider<T: Clone + PartialEq + 'static> {
    props: SliderProps<T>,
    mouse_move: Closure<dyn FnMut(MouseEvent)>,
    mouse_up: Closure<dyn FnMut(MouseEvent)>,
    link: ComponentLink<Self>,
    handle_ref: NodeRef,
    track_ref: NodeRef,
    tick_size: Option<i32>,
    is_moving: bool,
}

#[derive(Clone, PartialEq, Properties)]
pub struct SliderProps<T: Clone + PartialEq + 'static> {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub vertical: bool,
    #[prop_or_default]
    pub intent: Option<Intent>,
    #[prop_or_default]
    pub onchange: Callback<T>,
    #[prop_or_default]
    pub options: Vec<(T, Option<String>)>,
    pub value: T,
}

pub enum Msg<T> {
    StartChange,
    Change(T),
    StopChange,
    //KeyDown(KeyboardEvent),
}

impl<T: Clone + PartialEq + 'static> Component for Slider<T> {
    type Message = Msg<T>;
    type Properties = SliderProps<T>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mouse_move = {
            let link = link.clone();
            Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                link.send_message(Msg::Change(event.client_x()));
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
                let pixel_delta = value - (handle_rect.left() + handle_rect.width() / 2.0) as i32;
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
            } /*
              Msg::KeyDown(event) => match event.key().as_str() {
                  "ArrowDown" | "ArrowLeft" => self.props.onchange.emit(
                      (self.props.value - self.props.step_size).clamp(self.props.min, self.props.max),
                  ),
                  "ArrowUp" | "ArrowRight" => self.props.onchange.emit(
                      (self.props.value + self.props.step_size).clamp(self.props.min, self.props.max),
                  ),
                  x => yew::services::ConsoleService::log(&format!("keydown, {}", x)),
              },
              */
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
        let labels = self
            .props
            .options
            .into_iter()
            .map(|(x, y)| {
                let offset_percentage = ((x - self.props.min) * T::from(100_i32)
                    / (self.props.max - self.props.min))
                    .clamp(0_i32, 100_i32);
                html! {
                    <div
                        class=classes!("bp3-slider-label")
                        style=format!("left: {}%;", offset_percentage)
                    >
                        {y}
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
                    // onkeydown=self.link.callback(Msg::KeyDown)
                    tabindex=0
                >
                    <span class=classes!("bp3-slider-label")>
                        {self.props.value}
                    </span>
                </span>
            </div>
        }
    }

    fn rendered(&mut self, _first_render: bool) {
        let track_size = self.track_ref.cast::<Element>().unwrap().client_width();
        self.tick_size = Some(track_size / (self.props.max - self.props.min));
    }
}
