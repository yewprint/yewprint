use yew::prelude::*;
use yewprint::{Card, Elevation};

pub struct Example {
    props: ExampleProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ExampleProps {
    pub elevation: Option<Elevation>,
    pub interactive: bool,
}

impl Component for Example {
    type Message = ();
    type Properties = ExampleProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Example { props }
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
            <Card elevation=self.props.elevation interactive=self.props.interactive>
                <p>
                    {
                        "This is a card component. The elevation of the card can be adjusted. \
                        An interactive card reacts to being moused over."
                    }
                </p>
            </Card>
        }
    }
}
