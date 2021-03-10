use crate::Intent;
use yew::prelude::*;

pub const R: i32 = 45;
pub const SPINNER_TRACK: i32 = 0; // `M 50,50 m 0,-${R} a ${R},${R} 0 1 1 0,${R * 2} a ${R},${R} 0 1 1 0,-${R * 2}`;
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
    pub has_value: bool,
    #[prop_or_default]
    pub value: i32,
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
        html! {
            <div
                class=classes!(
                    "bp3-spinner",
                )
            >
                <div
                    class=classes!("bp3-spinner-animation")
                >
                    <svg
                        width=self.props.size
                        height=self.props.size
                        // stroke_width
                        // view_box
                    >
                        <path
                            class=classes!("bp3-spinner-track")
                        />
                        <path
                            class=classes!("bp3-spinner-head")
                        />
                    </svg>
                </div>
            </div>
        }
    }
}
