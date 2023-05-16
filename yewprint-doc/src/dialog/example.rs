use yew::prelude::*;
use yewprint::{AnchorButton, Button, Dark, Dialog, DialogBody, DialogFooter, Icon, Intent};

pub struct Example {
    show_dialog: bool,
    show_dialog_with_title: bool,
    show_dialog_with_title_and_footer: bool,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ExampleProps {}

pub enum Msg {
    ShowDialog,
    ShowDialogWithTitle,
    ShowDialogWithTitleAndFooter,
    Close,
}

impl Component for Example {
    type Message = Msg;
    type Properties = ExampleProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Example {
            show_dialog: false,
            show_dialog_with_title: false,
            show_dialog_with_title_and_footer: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ShowDialog => {
                self.show_dialog = true;
            }
            Msg::ShowDialogWithTitle => {
                self.show_dialog_with_title = true;
            }
            Msg::ShowDialogWithTitleAndFooter => {
                self.show_dialog_with_title_and_footer = true;
            }
            Msg::Close => {
                self.show_dialog = false;
                self.show_dialog_with_title = false;
                self.show_dialog_with_title_and_footer = false;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <Button
                    onclick={ctx.link().callback(|_| Msg::ShowDialog)}
                >
                    {"Show dialog"}
                </Button>
                <Dialog
                    open={self.show_dialog}
                    onclose={ctx.link().callback(|_| Msg::Close)}
                    class={Dark.classes()}
                >
                    <div style="margin: 0px 20px;">
                        <div class="bp3-popover2-target" aria-haspopup="true">
                            <InterestingDialogBody />
                            <VisitChatGptWebsiteButton />
                        </div>
                    </div>
                </Dialog>
                <Button
                    onclick={ctx.link().callback(|_| Msg::ShowDialogWithTitle)}
                >
                    {"Show dialog with title"}
                </Button>
                <Dialog
                    open={self.show_dialog_with_title}
                    onclose={ctx.link().callback(|_| Msg::Close)}
                    class={Dark.classes()}
                    title={html!("Embracing disruptive approach")}
                    icon={Icon::InfoSign}
                >
                    <div style="margin: 0px 20px;">
                        <div class="bp3-popover2-target" aria-haspopup="true">
                            <InterestingDialogBody />
                            <VisitChatGptWebsiteButton />
                        </div>
                    </div>
                </Dialog>
                <Button
                    onclick={ctx.link().callback(|_| Msg::ShowDialogWithTitleAndFooter)}
                >
                    {"Show dialog with title and footer"}
                </Button>
                <Dialog
                    open={self.show_dialog_with_title_and_footer}
                    onclose={ctx.link().callback(|_| Msg::Close)}
                    class={Dark.classes()}
                    title={html!("Embracing disruptive approach")}
                    icon={Icon::InfoSign}
                >
                    <InterestingDialogBody />
                    <DialogFooter
                        actions={html! {
                            <>
                            <Button onclick={ctx.link().callback(|_| Msg::Close)}>
                                {"Close"}
                            </Button>
                            <VisitChatGptWebsiteButton fill=false />
                            </>
                        }}
                    />
                </Dialog>
            </>
        }
    }
}

#[function_component(InterestingDialogBody)]
pub fn interesting_content(_: &()) -> Html {
    html! {
        <DialogBody>
            <p><strong>{"Our transformative solution optimizes paperclip management \
            through advanced technology, revolutionizing office supply logistics and \
            driving operational efficiency."}</strong></p>
            <p>{"Leveraging synergistic ideation and cutting-edge paradigm shifts,
            our groundbreaking solution addresses the pervasive issue of inefficient \
            paperclip utilization. We recognize the urgent need to optimize paperclip
            distribution and deployment, revolutionizing the way organizations engage
            with this crucial office asset. By seamlessly integrating blockchain
            technology, artificial intelligence algorithms, and quantum-inspired
            computing, our transformative platform offers a paradigmatic shift in the
            paperclip management landscape."}</p>
            <p>{"Our innovative solution streamlines paperclip logistics through a \
            gamified interface that incentivizes employees to achieve maximum paperclip
            productivity. Leveraging the power of machine learning, our algorithm
            predicts demand patterns, ensuring optimal inventory levels and eliminating
            wasteful overstocking. Furthermore, our blockchain-based tracking system
            guarantees immutable traceability, empowering organizations to perform
            data-driven analysis on paperclip utilization, leading to unprecedented
            insights and cost-saving opportunities."}</p>
            <p>{"By embracing this disruptive approach, organizations can transcend \
            traditional paperclip constraints and unlock a new era of operational
            efficiency. Our solution not only resolves the current paperclip dilemma
            but also paves the way for a sustainable future, reducing environmental
            impact through our proprietary green paperclip technology. Join us in the
            paperclip revolution and embrace the future of office supplies where
            mundane tasks are transformed into innovative experiences that drive
            productivity and fuel growth."}</p>
            <p>{"Unlock the power of seamless paperclip management with our innovative
            solution - experience unparalleled efficiency and cost-saving opportunities
            for your organization. Try it today and revolutionize your office supply
            logistics!"}</p>
        </DialogBody>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct VisitChatGptWebsiteButtonProps {
    #[prop_or(true)]
    pub fill: bool,
}

#[function_component(VisitChatGptWebsiteButton)]
pub fn visit_chat_gpt_website_button(
    VisitChatGptWebsiteButtonProps { fill }: &VisitChatGptWebsiteButtonProps,
) -> Html {
    html! {
        <AnchorButton
            href="https://chat.openai.com/"
            target="_blank"
            {fill}
            intent={Intent::Primary}
            icon={Icon::Share}
        >
            {"Generate all sorts of bs by visiting ChatGPT"}
        </AnchorButton>
    }
}
