use crate::{Button, IconName};
use gloo_timers::callback::Timeout;
use std::borrow::Cow;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
use std::time::Duration;
use yew::prelude::*;

pub struct PanelBuilder<F: Fn(Option<Html>, I) -> O, I, O> {
    title: Option<Html>,
    input: I,
    finish: F,
}

impl<F, I, O> PanelBuilder<F, I, O>
where
    F: Fn(Option<Html>, I) -> O,
{
    fn new(input: I, finish: F) -> PanelBuilder<F, I, O> {
        Self {
            title: None,
            input,
            finish,
        }
    }

    pub fn with_title(self, title: Html) -> PanelBuilder<F, I, O> {
        Self {
            title: Some(title),
            ..self
        }
    }

    pub fn finish(self) -> O {
        (self.finish)(self.title, self.input)
    }
}

#[derive(Clone)]
pub struct PanelStackState {
    opened_panels: Rc<RefCell<Vec<(Option<Html>, Html)>>>,
    version: usize,
    action: Option<StateAction>,
}

impl PanelStackState {
    pub fn new(content: Html) -> PanelBuilder<fn(Option<Html>, Html) -> Self, Html, Self> {
        PanelBuilder::new(content, |title, content| {
            let instance = PanelStackState {
                opened_panels: Default::default(),
                version: Default::default(),
                action: Default::default(),
            };

            instance.opened_panels.borrow_mut().push((title, content));

            instance
        })
    }

    pub fn close_panel(&mut self) -> bool {
        let opened_panels = self.opened_panels.borrow();
        if opened_panels.len() > 1 {
            self.version = self.version.wrapping_add(1);
            self.action.replace(StateAction::Pop);
            true
        } else {
            false
        }
    }

    pub fn open_panel(
        &mut self,
        content: Html,
    ) -> PanelBuilder<
        fn(Option<Html>, (Html, Rc<RefCell<Vec<(Option<Html>, Html)>>>)) -> bool,
        (Html, Rc<RefCell<Vec<(Option<Html>, Html)>>>),
        bool,
    > {
        let opened_panels = self.opened_panels.clone();
        self.version = self.version.wrapping_add(1);
        self.action.replace(StateAction::Push);
        PanelBuilder::new(
            (content, opened_panels),
            |title, (content, opened_panels)| {
                opened_panels.borrow_mut().push((title, content));
                true
            },
        )
    }
}

impl PartialEq for PanelStackState {
    fn eq(&self, other: &Self) -> bool {
        self.version == other.version
    }
}

impl fmt::Debug for PanelStackState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PanelStackState({})", self.version)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum StateAction {
    Push,
    Pop,
}

impl From<StateAction> for Classes {
    fn from(action: StateAction) -> Self {
        Classes::from(match action {
            StateAction::Push => "bp3-panel-stack2-push",
            StateAction::Pop => "bp3-panel-stack2-pop",
        })
    }
}

pub struct PanelStack {
    timeout_task: Option<Timeout>,
    props: PanelStackProps,
    link: html::Scope<Self>,
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct PanelStackProps {
    pub state: PanelStackState,
    #[prop_or_default]
    pub onclose: Option<Callback<()>>,
    #[prop_or_default]
    pub class: Classes,
}

pub enum PanelStackMessage {
    PopPanel,
}

impl Component for PanelStack {
    type Message = PanelStackMessage;
    type Properties = PanelStackProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            timeout_task: None,
            props: *ctx.props(),
            link: *ctx.link(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            PanelStackMessage::PopPanel => {
                self.props.state.opened_panels.borrow_mut().pop();
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let opened_panels = self.props.state.opened_panels.borrow();
        let action = self.props.state.action;
        let last = match action {
            Some(StateAction::Pop) => opened_panels.len() - 2,
            _ => opened_panels.len() - 1,
        };

        html! {
            <div
                class={classes!(
                    "bp3-panel-stack2",
                    action,
                    self.props.class.clone(),
                )}
            >
            {
                opened_panels
                    .iter()
                    .enumerate()
                    .rev()
                    .map(|(i, (title, content))| html! {
                        <Panel
                            title={title.clone()}
                            animation={
                                match action {
                                    _ if i == last => Animation::EnterStart,
                                    Some(StateAction::Push) if i == last - 1 =>
                                        Animation::ExitStart,
                                    Some(StateAction::Pop) if i == last + 1 =>
                                        Animation::ExitStart,
                                    _ => Animation::Exited,
                                }
                            }
                            onclose={(i > 0).then(|| self.props.onclose.clone()).flatten()}
                            key={i}
                        >
                            // TODO the state of content doesn't seem to be kept when re-opening
                            //      a panel using the same components
                            {content.clone()}
                        </Panel>
                    })
                    .collect::<Html>()
            }
            </div>
        }
    }

    fn rendered(&mut self, _first_render: bool) {
        if self.props.state.action.take() == Some(StateAction::Pop) {
            self.timeout_task.replace(Timeout::new(
                Duration::from_millis(400),
                self.link.callback(|_| PanelStackMessage::PopPanel),
            ));
        }
    }
}

struct Panel {
    animation: Animation,
    timeout_task: Option<Timeout>,
    props: PanelProps,
    link: html::Scope<Self>,
}

#[derive(Debug, Clone, PartialEq, Properties)]
struct PanelProps {
    title: Option<Html>,
    animation: Animation,
    onclose: Option<Callback<()>>,
    children: Children,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum PanelMessage {
    UpdateAnimation(Animation),
}

impl Component for Panel {
    type Message = PanelMessage;
    type Properties = PanelProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            animation: ctx.props().animation,
            timeout_task: None,
            props: *ctx.props(),
            link: *ctx.link(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            PanelMessage::UpdateAnimation(animation) => {
                self.animation = animation;
                true
            }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        if self.props != ctx.props() {
            self.animation = ctx.props().animation;
            self.props = ctx.props();
            true
        } else {
            false
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let style = if self.animation == Animation::Exited {
            "display:none"
        } else {
            "display:flex"
        };
        let classes = classes!(
            "bp3-panel-stack-view",
            match self.animation {
                Animation::EnterStart => Some("bp3-panel-stack2-enter"),
                Animation::Entering => Some("bp3-panel-stack2-enter bp3-panel-stack2-enter-active"),
                Animation::Entered => None,
                Animation::ExitStart => Some("bp3-panel-stack2-exit"),
                Animation::Exiting => Some("bp3-panel-stack2-exit bp3-panel-stack2-exit-active"),
                Animation::Exited => None,
            }
        );
        let back_button = self.props.onclose.clone().map(|onclose| {
            html! {
                <Button
                    class={classes!("bp3-panel-stack-header-back")}
                    style={Cow::Borrowed("padding-right:0")}
                    icon={IconName::ChevronLeft}
                    minimal={true}
                    small={true}
                    onclick={onclose.reform(|_| ())}
                >
                    // TODO: I get a lot of "VComp is not mounted" if I try to use the title
                    //       of the previous panel
                </Button>
            }
        });

        html! {
            <div class={classes} style={style}>
                <div class="bp3-panel-stack-header">
                    <span>{back_button.unwrap_or_default()}</span>
                    {self.props.title.clone().unwrap_or_default()}
                    <span/>
                </div>
                {for self.props.children.iter()}
            </div>
        }
    }

    fn rendered(&mut self, _first_render: bool) {
        match self.animation {
            Animation::EnterStart => {
                self.timeout_task.replace(Timeout::new(
                    Duration::from_millis(0),
                    self.link
                        .callback(|_| PanelMessage::UpdateAnimation(Animation::Entering)),
                ));
            }
            Animation::Entering => {
                self.timeout_task.replace(Timeout::new(
                    Duration::from_millis(400),
                    self.link
                        .callback(|_| PanelMessage::UpdateAnimation(Animation::Entered)),
                ));
            }
            Animation::Entered => {}
            Animation::ExitStart => {
                self.timeout_task.replace(Timeout::new(
                    Duration::from_millis(0),
                    self.link
                        .callback(|_| PanelMessage::UpdateAnimation(Animation::Exiting)),
                ));
            }
            Animation::Exiting => {
                self.timeout_task.replace(Timeout::new(
                    Duration::from_millis(400),
                    self.link
                        .callback(|_| PanelMessage::UpdateAnimation(Animation::Exited)),
                ));
            }
            Animation::Exited => {}
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Animation {
    EnterStart,
    Entering,
    Entered,
    ExitStart,
    Exiting,
    Exited,
}
