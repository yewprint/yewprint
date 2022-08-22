use yew::prelude::*;
use yewprint::Text;

#[derive(Clone, PartialEq, Eq, Properties)]
pub struct ExampleProps {
    pub ellipsize: bool,
    pub text: String,
}

#[function_component(Example)]
pub fn example(props: &ExampleProps) -> Html {
    html! {
        <div style="width: 150px; height: 20px">
            <Text ellipsize={props.ellipsize}>
                {&props.text}
            </Text>
        </div>
    }
}
