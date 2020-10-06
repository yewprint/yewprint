use yew::prelude::*;
use yewprint::HtmlSelect;

pub struct Example {
    link: ComponentLink<Self>,
    counter: i64,
    props: ExampleProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ExampleProps {
    pub minimal: bool,
    pub fill: bool,
}

pub enum Msg {
    AddOne,
}

impl Component for Example {
    type Message = Msg;
    type Properties = ExampleProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Example {
            counter: 0,
            link,
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => self.counter += 1,
        }
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
                <p>{"Counter: "}{self.counter}</p>
                <div>
                    <HtmlSelect
                        onchange=self.link.callback(|_| Msg::AddOne)
                        minimal=self.props.minimal
                        fill=self.props.fill
                    />
                </div>
            </div>
        }
    }
}
