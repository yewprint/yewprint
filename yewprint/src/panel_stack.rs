use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
use yew::prelude::*;

#[derive(Debug, PartialEq)]
pub struct PanelStackOpen {
    pub title: Option<Html>,
    pub content: Html,
}

#[derive(Clone)]
pub struct PanelStackState {
    opened_panels: Rc<RefCell<Vec<(Option<Html>, Html)>>>,
    version: usize,
}

impl PanelStackState {
    pub fn new(root_panel: PanelStackOpen) -> Self {
        let instance = Self {
            opened_panels: Default::default(),
            version: 0,
        };

        instance
            .opened_panels
            .borrow_mut()
            .push((root_panel.title, root_panel.content));

        instance
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

    pub fn open_panel(&mut self, panel: PanelStackOpen) -> bool {
        self.opened_panels
            .borrow_mut()
            .push((panel.title, panel.content));
        self.version = self.version.wrapping_add(1);
        true
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
