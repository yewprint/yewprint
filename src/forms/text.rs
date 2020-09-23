use yew::prelude::*;

pub struct Text {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub value: String,
    #[prop_or_default]
    pub oninput:Callback<InputData>
}


impl Component for Text {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
            Text {  props }
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
            <label class="bp3-text">
            <textarea
               placeholder="enter text"
               oninput=self.props.oninput.clone()
               value=&self.props.value
            />
            
            </label>
        }
    }
}

#[cfg(feature = "dev")]
pub mod doc {
    use super::*;

    pub struct TextDoc {
        props: Props,
    }

    #[derive(Clone, PartialEq, Properties)]
    pub struct Props {
        pub value: String,
        pub oninput:Callback<InputData>
    }

    impl Component for TextDoc {
        type Message = ();
        type Properties = Props;

        fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
            TextDoc { props }
        }

        fn update(&mut self, _msg: Self::Message) -> ShouldRender {
            true
        }

        fn change(&mut self, _props: Self::Properties) -> ShouldRender {
            true
        }

        fn view(&self) -> Html {
            html! {
                <div>
                    <Text
                        oninput=self.props.oninput.clone()
                        value=self.props.value
                        label="Dark theme"
                    />
                </div>
            }
        }
    }
}
