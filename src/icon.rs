use crate::Intent;
use implicit_clone::ImplicitClone;
use yew::prelude::*;

include!("icon_svg_paths.rs");

pub const ICON_SIZE_STANDARD: i32 = 16;
pub const ICON_SIZE_LARGE: i32 = 20;

impl Default for IconName {
    fn default() -> Self {
        IconName::Blank
    }
}

impl ImplicitClone for IconName {}

#[derive(Clone, PartialEq, Properties)]
pub struct IconProps {
    pub icon: IconName,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub title: Option<AttrValue>,
    #[prop_or_default]
    pub color: Option<AttrValue>,
    #[prop_or_default]
    pub intent: Option<Intent>,
    #[prop_or(16)]
    pub icon_size: i32,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

#[function_component(Icon)]
pub fn icon(
    IconProps {
        icon,
        class,
        title,
        color: fill,
        intent,
        icon_size,
        onclick,
    }: &IconProps,
) -> Html {
    let paths = if *icon_size == ICON_SIZE_STANDARD {
        icon_svg_paths_16(*icon)
    } else {
        icon_svg_paths_20(*icon)
    };
    let pixel_grid_size = if *icon_size >= ICON_SIZE_LARGE {
        ICON_SIZE_LARGE
    } else {
        ICON_SIZE_STANDARD
    };
    let icon_string = AttrValue::from(format!("{:?}", icon));
    let width = AttrValue::from(format!("{icon_size}"));
    let height = width.clone();

    html! {
        <span
            class={classes!("bp3-icon", class.clone(), intent)}
            {onclick}
        >
            <svg
                {fill}
                data-icon={&icon_string}
                {width}
                {height}
                viewBox={format!("0 0 {pixel_grid_size} {pixel_grid_size}")}
            >
                <desc>{title.clone().unwrap_or(icon_string)}</desc>
                {
                    paths
                        .iter()
                        .map(|x| html! {
                            <path d={*x} fillRule="evenodd" />
                        })
                        .collect::<Html>()
                }
            </svg>
        </span>
    }
}
