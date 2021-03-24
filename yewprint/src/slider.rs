use crate::Intent;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::Element;
use yew::prelude::*;

pub struct Slider {
    props: SliderProps,
    mouse_move: Closure<dyn FnMut(MouseEvent)>,
    mouse_up: Closure<dyn FnMut(MouseEvent)>,
    link: ComponentLink<Self>,
    handle_ref: NodeRef,
    track_ref: NodeRef,
    tick_size: f64,
}

#[derive(Clone, PartialEq, Properties)]
pub struct SliderProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub vertical: bool,
    #[prop_or_default]
    pub intent: Option<Intent>,
    pub value: i32,
    #[prop_or_default]
    pub step_size: i32,
    #[prop_or_default]
    pub min: i32,
    pub max: i32,
    #[prop_or_default]
    pub onchange: Callback<i32>,
}

pub enum Msg {
    StartChange,
    Change(i32),
    StopChange,
    KeyDown(KeyboardEvent),
}

impl Component for Slider {
    type Message = Msg;
    type Properties = SliderProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mouse_move = {
            let link = link.clone();
            Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                link.send_message(Msg::Change(event.client_x()));
            }) as Box<dyn FnMut(_)>)
        };
        let mouse_up = {
            let link = link.clone();
            Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
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
            tick_size: 0.0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::StartChange => {
                let document = yew::utils::document();
                let event_target: &web_sys::EventTarget = document.as_ref();
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
                yew::services::ConsoleService::log(&format!("mousemove, {}", value));
                let handle_rect = self
                    .handle_ref
                    .cast::<Element>()
                    .unwrap()
                    .get_bounding_client_rect();
                let pixel_delta = value as f64 - (handle_rect.left() + handle_rect.width() / 2.0);
                let value = if pixel_delta.is_nan() {
                    self.props.value as f64
                } else {
                    self.props.value as f64
                        + (pixel_delta / (self.tick_size as f64 * self.props.step_size as f64))
                            .round()
                            * self.props.step_size as f64
                };
                let value = (value as i32).clamp(self.props.min, self.props.max);
                if value != self.props.value {
                    self.props.onchange.emit(value);
                }
            }
            Msg::StopChange => {
                let document = yew::utils::document();
                let event_target: &web_sys::EventTarget = document.as_ref();
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
            Msg::KeyDown(event) => {
                yew::services::ConsoleService::log(&format!("keydown, {}", event.key()));
                let key = event.key();
                if key == "ArrowDown" || key == "ArrowLeft" {
                    self.props
                        .onchange
                        .emit(self.props.value - self.props.step_size);
                } else if key == "ArrowUp" || key == "ArrowRight" {
                    self.props
                        .onchange
                        .emit(self.props.value + self.props.step_size);
                }
            }
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
        let percentage = (100 * self.props.value / self.props.max).clamp(0, 100);

        html! {
            <div
                class=classes!(
                    "bp3-slider",
                    self.props.vertical.then(|| "bp3-vertical"),
                    self.props.intent,
                )
            >
                <div
                    class=classes!("bp3-slider-track")
                    ref={self.track_ref.clone()}
                >
                    <div class=classes!("bp3-slider-progress")>
                    </div>
                </div>
                <div class=classes!("bp3-slider-axis")>
                    <div
                        class=classes!("bp3-slider-label")
                        style="left: 0%;"
                    >
                        {self.props.min}
                    </div>
                    <div
                        class=classes!("bp3-slider-label")
                        style="left: 100%;"
                    >
                        {self.props.max}
                    </div>
                </div>
                <span
                    class=classes!("bp3-slider-handle")
                    ref={self.handle_ref.clone()}
                    style=format!("left: {}%", percentage)
                    onmousedown=self.link.callback(|_| Msg::StartChange)
                    onkeydown=self.link.callback(|event| Msg::KeyDown(event))
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
        let tick_size_ratio = 1.0 / (self.props.max as f64 - self.props.min as f64);
        self.tick_size = track_size as f64 * tick_size_ratio;
    }
}
