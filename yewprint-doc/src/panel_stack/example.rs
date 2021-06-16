use yew::prelude::*;
use yewprint::{Button, PanelStack, PanelStackState, Text};

pub struct Example {
    link: ComponentLink<Self>,
    props: ExampleProps,
    state: PanelStackState,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ExampleProps {
    pub animate: bool,
    pub vertical: bool,
}

#[derive(Debug, PartialEq)]
pub enum ExampleMessage {
    OpenPanel2,
    ClosePanel,
}

impl Component for Example {
    type Message = ExampleMessage;
    type Properties = ExampleProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = PanelStackState::new(html! {
            <>
            <div>{"Hello World!"}</div>
            <Button
                onclick=link.callback(|_| ExampleMessage::OpenPanel2)
            >
                {"Open panel 2"}
            </Button>
            </>
        })
        .with_title(html! {
            <Text class=classes!("bp3-heading") ellipsize=true>
                {"Root Panel"}
            </Text>
        })
        .finish();

        Example { link, props, state }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            ExampleMessage::OpenPanel2 => self
                .state
                .open_panel(html! {
                    <>
                    <Button onclick=self.link.callback(|_| ExampleMessage::ClosePanel)>
                        {"Close panel"}
                    </Button>
                    <Panel2 />
                    </>
                })
                .with_title(html! {
                    <Text class=classes!("bp3-heading") ellipsize=true>
                        {"Panel 2"}
                    </Text>
                })
                .finish(),
            // Always close the last panel.
            ExampleMessage::ClosePanel => self.state.close_panel(),
        }
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
                <PanelStack
                    state=self.state.clone()
                    class=classes!("docs-panel-stack-example")
                />
            </div>
        }
    }
}

// Second panel: a simple counter

pub struct Panel2 {
    link: ComponentLink<Self>,
    counter: i64,
}

pub enum Panel2Message {
    AddOne,
}

impl Component for Panel2 {
    type Message = Panel2Message;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Panel2 { counter: 0, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Panel2Message::AddOne => {
                self.counter += 1;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <p>{"Counter: "}{self.counter}</p>
                <div>
                    <Button onclick=self.link.callback(|_| Panel2Message::AddOne)>
                        {"Add 1"}
                    </Button>
                </div>
            </div>
        }
    }
}
