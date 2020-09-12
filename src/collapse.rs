use std::time::Duration;
use web_sys::Element;
use yew::prelude::*;
use yew::services::*;

pub struct Collapse {
    height: Option<String>,
    translated: bool,
    disable_transition: bool,
    overflow_visible: bool,
    render_children: bool,
    height_when_open: Option<i32>,
    animation_state: AnimationState,
    contents_ref: NodeRef,
    callback_delayed_state_change: Callback<()>,
    handle_delayed_state_change: Option<Box<dyn Task>>,
    props: Props,
    link: ComponentLink<Self>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub is_open: bool,
    #[prop_or_default]
    pub children: html::Children,
    #[prop_or_default]
    pub keep_children_mounted: bool,
    #[prop_or_else(|| Duration::from_millis(200))]
    pub transition_duration: Duration,
}

enum AnimationState {
    OpenStart,
    Opening,
    Open,
    ClosingStart,
    Closing,
    Closed,
}

pub enum Message {
    DelayedStateChange,
}

impl Component for Collapse {
    type Message = Message;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Collapse {
            height: None,
            disable_transition: false,
            overflow_visible: false,
            translated: false,
            render_children: props.is_open || props.keep_children_mounted,
            height_when_open: None,
            animation_state: if props.is_open {
                AnimationState::Open
            } else {
                AnimationState::Closed
            },
            contents_ref: NodeRef::default(),
            callback_delayed_state_change: link.callback(|_| Message::DelayedStateChange),
            handle_delayed_state_change: None,
            props,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Message::DelayedStateChange => match self.animation_state {
                AnimationState::OpenStart => {
                    self.animation_state = AnimationState::Opening;
                    self.height = self.height_when_open.as_ref().map(|x| format!("{}px", x));
                    self.handle_delayed_state_change = Some(Box::new(TimeoutService::spawn(
                        self.props.transition_duration,
                        self.callback_delayed_state_change.clone(),
                    )));
                    true
                }
                AnimationState::ClosingStart => {
                    self.animation_state = AnimationState::Closing;
                    self.height = Some("0px".to_string());
                    self.handle_delayed_state_change = Some(Box::new(TimeoutService::spawn(
                        self.props.transition_duration,
                        self.callback_delayed_state_change.clone(),
                    )));
                    true
                }
                AnimationState::Opening => {
                    crate::log!("open");
                    self.animation_state = AnimationState::Open;
                    self.height = Some("auto".to_string());
                    true
                }
                AnimationState::Closing => {
                    crate::log!("closed");
                    self.animation_state = AnimationState::Closed;
                    self.render_children = false;
                    true
                }
                _ => false,
            },
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            if props.is_open {
                match self.animation_state {
                    AnimationState::Open | AnimationState::Opening => {}
                    _ => {
                        crate::log!("openstart");
                        self.animation_state = AnimationState::OpenStart;
                        self.render_children = true;
                        self.translated = false;
                    }
                }
            } else {
                match self.animation_state {
                    AnimationState::Closed | AnimationState::Closing => {}
                    _ => {
                        crate::log!("closingstart");
                        self.animation_state = AnimationState::ClosingStart;
                        self.height = self.height_when_open.as_ref().map(|x| format!("{}px", x));
                        self.translated = true;
                    }
                }
            }

            self.props = props;
            true
        } else {
            false
        }
    }

    fn rendered(&mut self, _first_render: bool) {
        let client_height = self.contents_ref.cast::<Element>().unwrap().client_height();

        match self.animation_state {
            AnimationState::OpenStart | AnimationState::ClosingStart => {
                crate::log!("openstart -> opening {}", client_height);
                self.height_when_open = Some(client_height);
                self.link.send_message(Message::DelayedStateChange);
            }
            _ => {}
        }
    }

    fn view(&self) -> Html {
        let mut container_style = String::new();
        container_style.push_str("border: red 2px solid;");
        if let Some(ref height) = self.height {
            container_style.push_str("height: ");
            container_style.push_str(height);
            container_style.push_str("; ");
        }
        if self.overflow_visible {
            container_style.push_str("overflow-y: visible; ");
        }
        if self.disable_transition {
            container_style.push_str("transition: none 0s ease 0s; ");
        }

        let mut content_style = String::new();
        if !self.translated {
            content_style.push_str("transform: translateY(0px); ");
        } else if let Some(ref height_when_open) = self.height_when_open {
            content_style.push_str(&format!("transform: translateY(-{}px); ", height_when_open));
        } else {
            crate::log!("translated but height unknown");
        }
        if self.disable_transition {
            content_style.push_str("transition: none 0s ease 0s; ");
        }

        html! {
            <div class="bp3-collapse" style={container_style}>
                <div
                    class="bp3-collapse-body"
                    style={content_style}
                    aria-hidden={!self.render_children}
                    ref={self.contents_ref.clone()}
                >
                    {
                        if self.render_children {
                            self.props.children.clone()
                        } else {
                            Default::default()
                        }
                    }
                </div>
            </div>
        }
    }
}
