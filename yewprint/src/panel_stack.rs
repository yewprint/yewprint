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
}

impl PanelStackState {
    pub fn new(content: Html) -> PanelBuilder<fn(Option<Html>, Html) -> Self, Html, Self> {
        PanelBuilder::new(content, |title, content| {
            let instance = PanelStackState {
                opened_panels: Default::default(),
                version: 0,
            };

            instance.opened_panels.borrow_mut().push((title, content));

            instance
        })
    }

    pub fn close_panel(&mut self) -> bool {
        let mut opened_panels = self.opened_panels.borrow_mut();
        if opened_panels.len() > 1 {
            opened_panels.pop();
            self.version = self.version.wrapping_add(1);
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

pub struct PanelStack {
    props: PanelStackProps,
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct PanelStackProps {
    pub state: PanelStackState,
}

impl Component for PanelStack {
    type Message = ();
    type Properties = PanelStackProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        todo!()
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
        let count = self.props.state.opened_panels.borrow().len();

        html! {
            <div
                class=classes!(
                    "bp3-panel-stack2",
                    "bp3-panel-stack2-push",
                    "docs-panel-stack-example",
                )
            >
            {
                self
                    .props
                    .state
                    .opened_panels
                    .borrow()
                    .iter()
                    .enumerate()
                    .rev()
                    .map(|(i, (title, content))| html! {
                        <Panel
                            title=title.clone()
                            animation=Animation::None
                            selected={i == count - 1}
                        >
                            {content.clone()}
                        </Panel>
                    })
                    .collect::<Html>()
            }
            </div>
        }
    }
}

struct Panel {
    props: PanelProps,
}

#[derive(Debug, Clone, PartialEq, Properties)]
struct PanelProps {
    title: Option<Html>,
    animation: Animation,
    children: Children,
    selected: bool,
}

impl Component for Panel {
    type Message = ();
    type Properties = PanelProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        todo!()
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
        let style = (!self.props.selected).then(|| "display:none");

        html! {
            <div class="bp3-panel-stack-view" style=style>
                <div class="bp3-panel-stack-header">
                    <span/>
                    {self.props.title.clone().unwrap_or_default()}
                    <span/>
                </div>
                <div>
                {self.props.children.clone()}
                </div>
            </div>
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Animation {
    None,
}
