use crate::Intent;
use implicit_clone::ImplicitClone;
use std::fmt;
use yew::html::IntoPropValue;
use yew::prelude::*;

include!("icon_svg_paths.rs");

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct IconSize(pub f64);

impl IconSize {
    pub const STANDARD: IconSize = IconSize(16.0);
    pub const LARGE: IconSize = IconSize(20.0);

    pub fn as_f64(&self) -> f64 {
        self.0
    }

    pub fn as_f32(&self) -> f32 {
        self.0 as f32
    }
}

impl Default for IconSize {
    #[inline]
    fn default() -> Self {
        Self::STANDARD
    }
}

impl fmt::Display for IconSize {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

macro_rules! generate_into_prop_value {
    ($($ty:ty,)*) => {
        $(
        impl IntoPropValue<IconSize> for $ty {
            fn into_prop_value(self) -> IconSize {
                IconSize(self.into())
            }
        }

        impl IntoPropValue<IconSize> for &$ty {
            fn into_prop_value(self) -> IconSize {
                IconSize((*self).into())
            }
        }
        )*
    }
}

#[rustfmt::skip]
generate_into_prop_value!(
    u8, u16, u32,
    i8, i16, i32,
    f32,
);

impl IntoPropValue<f32> for IconSize {
    fn into_prop_value(self) -> f32 {
        self.as_f32()
    }
}

impl IntoPropValue<f64> for IconSize {
    fn into_prop_value(self) -> f64 {
        self.as_f64()
    }
}

impl Default for Icon {
    #[inline]
    fn default() -> Self {
        Icon::Blank
    }
}

impl ImplicitClone for Icon {}

impl Icon {
    pub fn render(&self) -> Html {
        self.render_with_props(&Default::default())
    }

    pub fn render_with_props(
        &self,
        IconProps {
            icon,
            class,
            title,
            color: fill,
            intent,
            size,
            onclick,
        }: &IconProps,
    ) -> Html {
        if let Icon::Custom(html) = icon {
            return html.clone();
        }

        let paths = if *size == IconSize::STANDARD {
            icon_svg_paths_16(icon)
        } else {
            icon_svg_paths_20(icon)
        };
        let pixel_grid_size = if *size >= IconSize::LARGE {
            IconSize::LARGE
        } else {
            IconSize::STANDARD
        };
        let icon_string = AttrValue::from(format!("{:?}", icon));
        let width = AttrValue::from(format!("{size}"));
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
}

#[derive(Debug, Default, Clone, PartialEq, Properties)]
pub struct IconProps {
    pub icon: Icon,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub title: Option<AttrValue>,
    #[prop_or_default]
    pub color: Option<AttrValue>,
    #[prop_or_default]
    pub intent: Option<Intent>,
    #[prop_or_default]
    pub size: IconSize,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

impl Component for Icon {
    type Properties = IconProps;
    type Message = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.props().icon.clone()
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        self.render_with_props(ctx.props())
    }
}

impl IntoPropValue<Icon> for &Option<Icon> {
    fn into_prop_value(self) -> Icon {
        self.clone().unwrap_or_else(|| Icon::Custom(html!()))
    }
}

impl IntoPropValue<Icon> for Option<Icon> {
    fn into_prop_value(self) -> Icon {
        self.unwrap_or_else(|| Icon::Custom(html!()))
    }
}

impl IntoPropValue<Icon> for Html {
    fn into_prop_value(self) -> Icon {
        Icon::Custom(self)
    }
}

impl IntoPropValue<Icon> for Option<Html> {
    fn into_prop_value(self) -> Icon {
        Icon::Custom(self.unwrap_or_default())
    }
}
