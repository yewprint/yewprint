use yew::prelude::*;

pub struct Slider {
    props: SliderProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct SliderProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub vertical: bool,
    #[prop_or_default]
    pub initial_value: i32,
    #[prop_or_default]
    pub value: i32,
}

impl Component for Slider {
    type Message = ();
    type Properties = SliderProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
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
                    "bp3-slider",
                    self.props.vertical.then(|| "bp3-vertical"),
                )
            >
            </div>
        }
    }
}
