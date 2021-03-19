use crate::Intent;
use yew::prelude::*;

include!(concat!(env!("OUT_DIR"), "/icon_svg_paths.rs"));

impl Default for IconName {
    fn default() -> Self {
        IconName::Blank
    }
}

pub const ICON_SIZE_STANDARD: i32 = 16;
pub const ICON_SIZE_LARGE: i32 = 20;

pub struct Icon {
    props: IconProps,
}

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

impl Component for Icon {
    type Message = ();
    type Properties = IconProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Icon { props }
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
        let paths = if self.props.icon_size == ICON_SIZE_STANDARD {
            icon_svg_paths_16(self.props.icon)
        } else {
            icon_svg_paths_20(self.props.icon)
        };
        let pixel_grid_size = if self.props.icon_size >= ICON_SIZE_LARGE {
            ICON_SIZE_LARGE
        } else {
            ICON_SIZE_STANDARD
        };
        let icon_string = format!("{:?}", self.props.icon);

        html! {
            <span
                class=classes!("bp3-icon", self.props.class.clone(), self.props.intent)
                onclick=self.props.onclick.clone()
            >
                <svg
                    fill?={self.props.color.clone()}
                    data-icon={icon_string.clone()}
                    width={self.props.icon_size}
                    height={self.props.icon_size}
                    viewBox={format!("0 0 {x} {x}", x=pixel_grid_size)}
                >
                    <desc>{self.props.title.clone().unwrap_or(icon_string)}</desc>
                    {
                        paths.iter()
                            .map(|x| html! {
                                <path d=x fillRule="evenodd" />
                            })
                            .collect::<Html>()
                    }
                </svg>
            </span>
        }
    }
}
