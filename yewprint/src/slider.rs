use crate::Intent;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use yew::prelude::*;

pub struct Slider {
    props: SliderProps,
    mouse_move: Closure<dyn FnMut(MouseEvent)>,
    mouse_up: Closure<dyn FnMut(MouseEvent)>,
    link: ComponentLink<Self>,
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
    StopChange,
}

impl Component for Slider {
    type Message = Msg;
    type Properties = SliderProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mouse_move = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            yew::services::ConsoleService::log("mousemove")
        }) as Box<dyn FnMut(_)>);
        let mouse_up = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            yew::services::ConsoleService::log("mouseup")
        }) as Box<dyn FnMut(_)>);
        Self {
            props,
            mouse_move,
            mouse_up,
            link,
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
            }
            Msg::StopChange => {
                let document = yew::utils::document();
                let event_target: &web_sys::EventTarget = document.as_ref();
                event_target
                    .add_event_listener_with_callback(
                        "mouseup",
                        self.mouse_up.as_ref().unchecked_ref(),
                    )
                    .unwrap();
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
                <div class=classes!("bp3-slider-track")>
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
                    style=format!("left: {}%", percentage)
                    onmousedown=self.link.callback(|_| Msg::StartChange)
                    onmouseup=self.link.callback(|_| Msg::StopChange)
                >
                    <span class=classes!("bp3-slider-label")>
                        {self.props.value}
                    </span>
                </span>
            </div>
        }
    }
}
