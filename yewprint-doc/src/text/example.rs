use yew::prelude::*;
use yewprint::Text;

pub struct Example {
    props: ExampleProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ExampleProps {
    pub ellipsize: bool,
    pub text: String,
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
            <div style="width: 150px; height: 20px">
                <Text class="bp3-fill" ellipsize=self.props.ellipsize>
                    {&self.props.text}
                </Text>
            </div>
        }
    }
}
