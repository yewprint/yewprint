use crate::Intent;
use std::cmp::{max, min};
use yew::prelude::*;

const R: u32 = 45;
const PATH_LENGTH: u32 = 280;
const SPINNER_MIN_SIZE: u32 = 10;
const STROKE_WIDTH: u32 = 4;
const MIN_STROKE_WIDTH: u32 = 16;
pub const SPINNER_SIZE_SMALL: u32 = 20;
pub const SPINNER_SIZE_STANDARD: u32 = 50;
pub const SPINNER_SIZE_LARGE: u32 = 100;

pub struct Spinner {
    props: SpinnerProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct SpinnerProps {
    #[prop_or_default]
    pub intent: Option<Intent>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or(SPINNER_SIZE_STANDARD)]
    pub size: u32,
    #[prop_or(0.25)]
    pub value: f32,
}

impl Component for Spinner {
    type Message = ();
    type Properties = SpinnerProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Spinner { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
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
        let path_length_float = PATH_LENGTH as f32;
        let size = max(SPINNER_MIN_SIZE, self.props.size);
        let stroke_width = min(MIN_STROKE_WIDTH, (STROKE_WIDTH * SPINNER_SIZE_LARGE) / size);

        let view_box = {
            let radius = R as f32 + stroke_width as f32 / 2.00;
            let view_box_x = 50.00 - radius;
            let view_box_width = radius * 2.00;
            format!(
                "{:.2} {:.2} {:.2} {:.2}",
                view_box_x, view_box_x, view_box_width, view_box_width,
            )
        };
        let spinner_track: String = format!(
            "M 50,50 m 0,-{R} a {R},{R} 0 1 1 0,{R2} a {R},{R} 0 1 1 0,-{R2}",
            R = R,
            R2 = R * 2,
        );
        let stroke_offset =
            path_length_float - path_length_float * (self.props.value.clamp(0.0, 1.0));
        html! {
            <div
                class=classes!(
                    "bp3-spinner",
                    self.props.intent,
                    self.props.class.clone(),
                )
            >
                <div
                    class=classes!("bp3-spinner-animation")
                >
                    <svg
                        width=size
                        height=size
                        stroke-width=stroke_width
                        viewBox=view_box
                    >
                        <path
                            class=classes!("bp3-spinner-track")
                            d=spinner_track
                        />
                        <path
                            class=classes!("bp3-spinner-head")
                            d=spinner_track
                            pathLength=PATH_LENGTH
                            stroke-dasharray=format!("{} {}", PATH_LENGTH, PATH_LENGTH)
                            stroke-dashoffset=stroke_offset
                        />
                    </svg>
                </div>
            </div>
        }
    }
}
