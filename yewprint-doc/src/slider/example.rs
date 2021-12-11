use std::borrow::Cow;
use yew::prelude::*;
use yewprint::{Intent, Slider, Tag};

pub struct Example {
    props: ExampleProps,
    float: f64,
    integer: i32,
    log_level: Option<LogLevel>,
    link: &html::Scope<Self>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ExampleProps {
    pub vertical: bool,
    pub intent: Option<Intent>,
}

pub enum Msg {
    FloatUpdate(f64),
    IntegerUpdate(i32),
    LogLevelUpdate(LogLevel),
    Noop,
}

impl Component for Example {
    type Message = Msg;
    type Properties = ExampleProps;

    fn create(ctx: &Context<Self>) -> Self {
        Example {
            props,
            float: 1.2,
            integer: 30,
            log_level: None,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::FloatUpdate(value) => {
                self.float = value;
            }
            Msg::IntegerUpdate(value) => {
                self.integer = value;
            }
            Msg::LogLevelUpdate(value) => {
                self.log_level = Some(value);
            }
            Msg::Noop => {}
        }
        true
    }

    fn view(&self) -> Html {
        let percentage_labels = (0..=100)
            .step_by(1)
            .map(|x| (x, (x % 10 == 0).then(|| format!("{}%", x).into())))
            .collect::<Vec<_>>();

        html! {
            <>
                <div
                    style="display: flex; align-items: flex-start; width: 100%"
                >
                    <Slider<f64>
                        selected={self.float}
                        values={vec![
                            (0.0, Some("0".into())),
                            (0.1, None),
                            (0.2, None),
                            (0.3, None),
                            (0.4, None),
                            (0.5, Some("0.5".into())),
                            (0.6, None),
                            (0.7, None),
                            (0.8, None),
                            (0.9, None),
                            (1.0, Some("1".into())),
                        ]}
                        intent={self.props.intent}
                        onchange={self.link.callback(|x| Msg::FloatUpdate(x))}
                    />
                    <Tag
                        style={Cow::Borrowed("width: 32px; margin-left: 16px")}
                        minimal=true
                        intent={self.props.intent}
                    >
                        {format!("{:.1}", self.float)}
                    </Tag>
                </div>
                <Slider<i32>
                    values={percentage_labels}
                    selected={self.integer}
                    intent={self.props.intent}
                    value_label={Cow::Owned(format!("{}%", self.integer))}
                    onchange={self.link.callback(|x| Msg::IntegerUpdate(x))}
                />
                <Slider<LogLevel>
                    values={vec![
                        (LogLevel::Off, Some("OFF".into())),
                        (LogLevel::Error, Some("ERROR".into())),
                        (LogLevel::Warn, Some("WARN".into())),
                        (LogLevel::Info, Some("INFO".into())),
                        (LogLevel::Debug, Some("DEBUG".into())),
                        (LogLevel::Trace, Some("TRACE".into())),
                    ]}
                    intent={self.props.intent}
                    selected={self.log_level.clone()}
                    onchange={self.link.callback(|x| Msg::LogLevelUpdate(x))}
                />
                <Slider<()>
                    values={vec![((), Some("Neo".into()))]}
                    intent={self.props.intent}
                    selected={()}
                    onchange={self.link.callback(|_| Msg::Noop)}
                />
            </>
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Off,
}
