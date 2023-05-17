use gloo::timers::callback::Timeout;
use yew::prelude::*;
use yewprint::{Alert, Button, Icon, Intent};

pub struct Example {
    open_error: bool,
    open_deletion: bool,
    loading: bool,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ExampleProps {
    pub will_load: bool,
}

pub enum Msg {
    OpenFileError,
    OpenFileDeletion,
    Close(bool),
    FinishClosing,
}

impl Component for Example {
    type Message = Msg;
    type Properties = ExampleProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Example {
            open_error: false,
            open_deletion: false,
            loading: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let Self::Properties { will_load, .. } = *ctx.props();

        match msg {
            Msg::OpenFileError => {
                self.loading = false;
                self.open_error = true;
            }
            Msg::OpenFileDeletion => {
                self.loading = false;
                self.open_deletion = true;
            }
            Msg::Close(res) => {
                if res {
                    if will_load {
                        self.loading = true;
                        let callback = ctx.link().callback(|()| Msg::FinishClosing);
                        Timeout::new(2000, move || callback.emit(())).forget();
                    } else {
                        self.open_error = false;
                        self.open_deletion = false;
                    }
                } else {
                    self.open_error = false;
                    self.open_deletion = false;
                }
            }
            Msg::FinishClosing => {
                self.open_error = false;
                self.open_deletion = false;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <Button
                    onclick={ctx.link().callback(|_| Msg::OpenFileError)}
                >
                    {"Open file error alert"}
                </Button>
                <Alert
                    open={self.open_error}
                    onclose={ctx.link().callback(|res| Msg::Close(res))}
                    loading={self.loading}
                >
                    <p>{"Couldn't create the file because the containing folder doesn't \
                    exist anymore. You will be assimilated."}</p>
                </Alert>
                <Button
                    onclick={ctx.link().callback(|_| Msg::OpenFileDeletion)}
                >
                    {"Open file deletion alert"}
                </Button>
                <Alert
                    open={self.open_deletion}
                    onclose={ctx.link().callback(|res| Msg::Close(res))}
                    loading={self.loading}
                    icon={Icon::Trash}
                    intent={Intent::Danger}
                    confirm_button={html!("Move to Trash")}
                    cancel_button={html!("Cancel")}
                >
                    <p>{"Are you sure you want to move "}<b>{"filename"}</b>{" to Trash? \
                    You will be able to restore it later, but it will become Borg."}</p>
                </Alert>
            </>
        }
    }
}
