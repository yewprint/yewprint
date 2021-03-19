use crate::Intent;
// use std::cmp::{min};
use yew::prelude::*;

pub const R: i32 = 45;
pub const PATH_LENGTH: i32 = 280;
pub const MIN_SIZE: i32 = 10;
pub const STROKE_WIDTH: i32 = 4;
pub const MIN_STROKE_WIDTH: i32 = 16;
pub const SIZE_SMALL: i32 = 20;
pub const SIZE_STANDARD: i32 = 50;
pub const SIZE_LARGE: i32 = 100;

pub struct Spinner {
    props: SpinnerProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct SpinnerProps {
    #[prop_or_default]
    pub intent: Option<Intent>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub size: i32,
    #[prop_or_default]
    pub value: Option<f32>,
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
        let default_value = 0.25;
        let path_length_float = PATH_LENGTH as f32;
        // fn get_size() -> i32 {}
        /* fn get_view_box(stroke_width: i32) -> String {
            let radius = R + stroke_width / 2;
            let view_box_x = (50 - radius).to_fixed(2);
            let view_box_width = (radius * 2).to_fixed(2);
            format!(
                "{} {} {} {}",
                view_box_x, view_box_x, view_box_width, view_box_width,
            )
        };*/
        let spinner_track: String = format!(
            "M 50,50 m 0,-{R} a {R},{R} 0 1 1 0,{R2} a {R},{R} 0 1 1 0,-{R2}",
            R = 45,
            R2 = 45 * 2,
        );
        // let size = self.get_size();
        // let stroke_width =
        //     cmp::min(MIN_STROKE_WIDTH, (STROKE_WIDTH * SIZE_LARGE) / size);
        let stroke_offset = path_length_float
            - path_length_float
                * (self
                    .props
                    .value
                    .map(|x| x.clamp(0.0, 1.0))
                    .unwrap_or(default_value));
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
                        width=self.props.size
                        height=self.props.size
                        // stroke_width=stroke_width.to_fixed(2)
                        // view_box= self.get_view_box(stroke_width)
                    >
                        <path
                            class=classes!("bp3-spinner-track")
                            d=spinner_track
                        />
                        <path
                            class=classes!("bp3-spinner-head")
                            d=spinner_track
                            path_length=PATH_LENGTH
                            stroke_dash_array=format!("{} {}", PATH_LENGTH, PATH_LENGTH)
                            stroke_dash_offset=stroke_offset
                        />
                    </svg>
                </div>
            </div>
        }
    }
}
