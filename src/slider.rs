use crate::Intent;
use implicit_clone::{unsync::IArray, ImplicitClone};
use std::marker::PhantomData;
use wasm_bindgen::JsCast;
use web_sys::Element;
use yew::prelude::*;

pub struct Slider<T: ImplicitClone + PartialEq + 'static> {
    handle_ref: NodeRef,
    track_ref: NodeRef,
    is_moving: bool,
    focus_handle: bool,
    phantom: PhantomData<T>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct SliderProps<T: ImplicitClone + PartialEq + 'static> {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub vertical: bool,
    #[prop_or_default]
    pub intent: Option<Intent>,
    #[prop_or_default]
    pub value_label: Option<AttrValue>,
    pub onchange: Callback<T>,
    pub values: IArray<(T, Option<AttrValue>)>,
    pub selected: Option<T>,
}

#[derive(Debug)]
pub enum Msg {
    StartChange { pointer_id: i32 },
    Mouse(PointerEvent),
    StopChange,
    Keyboard(KeyboardEvent),
}

impl<T: ImplicitClone + PartialEq + 'static> Component for Slider<T> {
    type Message = Msg;
    type Properties = SliderProps<T>;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            handle_ref: NodeRef::default(),
            track_ref: NodeRef::default(),
            is_moving: false,
            focus_handle: false,
            phantom: PhantomData,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::StartChange { pointer_id } if ctx.props().values.len() > 1 => {
                if let Some(handle) = self.handle_ref.cast::<web_sys::HtmlElement>() {
                    handle.set_pointer_capture(pointer_id).unwrap();
                }
                self.is_moving = true;
                true
            }
            Msg::StartChange { .. } => false,
            Msg::Mouse(event) if ctx.props().values.len() > 1 => {
                let track_rect = self.track_ref.cast::<Element>().expect("no track ref");
                let tick_size = (track_rect.client_width() as f64)
                    / ctx.props().values.len().saturating_sub(1) as f64;
                let pixel_delta =
                    (event.client_x() as f64) - track_rect.get_bounding_client_rect().left();

                let position = (pixel_delta / tick_size).round() as usize;

                let (value, _) = ctx.props().values.get(position).unwrap_or_else(|| {
                    ctx.props()
                        .values
                        .last()
                        .cloned()
                        .expect("No value in the array")
                });

                if Some(&value) != ctx.props().selected.as_ref() {
                    ctx.props().onchange.emit(value);
                }
                true
            }
            Msg::Mouse { .. } => false,
            Msg::StopChange => {
                self.is_moving = false;
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
                        .position(|(value, _)| Some(value) == ctx.props().selected)
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
                        .position(|(value, _)| Some(value) == ctx.props().selected)
                        .map(|i| i.saturating_add(1))
                        .unwrap_or(0);
                    let (value, _) = ctx.props().values.get(index).unwrap_or_else(|| {
                        let (value, label) = ctx.props().values.last().expect(
                            "Already check, \
                                    there are at least 2 values in ctx.props().options; qed",
                        );
                        (value.clone(), label.clone())
                    });
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
            .position(|(value, _)| Some(value) == ctx.props().selected);
        let labels = if ctx.props().values.len() > 1 {
            ctx.props()
                .values
                .iter()
                .enumerate()
                .filter_map(|(i, (_, label))| {
                    label.map(|x| {
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
                style={"touch-action: pan-y; -webkit-touch-callout: none;"}
                onpointerdown={(ctx.props().values.len() > 1).then(
                    || ctx.link().batch_callback(
                        |event: PointerEvent| {
                            vec![Msg::StartChange { pointer_id: event.pointer_id() }, Msg::Mouse(event)]
                        }
                    )
                )}
                onpointermove={(ctx.props().values.len() > 1).then(
                    || ctx.link().batch_callback(
                        |event: PointerEvent| {
                            if let Some(target) = event.target() {
                                if let Some(el) = target.dyn_ref::<web_sys::Element>() {
                                    if el.has_pointer_capture(event.pointer_id()) {
                                        return vec![Msg::Mouse(event)];
                                    }
                                }
                            }
                            vec![]
                        }
                    )
                )}
                onpointerup={(ctx.props().values.len() > 1).then(
                    || ctx.link().callback(
                        |_event: PointerEvent| {
                            Msg::StopChange
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
                                        "-webkit-touch-callout: none; left: calc({}% - 8px);",
                                        100.0 * (index as f64)
                                            / (ctx.props().values.len() as f64 - 1.0),
                                    )}
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
