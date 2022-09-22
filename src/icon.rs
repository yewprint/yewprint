use crate::Intent;
use core::str::FromStr;
use yew::prelude::*;

include!(concat!(env!("OUT_DIR"), "/icon_svg_paths.rs"));

impl Default for IconName {
    fn default() -> Self {
        IconName::Blank
    }
}

impl FromStr for IconName {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut res: Result<Self, Self::Err> = Err(format!("cannot parse {} as IconName", s));

        let s = s.to_lowercase();

        for x in IconName::iter() {
            if format!("{:?}", x).to_lowercase() == s.to_lowercase() {
                res = Ok(x)
            }
        }

        res
    }
}

pub const ICON_SIZE_STANDARD: i32 = 16;
pub const ICON_SIZE_LARGE: i32 = 20;

#[derive(Clone, PartialEq, Properties)]
pub struct IconProps {
    pub icon: IconName,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub title: Option<String>,
    #[prop_or_default]
    pub color: Option<String>,
    #[prop_or_default]
    pub intent: Option<Intent>,
    #[prop_or(16)]
    pub icon_size: i32,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

#[function_component(Icon)]
pub fn icon(props: &IconProps) -> Html {
    let paths = if props.icon_size == ICON_SIZE_STANDARD {
        icon_svg_paths_16(props.icon)
    } else {
        icon_svg_paths_20(props.icon)
    };
    let pixel_grid_size = if props.icon_size >= ICON_SIZE_LARGE {
        ICON_SIZE_LARGE
    } else {
        ICON_SIZE_STANDARD
    };
    let icon_string = format!("{:?}", props.icon);

    html! {
        <span
            class={classes!("bp3-icon", props.class.clone(), props.intent)}
            onclick={props.onclick.clone()}
        >
            <svg
                fill={props.color.clone()}
                data-icon={icon_string.clone()}
                width={props.icon_size.to_string()}
                height={props.icon_size.to_string()}
                viewBox={format!("0 0 {x} {x}", x=pixel_grid_size)}
            >
                <desc>{props.title.clone().unwrap_or(icon_string)}</desc>
                {
                    paths.iter()
                        .map(|x| html! {
                            <path d={*x} fillRule="evenodd" />
                        })
                        .collect::<Html>()
                }
            </svg>
        </span>
    }
}
