#[cfg(feature = "doc")]
pub mod doc {
    use yew::prelude::*;

    pub struct ButtonDoc;

    impl Component for ButtonDoc {
        type Message = ();
        type Properties = ();

        fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
            ButtonDoc
        }

        fn update(&mut self, _msg: Self::Message) -> ShouldRender {
            true
        }

        fn change(&mut self, _props: Self::Properties) -> ShouldRender {
            true
        }

        fn view(&self) -> Html {
            let source = crate::include_example!("example.rs");

            html! {
                <div>
                    <h1>{"Button"}</h1>
                    <div>{source}</div>
                </div>
            }
        }
    }
}
