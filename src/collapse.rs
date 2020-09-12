use std::time::Duration;
use web_sys::Element;
use yew::prelude::*;
use yew::services::timeout::*;

pub struct Collapse {
    height: Option<String>,
    height_when_open: Option<i32>,
    animation_state: AnimationState,
    contents_ref: NodeRef,
    callback_delayed_state_change: Callback<()>,
    handle_delayed_state_change: Option<TimeoutTask>,
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
                AnimationState::Opening => {
                    crate::log!("open");
                    self.animation_state = AnimationState::Open;
                    self.height = Some("auto".to_string());
                    true
                }
                AnimationState::Closing => {
                    crate::log!("closed");
                    self.animation_state = AnimationState::Closed;
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
                        self.height = self.height_when_open.as_ref().map(|x| format!("{}px", x));
                    }
                }
            } else {
                match self.animation_state {
                    AnimationState::Closed | AnimationState::Closing => {}
                    _ => {
                        crate::log!("closingstart");
                        self.animation_state = AnimationState::ClosingStart;
                        self.height = self.height_when_open.as_ref().map(|x| format!("{}px", x));
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

        if client_height > 0 {
            self.height_when_open = Some(client_height);
        }

        match self.animation_state {
            AnimationState::OpenStart => {
                crate::log!("openstart -> opening {}", client_height);
                self.animation_state = AnimationState::Opening;
                //self.height = Some(format!("{}px", client_height));
                self.height_when_open = Some(client_height);
                self.handle_delayed_state_change =
                    Some(yew::services::timeout::TimeoutService::spawn(
                        self.props.transition_duration,
                        self.callback_delayed_state_change.clone(),
                    ));
            }
            AnimationState::ClosingStart => {
                crate::log!("closingstart -> closing {}", client_height);
                self.animation_state = AnimationState::Closing;
                self.height = Some("0px".to_string());
                self.height_when_open = Some(client_height);
                self.handle_delayed_state_change =
                    Some(yew::services::timeout::TimeoutService::spawn(
                        self.props.transition_duration,
                        self.callback_delayed_state_change.clone(),
                    ));
            }
            _ => {}
        }
    }

    fn view(&self) -> Html {
        let is_content_visible = !matches!(self.animation_state, AnimationState::Closed);
        crate::log!("is_content_visible: {}", is_content_visible);
        let should_render_children = is_content_visible || self.props.keep_children_mounted;
        let display_with_transform = is_content_visible
            && !matches!(
                self.animation_state,
                AnimationState::Closing | AnimationState::ClosingStart
            );
        let is_auto_height = matches!(self.height.as_deref(), Some("auto"));

        let mut container_style = String::new();
        if is_content_visible {
            if let Some(ref height) = self.height {
                container_style.push_str("height: ");
                container_style.push_str(height);
                container_style.push_str("; ");
            }
        }
        if is_auto_height {
            container_style.push_str("overflow-y: visible; ");
            container_style.push_str("transition: none 0s ease 0s; ");
        }
        container_style.push_str("border: red 2px solid;");

        let mut content_style = String::new();
        if display_with_transform {
            content_style.push_str("transform: translateY(0px); ");
        } else if let Some(ref height_when_open) = self.height_when_open {
            content_style.push_str(&format!("transform: translateY(-{}px); ", height_when_open));
        }
        if is_auto_height {
            content_style.push_str("transition: none 0s ease 0s; ");
        }

        html! {
            <div class="bp3-collapse" style={container_style}>
                <div
                    class="bp3-collapse-body"
                    style={content_style}
                    aria-hidden={!is_content_visible && self.props.keep_children_mounted}
                    ref={self.contents_ref.clone()}
                >
                    {
                        if should_render_children {
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
