use yew::prelude::*;

pub struct Slider {
    props: SliderProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct SliderProps {}

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
            <div>
                <p>{"Hello, world!"}</p>
            </div>
        }
    }
}
