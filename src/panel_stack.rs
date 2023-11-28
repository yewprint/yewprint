use crate::{Button, Icon};
use gloo::timers::callback::Timeout;
use implicit_clone::ImplicitClone;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
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

impl ImplicitClone for PanelStackState {}

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
        Rc::ptr_eq(&self.opened_panels, &other.opened_panels) && self.version == other.version
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

    // We keep track of the latest action to perform from the PanelStackState
    // because we need a mutable access to the action.
    action_to_perform: Option<StateAction>,
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

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            timeout_task: None,
            action_to_perform: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            PanelStackMessage::PopPanel => {
                ctx.props().state.opened_panels.borrow_mut().pop();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let opened_panels = ctx.props().state.opened_panels.borrow();
        let last = match self.action_to_perform {
            Some(StateAction::Pop) => opened_panels.len() - 2,
            _ => opened_panels.len() - 1,
        };

        html! {
            <div
                class={classes!(
                    "bp3-panel-stack2",
                    self.action_to_perform,
                    ctx.props().class.clone(),
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
                                match self.action_to_perform {
                                    _ if i == last => Animation::EnterStart,
                                    Some(StateAction::Push) if i == last - 1 =>
                                        Animation::ExitStart,
                                    Some(StateAction::Pop) if i == last + 1 =>
                                        Animation::ExitStart,
                                    _ => Animation::Exited,
                                }
                            }
                            onclose={(i > 0).then(|| ctx.props().onclose.clone()).flatten()}
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

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        self.action_to_perform = ctx.props().state.action;
        true
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        if self.action_to_perform.take() == Some(StateAction::Pop) {
            let link = ctx.link().clone();
            self.timeout_task.replace(Timeout::new(400, move || {
                link.send_message(PanelStackMessage::PopPanel)
            }));
        }
    }
}

struct Panel {
    animation: Animation,
    timeout_task: Option<Timeout>,
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

    fn view(&self, ctx: &Context<Self>) -> Html {
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
        let back_button = ctx.props().onclose.clone().map(|onclose| {
            html! {
                <Button
                    class={classes!("bp3-panel-stack-header-back")}
                    style={"padding-right:0"}
                    icon={Icon::ChevronLeft}
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
                    <span>{back_button}</span>
                    {ctx.props().title.clone()}
                    <span/>
                </div>
                {for ctx.props().children.iter()}
            </div>
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        self.animation = ctx.props().animation;
        true
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        match self.animation {
            Animation::EnterStart => {
                let link = ctx.link().clone();
                self.timeout_task.replace(Timeout::new(0, move || {
                    link.send_message(PanelMessage::UpdateAnimation(Animation::Entering));
                }));
            }
            Animation::Entering => {
                let link = ctx.link().clone();
                self.timeout_task.replace(Timeout::new(400, move || {
                    link.send_message(PanelMessage::UpdateAnimation(Animation::Entered));
                }));
            }
            Animation::Entered => {}
            Animation::ExitStart => {
                let link = ctx.link().clone();
                self.timeout_task.replace(Timeout::new(0, move || {
                    link.send_message(PanelMessage::UpdateAnimation(Animation::Exiting));
                }));
            }
            Animation::Exiting => {
                let link = ctx.link().clone();
                self.timeout_task.replace(Timeout::new(400, move || {
                    link.send_message(PanelMessage::UpdateAnimation(Animation::Exited));
                }));
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
