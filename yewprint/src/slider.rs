use crate::Intent;
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
    tick_size: Option<f64>,
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
    #[prop_or_default]
    pub value_label: Option<String>,
    pub value: T,
}

pub enum Msg {
    StartChange,
    Change(u32),
    StopChange,
    KeyDown(KeyboardEvent),
}

impl<T: Clone + PartialEq + 'static> Component for Slider<T> {
    type Message = Msg;
    type Properties = SliderProps<T>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mouse_move = {
            let link = link.clone();
            Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                link.send_message(Msg::Change(event.client_x() as u32));
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
        let document = yew::utils::document();
        let event_target: &web_sys::EventTarget = document.as_ref();
        let track_rect = self
            .track_ref
            .cast::<Element>()
            .unwrap()
            .get_bounding_client_rect();
        match msg {
            Msg::StartChange => {
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
            Msg::Change(position) => {
                let pixel_delta = position.saturating_sub(track_rect.left() as u32);
                let position = pixel_delta as f64 / self.tick_size.unwrap() as f64;

                let (value, _) = self
                    .props
                    .options
                    .get(position.round() as usize)
                    .unwrap_or_else(|| self.props.options.last().unwrap())
                    .clone();

                if value != self.props.value {
                    self.props.onchange.emit(value);
                };
            }
            Msg::StopChange => {
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
                "ArrowDown" | "ArrowLeft" => {
                    event.prevent_default();
                    let index = self
                        .props
                        .options
                        .iter()
                        .position(|i| i.0 == self.props.value)
                        .unwrap();
                    let index = index.saturating_sub(1);
                    let (value, _) = self.props.options[index].clone();
                    self.props.onchange.emit(value);
                }
                "ArrowUp" | "ArrowRight" => {
                    event.prevent_default();
                    let index = self
                        .props
                        .options
                        .iter()
                        .position(|i| i.0 == self.props.value)
                        .unwrap();
                    let index = index.saturating_add(1);
                    let (value, _) = self
                        .props
                        .options
                        .get(index)
                        .unwrap_or_else(|| self.props.options.last().unwrap())
                        .clone();
                    self.props.onchange.emit(value);
                }
                _ => (),
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
        let value_index = self
            .props
            .options
            .iter()
            .position(|i| i.0 == self.props.value)
            .unwrap();
        let percentage = 100 * (value_index) / (self.props.options.len() - 1);
        let labels = self
            .props
            .options
            .iter()
            .enumerate()
            .filter_map(|(i, (_, label))| {
                label.clone().map(|x| {
                    html! {
                        <div
                            class=classes!("bp3-slider-label")
                            style=format!("left: {}%;", i * 100 / (self.props.options.len() - 1))
                        >
                            {x}
                        </div>
                    }
                })
            })
            .collect::<Html>();
        let value_label = match &self.props.value_label {
            Some(value) => html! {
                <span class=classes!("bp3-slider-label")>
                    {value}
                </span>
            },
            None => html! {},
        };

        html! {
            <div
                class=classes!(
                    "bp3-slider",
                    self.props.vertical.then(|| "bp3-vertical"),
                )
                onclick=self.link.callback(
                    |event: MouseEvent| Msg::Change(event.client_x() as u32)
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
                        style=format!("left: 0%; right: {}%; top: 0px;", 100 - percentage)
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
                    {value_label}
                </span>
            </div>
        }
    }

    fn rendered(&mut self, _first_render: bool) {
        let track_size = self.track_ref.cast::<Element>().unwrap().client_width() as f64;
        self.tick_size = Some(track_size / (self.props.options.len() - 1) as f64);
    }
}
