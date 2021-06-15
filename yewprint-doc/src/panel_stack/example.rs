use yew::prelude::*;
use yewprint::{Button, PanelStack, PanelStackOpen, PanelStackState, Text};

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
    OpenPanel(PanelStackOpen),
    ClosePanel,
}

impl Component for Example {
    type Message = ExampleMessage;
    type Properties = ExampleProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = PanelStackState::new(
            |onopen, onclose| PanelStackOpen {
                title: Some(html! {
                    <Text class=classes!("bp3-heading") ellipsize=true>
                        {"Hello World"}
                    </Text>
                }),
                content: html! {
                    <>
                    <div>{"Root panel"}</div>
                    <Button
                        onclick=onopen.reform(move |_| PanelStackOpen {
                            title: Some(html! {
                                <Text class=classes!("bp3-heading") ellipsize=true>
                                    {"Panel 2"}
                                </Text>
                            }),
                            content: html! {
                                <>
                                <div>{"Panel 2 content"}</div>
                                <Button onclick=onclose.reform(|_| ())>
                                    {"Close panel"}
                                </Button>
                                </>
                            },
                        })
                    >
                        {"Open panel 2"}
                    </Button>
                    </>
                },
            },
            link.callback(|open| ExampleMessage::OpenPanel(open)),
            link.callback(|_| ExampleMessage::ClosePanel),
        );

        Example { link, props, state }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            ExampleMessage::OpenPanel(open) => self.state.open_panel(open),
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
                <PanelStack state=self.state.clone() />
            </div>
        }
    }
}
