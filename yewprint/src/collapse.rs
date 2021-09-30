use std::time::Duration;
use web_sys::Element;
use yew::prelude::*;
use yew::services::*;

pub struct Collapse {
    height: Height,
    translated: bool,
    overflow_visible: bool,
    render_children: bool,
    height_when_open: Option<String>,
    animation_state: AnimationState,
    contents_ref: NodeRef,
    callback_delayed_state_change: Callback<()>,
    handle_delayed_state_change: Option<Box<dyn Task>>,
    props: CollapseProps,
    link: ComponentLink<Self>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct CollapseProps {
    #[prop_or_default]
    pub is_open: bool,
    #[prop_or_default]
    pub children: html::Children,
    #[prop_or_default]
    pub keep_children_mounted: bool,
    #[prop_or(Duration::from_millis(200))]
    pub transition_duration: Duration,
    #[prop_or_default]
    pub class: Classes,
}

#[derive(Copy, Clone, Debug)]
enum AnimationState {
    OpenStart,
    Opening,
    Open,
    ClosingStart,
    Closing,
    Closed,
}

#[derive(Copy, Clone, Debug)]
enum Height {
    Zero,
    Auto,
    Full,
}

impl Component for Collapse {
    type Message = ();
    type Properties = CollapseProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Collapse {
            height: if props.is_open {
                Height::Auto
            } else {
                Height::Zero
            },
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
            callback_delayed_state_change: link.callback(|_| ()),
            handle_delayed_state_change: None,
            props,
            link,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            if props.is_open {
                match self.animation_state {
                    AnimationState::Open | AnimationState::Opening => {}
                    _ => {
                        self.animation_state = AnimationState::OpenStart;
                        self.render_children = true;
                        self.translated = false;
                    }
                }
            } else {
                match self.animation_state {
                    AnimationState::Closed | AnimationState::Closing => {}
                    _ => {
                        self.animation_state = AnimationState::ClosingStart;
                        self.height = Height::Full;
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

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        match self.animation_state {
            AnimationState::OpenStart => {
                self.animation_state = AnimationState::Opening;
                self.height = Height::Full;
                self.handle_delayed_state_change = Some(Box::new(TimeoutService::spawn(
                    self.props.transition_duration,
                    self.callback_delayed_state_change.clone(),
                )));
                true
            }
            AnimationState::ClosingStart => {
                self.animation_state = AnimationState::Closing;
                self.height = Height::Zero;
                self.handle_delayed_state_change = Some(Box::new(TimeoutService::spawn(
                    self.props.transition_duration,
                    self.callback_delayed_state_change.clone(),
                )));
                true
            }
            AnimationState::Opening => {
                self.animation_state = AnimationState::Open;
                self.height = Height::Auto;
                true
            }
            AnimationState::Closing => {
                self.animation_state = AnimationState::Closed;
                if !self.props.keep_children_mounted {
                    self.render_children = false;
                }
                true
            }
            _ => false,
        }
    }

    fn rendered(&mut self, _first_render: bool) {
        if self.render_children {
            let client_height = self.contents_ref.cast::<Element>().unwrap().client_height();
            self.height_when_open = Some(format!("{}px", client_height));
        }

        match self.animation_state {
            AnimationState::OpenStart | AnimationState::ClosingStart => self.link.send_message(()),
            _ => {}
        }
    }

    fn view(&self) -> Html {
        let mut container_style = String::with_capacity(30);
        match (self.height, self.height_when_open.as_ref()) {
            (Height::Zero, _) => container_style.push_str("height: 0px; "),
            (Height::Auto, _) => container_style.push_str("height: auto; "),
            (Height::Full, Some(height)) => {
                container_style.push_str("height: ");
                container_style.push_str(height.as_str());
                container_style.push_str("; ");
            }
            _ => unreachable!("height_when_open was undefined while height is full"),
        };
        if self.overflow_visible {
            container_style.push_str("overflow-y: visible; ");
        }

        let mut content_style = String::with_capacity(40);
        if !self.translated {
            content_style.push_str("transform: translateY(0px); ");
        } else if let Some(ref height_when_open) = self.height_when_open {
            content_style.push_str(&format!("transform: translateY(-{}); ", height_when_open));
        } else {
            unreachable!("height_when_open was undefined while translated is set");
        }

        html! {
            <div class=classes!("bp3-collapse") style={container_style}>
                <div
                    class=classes!(
                        "bp3-collapse-body",
                        self.props.class.clone(),
                    )
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
