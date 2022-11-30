use crate::Intent;
use std::borrow::Cow;
use std::marker::PhantomData;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::Element;
use yew::prelude::*;

pub struct Slider<T: Clone + PartialEq + 'static> {
    mouse_move: Closure<dyn FnMut(MouseEvent)>,
    mouse_up: Closure<dyn FnMut(MouseEvent)>,
    handle_ref: NodeRef,
    track_ref: NodeRef,
    is_moving: bool,
    focus_handle: bool,
    phantom: PhantomData<T>,
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
    pub value_label: Option<Cow<'static, str>>,
    pub onchange: Callback<T>,
    pub values: Vec<(T, Option<Cow<'static, str>>)>,
    pub selected: Option<T>,
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

    fn create(ctx: &Context<Self>) -> Self {
        let mouse_move = {
            let link = ctx.link().clone();
            Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                link.send_message(Msg::Mouse(event));
            }) as Box<dyn FnMut(_)>)
        };
        let mouse_up = {
            let link = ctx.link().clone();
            Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
                link.send_message(Msg::StopChange);
            }) as Box<dyn FnMut(_)>)
        };
        Self {
            mouse_move,
            mouse_up,
            handle_ref: NodeRef::default(),
            track_ref: NodeRef::default(),
            is_moving: false,
            focus_handle: false,
            phantom: PhantomData,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::StartChange if ctx.props().values.len() > 1 => {
                let document = gloo::utils::document();
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
            Msg::Mouse(event) if ctx.props().values.len() > 1 => {
                if event.buttons() == crate::MOUSE_EVENT_BUTTONS_PRIMARY {
                    let track_rect = self.track_ref.cast::<Element>().expect("no track ref");
                    let tick_size = (track_rect.client_width() as f64)
                        / ctx.props().values.len().saturating_sub(1) as f64;
                    let pixel_delta =
                        (event.client_x() as f64) - track_rect.get_bounding_client_rect().left();

                    let position = (pixel_delta / tick_size).round() as usize;

                    let (value, _) =
                        ctx.props().values.get(position).unwrap_or_else(|| {
                            ctx.props().values.last().expect("No value in the vec")
                        });

                    if Some(value) != ctx.props().selected.as_ref() {
                        ctx.props().onchange.emit(value.clone());
                    }

                    true
                } else {
                    false
                }
            }
            Msg::Mouse(_) => false,
            Msg::StopChange => {
                let document = gloo::utils::document();
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
            Msg::Keyboard(event) if ctx.props().values.len() > 1 => match event.key().as_str() {
                "ArrowDown" | "ArrowLeft" => {
                    self.focus_handle = true;
                    event.prevent_default();
                    let index = ctx
                        .props()
                        .values
                        .iter()
                        .position(|(value, _)| Some(value) == ctx.props().selected.as_ref())
                        .map(|i| i.saturating_sub(1))
                        .unwrap_or(0);
                    let (value, _) = ctx.props().values[index].clone();
                    ctx.props().onchange.emit(value);
                    true
                }
                "ArrowUp" | "ArrowRight" => {
                    self.focus_handle = true;
                    event.prevent_default();
                    let index = ctx
                        .props()
                        .values
                        .iter()
                        .position(|(value, _)| Some(value) == ctx.props().selected.as_ref())
                        .map(|i| i.saturating_add(1))
                        .unwrap_or(0);
                    let (value, _) = ctx
                        .props()
                        .values
                        .get(index)
                        .unwrap_or_else(|| {
                            ctx.props().values.last().expect(
                                "Already check, \
                                    there are at least 2 values in ctx.props().options; qed",
                            )
                        })
                        .clone();
                    ctx.props().onchange.emit(value);
                    true
                }
                _ => false,
            },
            Msg::Keyboard(_) => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let value_index = ctx
            .props()
            .values
            .iter()
            .position(|(value, _)| Some(value) == ctx.props().selected.as_ref());
        let labels = if ctx.props().values.len() > 1 {
            ctx.props()
                .values
                .iter()
                .enumerate()
                .filter_map(|(i, (_, label))| {
                    label.clone().map(|x| {
                        html! {
                            <div
                                class={classes!("bp3-slider-label")}
                                style={format!(
                                    "left: {}%;", (i as f64) * 100.0
                                        / ((ctx.props().values.len() as f64) - 1.0)
                                )}
                            >
                                {x}
                            </div>
                        }
                    })
                })
                .collect::<Html>()
        } else if let Some((_, Some(label))) = ctx.props().values.first() {
            html! {
                <div
                    class={classes!("bp3-slider-label")}
                    style={"left: 50%;"}
                >
                    {label}
                </div>
            }
        } else {
            html!()
        };
        let value_label = ctx.props().value_label.clone().map(|x| {
            html! {
                <span class={classes!("bp3-slider-label")}>
                    {x}
                </span>
            }
        });

        html! {
            <div
                class={classes!(
                    "bp3-slider",
                    ctx.props().vertical.then_some("bp3-vertical"),
                )}
                onmousedown={(ctx.props().values.len() > 1).then(
                    || ctx.link().batch_callback(
                        |event: MouseEvent| {
                            if event.buttons() ==
                                crate::MOUSE_EVENT_BUTTONS_PRIMARY
                            {
                                vec![Msg::StartChange, Msg::Mouse(event)]
                            } else {
                                vec![]
                            }
                        }
                    )
                )}
            >
                <div
                    class={classes!("bp3-slider-track")}
                    ref={self.track_ref.clone()}
                >
                    {
                        if value_index.is_none() && !ctx.props().values.is_empty() {
                            html! {
                                <div
                                    class={classes!("bp3-slider-progress")}
                                    style="top: 0px;"
                                    onkeydown={ctx.link().callback(|event| Msg::Keyboard(event))}
                                    tabindex=0
                                >
                                </div>
                            }
                        } else {
                            html! {
                                <div
                                    class={classes!("bp3-slider-progress")}
                                    style="top: 0px;"
                                >
                                </div>
                            }
                        }
                    }
                    {
                        match value_index {
                            Some(index) if ctx.props().values.len() > 1
                                && ctx.props().intent.is_some() => {
                                html! {
                                    <div
                                        class={classes!("bp3-slider-progress", ctx.props().intent)}
                                        style={format!(
                                            "left: 0%; right: {}%; top: 0px;",
                                            100.0 - (
                                                100.0 * (index as f64)
                                                / (ctx.props().values.len() as f64 - 1.0)
                                            )
                                        )}
                                    >
                                    </div>
                                }
                            }
                            _ => html! {},
                        }
                    }
                </div>
                <div class={classes!("bp3-slider-axis")}>
                    {labels}
                </div>
                {
                    match value_index {
                        Some(index) if ctx.props().values.len() > 1 =>
                            {
                            html! {
                                <span
                                    class={classes!(
                                        "bp3-slider-handle",
                                        self.is_moving.then_some("bp3-active"),
                                    )}
                                    ref={self.handle_ref.clone()}
                                    style={format!(
                                        "left: calc({}% - 8px);",
                                        100.0 * (index as f64)
                                            / (ctx.props().values.len() as f64 - 1.0),
                                    )}
                                    onmousedown={ctx.link().batch_callback(
                                    |event: MouseEvent| {
                                        if event.buttons() == crate::MOUSE_EVENT_BUTTONS_PRIMARY {
                                            vec![Msg::StartChange]
                                        } else {
                                            vec![]
                                        }
                                    })}
                                    onkeydown={ctx.link().callback(|event| Msg::Keyboard(event))}
                                    tabindex=0
                                >
                                    {value_label.clone()}
                                </span>
                            }
                        }
                        Some(_) => {
                            html! {
                                <span
                                    class={classes!(
                                        "bp3-slider-handle",
                                        self.is_moving.then_some("bp3-active"),
                                    )}
                                    ref={self.handle_ref.clone()}
                                    style="left: calc(50% - 8px);"
                                >
                                    {value_label.clone()}
                                </span>
                            }
                        }
                        None => html!(),
                    }
                }
            </div>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _: bool) {
        if self.focus_handle {
            if let Some(element) = self.handle_ref.cast::<web_sys::HtmlElement>() {
                let _ = element.focus();
            }
            self.focus_handle = false;
        }
    }
}
