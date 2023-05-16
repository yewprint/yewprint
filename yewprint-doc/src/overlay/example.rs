use yew::prelude::*;
use yewprint::{Button, Card, Dark, Elevation, Icon, Intent, Overlay, H3};

pub struct Example {
    open: bool,
    tall: bool,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ExampleProps {
    pub backdrop: bool,
}

pub enum Msg {
    Open,
    Close,
    ToggleTall,
}

impl Component for Example {
    type Message = Msg;
    type Properties = ExampleProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Example {
            open: false,
            tall: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Open => {
                self.open = true;
            }
            Msg::Close => {
                self.open = false;
            }
            Msg::ToggleTall => {
                self.tall ^= true;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Self::Properties { backdrop } = &ctx.props();

        html! {
            <div>
                <div>
                    <Button
                        onclick={ctx.link().callback(|_| Msg::Open)}
                    >
                        {"Show overlay"}
                    </Button>
                    <Overlay
                        open={self.open}
                        onclose={ctx.link().callback(|_| Msg::Close)}
                        {backdrop}
                        class={classes!(Dark.classes())}
                        style="left: calc(50vw - 200px); margin: 10vh 0; top: 0; width: 400px;"
                    >
                        <Card elevation={Elevation::Level4} style="height: 100%">
                            <H3>{"I'm an Overlay!"}</H3>
                            <p>{"This is a simple container with some inline styles to position \
                            it on the screen."}</p>
                            <p>{"Click the \"Make me scroll\" button below to make this overlay's \
                            content really tall, which will make the overlay's container \
                            (but not the page) scrollable"}</p>
                            <div class="bp3-dialog-footer-actions">
                                <Button
                                    intent={Intent::Danger}
                                    onclick={ctx.link().callback(|_| Msg::Close)}
                                >
                                    {"Close"}
                                </Button>
                                <Button
                                    active={self.tall}
                                    onclick={ctx.link().callback(|_| Msg::ToggleTall)}
                                    icon={Icon::DoubleChevronDown}
                                    right_icon={Icon::DoubleChevronDown}
                                >
                                    {"Make me scroll"}
                                </Button>
                            </div>
                        </Card>
                    </Overlay>
                </div>
            </div>
        }
    }
}
