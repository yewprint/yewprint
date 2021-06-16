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
    OpenPanel2,
    ClosePanel,
}

impl Component for Example {
    type Message = ExampleMessage;
    type Properties = ExampleProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = PanelStackState::new(PanelStackOpen {
            title: Some(html! {
                <Text class=classes!("bp3-heading") ellipsize=true>
                    {"Hello World"}
                </Text>
            }),
            content: html! {
                <>
                <div>{"Root panel"}</div>
                <Button
                    onclick=link.callback(|_| ExampleMessage::OpenPanel2)
                >
                    {"Open panel 2"}
                </Button>
                </>
            },
        });

        Example { link, props, state }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            ExampleMessage::OpenPanel2 => self.state.open_panel(PanelStackOpen {
                title: Some(html! {
                    <Text class=classes!("bp3-heading") ellipsize=true>
                        {"Panel 2"}
                    </Text>
                }),
                content: html! {
                    <>
                    <div>{"Panel 2 content"}</div>
                    <Button onclick=self.link.callback(|_| ExampleMessage::ClosePanel)>
                        {"Close panel"}
                    </Button>
                    </>
                },
            }),
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
                <PanelStack state=self.state.clone() />
            </div>
        }
    }
}
