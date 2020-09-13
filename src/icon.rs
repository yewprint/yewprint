use yew::prelude::*;

include!(concat!(env!("OUT_DIR"), "/icon_svg_paths.rs"));

impl Default for IconName {
    fn default() -> Self {
        IconName::Blank
    }
}

pub const SIZE_STANDARD: i32 = 16;
pub const SIZE_LARGE: i32 = 20;

pub struct Icon {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub icon: IconName,
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub title: Option<String>,
    #[prop_or_default]
    pub color: Option<String>,
    #[prop_or(16)]
    pub icon_size: i32,
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
}

impl Component for Icon {
    type Message = ();
    type Properties = Props;

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
        let mut class = "bp3-icon ".to_string();
        class.push_str(self.props.class.as_str());

        let paths = if self.props.icon_size == SIZE_STANDARD {
            icon_svg_paths_16(self.props.icon)
        } else {
            icon_svg_paths_20(self.props.icon)
        };
        let pixel_grid_size = if self.props.icon_size >= SIZE_LARGE {
            SIZE_LARGE
        } else {
            SIZE_STANDARD
        };
        let icon_string = format!("{:?}", self.props.icon);

        html! {
            <span class=class onclick?={self.props.onclick.clone()}>
                <svg
                    fill?={self.props.color.clone()}
                    data-icon={icon_string.clone()}
                    width={self.props.icon_size}
                    height={self.props.icon_size}
                    viewBox={format!("0 0 {x} {x}", x=pixel_grid_size)}
                >
                    <desc>{self.props.title.clone().unwrap_or_else(|| icon_string)}</desc>
                    {
                        paths.iter()
                            .map(|x| html! {
                                <path d=x fillRule="evenodd" />
                            })
                            .collect::<Vec<_>>()
                    }
                </svg>
            </span>
        }
    }
}
