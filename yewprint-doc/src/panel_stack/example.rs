use yew::prelude::*;
use yewprint::{Button, Intent, PanelStack, PanelStackState, Text};

pub struct Example {
    link: &html::Scope<Self>,
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

    fn create(ctx: &Context<Self>) -> Self {
        let state = PanelStackState::new(html! {
            <div class={classes!("docs-panel-stack-contents-example")}>
                <div>{"Hello World!"}</div>
                <Button
                    intent={Intent::Primary}
                    onclick={link.callback(|_| ExampleMessage::OpenPanel2)}
                >
                    {"Open panel 2"}
                </Button>
            </div>
        })
        .with_title(html! {
            <Text class={classes!("bp3-heading")} ellipsize=true>
                {"Root Panel"}
            </Text>
        })
        .finish();

        Example { link, props, state }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            ExampleMessage::OpenPanel2 => self
                .state
                .open_panel(html! {
                    <div class={classes!("docs-panel-stack-contents-example")}>
                        <Button
                            intent={Intent::Success}
                            onclick={self.link.callback(|_| ExampleMessage::OpenPanel2)}
                        >
                            {"Open another panel 2"}
                        </Button>
                        <Panel2 />
                    </div>
                })
                .with_title(html! {
                    <Text class={classes!("bp3-heading")} ellipsize=true>
                        {"Panel 2"}
                    </Text>
                })
                .finish(),
            // Always close the last panel.
            ExampleMessage::ClosePanel => self.state.close_panel(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <PanelStack
                    state={self.state.clone()}
                    onclose={self.link.callback(|_| ExampleMessage::ClosePanel)}
                    class={classes!("docs-panel-stack-example")}
                />
            </div>
        }
    }
}

// Second panel: a simple counter

pub struct Panel2 {
    link: &html::Scope<Self>,
    counter: i64,
}

pub enum Panel2Message {
    AddOne,
}

impl Component for Panel2 {
    type Message = Panel2Message;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Panel2 { counter: 0, link }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Panel2Message::AddOne => {
                self.counter += 1;
                true
            }
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <p>{"Counter: "}{self.counter}</p>
                <div>
                    <Button onclick={self.link.callback(|_| Panel2Message::AddOne)}>
                        {"Add 1"}
                    </Button>
                </div>
            </div>
        }
    }
}
