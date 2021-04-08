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
    is_moving: bool,
    focus_handle: bool,
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
    pub value_label: Option<String>,
    pub onchange: Callback<T>,
    pub options: Vec<(T, Option<String>)>,
    pub value: T,
}

pub enum Msg {
    StartChange,
    Mouse(MouseEvent),
    StopChange,
    Keyboard(KeyboardEvent),
}

impl<T: Clone + PartialEq + 'static> Component for Slider<T> {
    type Message = Msg;
    type Properties = SliderProps<T>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mouse_move = {
            let link = link.clone();
            Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                link.send_message(Msg::Mouse(event));
            }) as Box<dyn FnMut(_)>)
        };
        let mouse_up = {
            let link = link.clone();
            Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
                link.send_message(Msg::StopChange);
            }) as Box<dyn FnMut(_)>)
        };
        Self {
            props,
            mouse_move,
            mouse_up,
            link,
            handle_ref: NodeRef::default(),
            track_ref: NodeRef::default(),
            is_moving: false,
            focus_handle: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::StartChange if self.props.options.len() > 1 => {
                let document = yew::utils::document();
                let event_target: &web_sys::EventTarget = document.as_ref();
                self.is_moving = true;
                event_target
                    .add_event_listener_with_callback(
                        "mousemove",
                        self.mouse_move.as_ref().unchecked_ref(),
                    )
                    .expect("No event listener to add");
                event_target
                    .add_event_listener_with_callback(
                        "mouseup",
                        self.mouse_up.as_ref().unchecked_ref(),
                    )
                    .expect("No event listener to add");

                true
            }
            Msg::StartChange => false,
            Msg::Mouse(event) if self.props.options.len() > 1 => {
                let track_rect = self.track_ref.cast::<Element>().expect("no track ref");
                let tick_size = (track_rect.client_width() as f64)
                    / self.props.options.len().saturating_sub(1) as f64;
                let pixel_delta = (event.client_x() as u32)
                    .saturating_sub(track_rect.get_bounding_client_rect().left() as u32);

                let position = (pixel_delta as f64 / tick_size).round() as usize;

                let (value, _) = self
                    .props
                    .options
                    .get(position)
                    .unwrap_or_else(|| self.props.options.last().expect("No value in the vec"));

                if *value != self.props.value {
                    self.props.onchange.emit(value.clone());
                }

                true
            }
            Msg::Mouse(_) => false,
            Msg::StopChange => {
                let document = yew::utils::document();
                let event_target: &web_sys::EventTarget = document.as_ref();
                self.is_moving = false;
                event_target
                    .remove_event_listener_with_callback(
                        "mousemove",
                        self.mouse_move.as_ref().unchecked_ref(),
                    )
                    .expect("No event listener to remove");
                event_target
                    .remove_event_listener_with_callback(
                        "mouseup",
                        self.mouse_up.as_ref().unchecked_ref(),
                    )
                    .expect("No event listener to remove");
                true
            }
            Msg::Keyboard(event) if self.props.options.len() > 1 => match event.key().as_str() {
                "ArrowDown" | "ArrowLeft" => {
                    self.focus_handle = true;
                    event.prevent_default();
                    let index = self
                        .props
                        .options
                        .iter()
                        .position(|(value, _)| *value == self.props.value)
                        .map(|i| i.saturating_sub(1))
                        .unwrap_or(0);
                    let (value, _) = self.props.options[index].clone();
                    self.props.onchange.emit(value);
                    true
                }
                "ArrowUp" | "ArrowRight" => {
                    self.focus_handle = true;
                    event.prevent_default();
                    let index = self
                        .props
                        .options
                        .iter()
                        .position(|(value, _)| *value == self.props.value)
                        .map(|i| i.saturating_add(1))
                        .unwrap_or(0);
                    let (value, _) = self
                        .props
                        .options
                        .get(index)
                        .unwrap_or_else(|| {
                            self.props.options.last().expect(
                                "Already check, \
                                    there are at least 2 values in self.props.options; qed",
                            )
                        })
                        .clone();
                    self.props.onchange.emit(value);
                    true
                }
                _ => false,
            },
            Msg::Keyboard(_) => false,
        }
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
            .position(|(value, _)| *value == self.props.value);
        let labels = if self.props.options.len() > 1 {
            self
            .props
            .options
            .iter()
            .enumerate()
            .filter_map(|(i, (_, label))| {
                label.clone().map(|x| {
                    html! {
                        <div
                            class=classes!("bp3-slider-label")
                            style=format!("left: {}%;", (i as f64) * 100.0 / ((self.props.options.len() as f64) - 1.0))
                        >
                            {x}
                        </div>
                    }
                })
            })
            .collect::<Html>()
        } else if let Some((_, Some(label))) = self.props.options.first() {
            html! {
                <div
                    class=classes!("bp3-slider-label")
                    style="left: 50%;"
                >
                    {label}
                </div>
            }
        } else {
            html!()
        };
        let value_label = self.props.value_label.clone().map(|x| {
            html! {
                <span class=classes!("bp3-slider-label")>
                    {x}
                </span>
            }
        });

        html! {
            <div
                class=classes!(
                    "bp3-slider",
                    self.props.vertical.then(|| "bp3-vertical"),
                )
                onclick?=(self.props.options.len() > 1).then(|| self.link.callback(
                    |event| Msg::Mouse(event)
                ))
            >
                <div
                    class=classes!("bp3-slider-track")
                    ref={self.track_ref.clone()}
                >
                    {
                        if value_index.is_none() && !self.props.options.is_empty() {
                            html! {
                                <div
                                    class=classes!("bp3-slider-progress")
                                    style="top: 0px;"
                                    onkeydown=self.link.callback(|event| Msg::Keyboard(event))
                                    tabindex=0
                                >
                                </div>
                            }
                        } else {
                            html! {
                                <div
                                    class=classes!("bp3-slider-progress")
                                    style="top: 0px;"
                                >
                                </div>
                            }
                        }
                    }
                    {
                        match value_index {
                            Some(index) if self.props.options.len() > 1 => {
                                html! {
                                    <div
                                        class=classes!("bp3-slider-progress", self.props.intent)
                                        style=format!(
                                            "left: 0%; right: {}%; top: 0px;",
                                            100.0 - (
                                                100.0 * (index as f64)
                                                / (self.props.options.len() as f64 - 1.0)
                                            )
                                        )
                                    >
                                    </div>
                                }
                            }
                            _ => html! {},
                        }
                    }
                </div>
                <div class=classes!("bp3-slider-axis")>
                    {labels}
                </div>
                {
                    match value_index {
                        Some(index) if self.props.options.len() > 1 =>
                            {
                            html! {
                                <span
                                    class=classes!(
                                        "bp3-slider-handle",
                                        self.is_moving.then(|| "bp3-active"),
                                    )
                                    ref={self.handle_ref.clone()}
                                    style=format!(
                                        "left: calc({}% - 8px);",
                                        100.0 * (index as f64)
                                            / (self.props.options.len() as f64 - 1.0),
                                    )
                                    onmousedown=self.link.callback(|_| Msg::StartChange)
                                    onkeydown=self.link.callback(|event| Msg::Keyboard(event))
                                    tabindex=0
                                >
                                    {value_label.clone().unwrap_or_default()}
                                </span>
                            }
                        }
                        Some(_) => {
                            html! {
                                <span
                                    class=classes!(
                                        "bp3-slider-handle",
                                        self.is_moving.then(|| "bp3-active"),
                                    )
                                    ref={self.handle_ref.clone()}
                                    style="left: calc(50% - 8px);"
                                >
                                    {value_label.clone().unwrap_or_default()}
                                </span>
                            }
                        }
                        None => html!(),
                    }
                }
            </div>
        }
    }

    fn rendered(&mut self, _: bool) {
        if self.focus_handle {
            if let Some(element) = self.handle_ref.cast::<web_sys::HtmlElement>() {
                let _ = element.focus();
            }
            self.focus_handle = false;
        }
    }
}
