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
    pub options: Vec<(T, Option<String>)>,
    #[prop_or_default]
    pub value_label: Option<String>,
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
                    .expect("No event listener to add");
                event_target
                    .add_event_listener_with_callback(
                        "mouseup",
                        self.mouse_up.as_ref().unchecked_ref(),
                    )
                    .expect("No event listener to add");
            }
            Msg::Mouse(event) => {
                let track_rect = self.track_ref.cast::<Element>().expect("no track ref");
                let tick_size = Some(
                    (track_rect.client_width() as f64)
                        / self.props.options.len().saturating_sub(1) as f64,
                );
                let pixel_delta = (event.client_x() as u32)
                    .saturating_sub(track_rect.get_bounding_client_rect().left() as u32);

                let position = (pixel_delta as f64 / tick_size.expect("tick size is None") as f64)
                    .round() as usize;

                let (value, _) = self
                    .props
                    .options
                    .get(position)
                    .unwrap_or_else(|| {
                        self.props
                            .options
                            .last()
                            .expect("value is not in the options")
                    })
                    .clone();

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
                    .expect("No event listener to remove");
                event_target
                    .remove_event_listener_with_callback(
                        "mouseup",
                        self.mouse_up.as_ref().unchecked_ref(),
                    )
                    .expect("No event listener to remove");
            }
            Msg::Keyboard(event) => match event.key().as_str() {
                "ArrowDown" | "ArrowLeft" => {
                    event.prevent_default();
                    let index = self
                        .props
                        .options
                        .iter()
                        .position(|(value, _)| *value == self.props.value)
                        .expect("Not in self.props.options");
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
                        .position(|(value, _)| *value == self.props.value)
                        .expect("Not in self.props.options");
                    let index = index.saturating_add(1);
                    let (value, _) = self
                        .props
                        .options
                        .get(index)
                        .unwrap_or_else(|| {
                            self.props
                                .options
                                .last()
                                .expect("No values in self.props.options")
                        })
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
            .position(|(value, _)| *value == self.props.value)
            .expect("self.props.options is empty");
        let percentage = 100.0 * (value_index as f64) / (self.props.options.len() as f64 - 1.0);
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
                onclick=self.link.callback(
                    |event| Msg::Mouse(event)
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
                        style=format!("left: 0%; right: {}%; top: 0px;", 100.0 - percentage)
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
                    onkeydown=self.link.callback(|event| Msg::Keyboard(event))
                    tabindex=0
                >
                    {value_label.clone().unwrap_or_default()}
                </span>
            </div>
        }
    }
}
