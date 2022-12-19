use crate::Intent;
use yew::html::IntoPropValue;
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct ProgressBarProps {
    #[prop_or_default]
    pub animate: bool,
    #[prop_or_default]
    pub stripes: bool,
    #[prop_or(RatioOrPercentage(1.0))]
    pub value: RatioOrPercentage,
    #[prop_or_default]
    pub intent: Option<Intent>,
    #[prop_or_default]
    pub class: Classes,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RatioOrPercentage(f64);

impl IntoPropValue<RatioOrPercentage> for f64 {
    fn into_prop_value(self) -> RatioOrPercentage {
        RatioOrPercentage(self)
    }
}

impl IntoPropValue<RatioOrPercentage> for f32 {
    fn into_prop_value(self) -> RatioOrPercentage {
        RatioOrPercentage(self.into())
    }
}

macro_rules! generate_into_prop_value_integers {
    ($($ty:ty,)*) => {
        $(
        impl IntoPropValue<RatioOrPercentage> for $ty {
            fn into_prop_value(self) -> RatioOrPercentage {
                RatioOrPercentage(self as f64 / 100.0)
            }
        }
        )*
    };
}

#[rustfmt::skip]
generate_into_prop_value_integers!(
    u8, u16, u32,
    i8, i16, i32,
);

#[function_component(ProgressBar)]
pub fn progress_bar(
    ProgressBarProps {
        animate,
        stripes,
        value,
        intent,
        class,
    }: &ProgressBarProps,
) -> Html {
    let percent = (value.0 * 100.0).clamp(0.0, 100.0);
    let style = format!("width: {}%", percent);

    html! {
        <div
            class={classes!(
                "bp3-progress-bar",
                intent,
                (!animate).then_some("bp3-no-animation"),
                (!stripes).then_some("bp3-no-stripes"),
                class.clone(),
            )}
        >
            <div class={classes!("bp3-progress-meter")} {style} />
        </div>
    }
}
