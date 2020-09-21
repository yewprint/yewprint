use yew::prelude::*;

pub struct Button {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub title: String,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    pub children: html::Children,
}

impl Component for Button {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Button { props }
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
            <button class="bp3-button" onclick={self.props.onclick.clone()}>
                {self.props.children.clone()}
            </button>
        }
    }
}

#[cfg(feature = "dev")]
pub mod doc {
    use super::*;

    pub struct ButtonDoc {
        link: ComponentLink<Self>,
        counter: i64,
    }

    pub enum Msg {
        AddOne,
    }

    impl Component for ButtonDoc {
        type Message = Msg;
        type Properties = ();

        fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
            ButtonDoc { counter: 0, link }
        }

        fn update(&mut self, msg: Self::Message) -> ShouldRender {
            match msg {
                Msg::AddOne => self.counter += 1,
            }
            true
        }

        fn change(&mut self, _props: Self::Properties) -> ShouldRender {
            true
        }

        fn view(&self) -> Html {
            html! {
                <div>
                    <p> {"Counter: "} { self.counter }</p>
                    <div>
                        <Button onclick=self.link.callback(|_| Msg::AddOne)>{ "Add 1" }</Button>
                    </div>
                </div>
            }
        }
    }
}
