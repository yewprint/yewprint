use crate::Text;
use yew::prelude::*;

pub struct PanelStack {
    link: ComponentLink<Self>,
    props: PanelStackProps,
    opened_panels: Vec<(Option<Html>, Html)>,
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct PanelStackProps {
    #[prop_or_default]
    pub title: Option<Html>,
    pub children: Children,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PanelMessage {
    OpenPanel(Option<Html>, Html),
    ClosePanel,
}

impl Component for PanelStack {
    type Message = PanelMessage;
    type Properties = PanelStackProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            opened_panels: Default::default(),
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            PanelMessage::OpenPanel(_, _) => todo!(),
            PanelMessage::ClosePanel => {
                self.opened_panels.pop();
                true
            }
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
        let mut it = self.opened_panels.iter().rev();
        let last = it.next();
        let previous = it.next();

        html! {
            <div
                class=classes!(
                    "bp3-panel-stack2",
                    "bp3-panel-stack2-push",
                    "docs-panel-stack-example")
            >
                <Panel
                    title=self.props.title.clone()
                    animation=Animation::None
                    onopen=self.link.callback(|(title, content)| PanelMessage::OpenPanel(title, content))
                    onclose=self.link.callback(|_| PanelMessage::ClosePanel)
                >
                {self.props.children.clone()}
                </Panel>
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
    onopen: Callback<(Option<Html>, Html)>,
    onclose: Callback<()>,
    children: Children,
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
        html! {
            <div class="bp3-panel-stack-view">
                <div class="bp3-panel-stack-header">
                    <span/>
                    {self.props.title.clone().unwrap_or_default()}
                    <span/>
                </div>
                <div>
                // TODO how to pass onopen and onclose
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
