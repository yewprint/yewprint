use crate::Intent;
use yew::prelude::*;

const R: f32 = 45.0;
const PATH_LENGTH: f32 = 280.0;
const SPINNER_MIN_SIZE: f32 = 10.0;
const STROKE_WIDTH: f32 = 4.0;
const MIN_STROKE_WIDTH: f32 = 16.0;
pub const SPINNER_SIZE_SMALL: f32 = 20.0;
pub const SPINNER_SIZE_STANDARD: f32 = 50.0;
pub const SPINNER_SIZE_LARGE: f32 = 100.0;

#[derive(Clone, PartialEq, Properties)]
pub struct SpinnerProps {
    #[prop_or_default]
    pub intent: Option<Intent>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or(SPINNER_SIZE_STANDARD)]
    pub size: f32,
    #[prop_or(0.25)]
    pub value: f32,
}

#[function_component(Spinner)]
pub fn spinner(props: &SpinnerProps) -> Html {
    let size = f32::max(SPINNER_MIN_SIZE, props.size);
    let stroke_width = f32::min(MIN_STROKE_WIDTH, (STROKE_WIDTH * SPINNER_SIZE_LARGE) / size);
    let view_box = {
        let radius = R + stroke_width / 2.00;
        let view_box_x = 50.00 - radius;
        let view_box_width = radius * 2.00;
        format!(
            "{:.2} {:.2} {:.2} {:.2}",
            view_box_x, view_box_x, view_box_width, view_box_width,
        )
    };
    let spinner_track = AttrValue::from(format!(
        "M 50,50 m 0,-{R:.0} a {R:.0},{R:.0} 0 1 1 0,{R2:.0} a {R:.0},{R:.0} 0 1 1 0,-{R2:.0}",
        R2 = R * 2.0,
    ));
    let stroke_offset = PATH_LENGTH - PATH_LENGTH * props.value.clamp(0.0, 1.0);
    let width = AttrValue::from(format!("{size}"));
    let height = width.clone();

    html! {
        <div
            class={classes!(
                "bp3-spinner",
                props.intent,
                props.class.clone(),
            )}
        >
            <div
                class={classes!("bp3-spinner-animation")}
            >
                <svg
                    {width}
                    {height}
                    stroke-width={stroke_width.to_string()}
                    {view_box}
                >
                    <path
                        class={classes!("bp3-spinner-track")}
                        d={&spinner_track}
                    />
                    <path
                        class={classes!("bp3-spinner-head")}
                        d={&spinner_track}
                        pathLength={PATH_LENGTH.to_string()}
                        stroke-dasharray={format!("{} {}", PATH_LENGTH, PATH_LENGTH)}
                        stroke-dashoffset={stroke_offset.to_string()}
                    />
                </svg>
            </div>
        </div>
    }
}
