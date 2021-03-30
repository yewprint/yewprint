use yew::prelude::*;
use yewprint::{Intent, Slider};

pub struct Example {
    props: ExampleProps,
    float: f64,
    integer: i32,
    log_level: LogLevel,
    link: ComponentLink<Self>,
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
}

impl Component for Example {
    type Message = Msg;
    type Properties = ExampleProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Example {
            props,
            float: Default::default(),
            integer: Default::default(),
            log_level: LogLevel::Info,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FloatUpdate(value) => {
                self.float = value;
            }
            Msg::IntegerUpdate(value) => {
                self.integer = value;
            }
            Msg::LogLevelUpdate(value) => {
                self.log_level = value;
            }
        }
        true
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
        let labels = (0..=100)
            .step_by(10)
            .map(|x| (x, format!("{}%", x)))
            .collect::<Vec<(i32, String)>>();

        html! {
            <>
                <Slider<f64>
                    value=self.float
                    options=vec![
                        (0.0, String::from("0.0")),
                        (0.1, String::from("0.1")),
                        (0.2, String::from("0.2")),
                        (0.3, String::from("0.3")),
                        (0.4, String::from("0.4")),
                        (0.5, String::from("0.5")),
                        (0.6, String::from("0.6")),
                        (0.7, String::from("0.7")),
                        (0.8, String::from("0.8")),
                        (0.9, String::from("0.9")),
                        (1.0, String::from("1.0")),
                    ]
                    intent=self.props.intent
                    onchange=self.link.callback(|x| Msg::FloatUpdate(x))
                />
                <Slider<i32>
                    options=labels
                    value=self.integer
                    intent=self.props.intent
                    onchange=self.link.callback(|x| Msg::IntegerUpdate(x))
                />
                <Slider<LogLevel>
                    options=vec![
                        (LogLevel::Trace, "TRACE".to_string()),
                        (LogLevel::Debug, "DEBUG".to_string()),
                        (LogLevel::Info, "INFO".to_string()),
                        (LogLevel::Warn, "WARN".to_string()),
                        (LogLevel::Error, "ERROR".to_string()),
                        (LogLevel::Off, "OFF".to_string()),
                    ]
                    intent=self.props.intent
                    value=self.log_level.clone()
                    onchange=self.link.callback(|x| Msg::LogLevelUpdate(x))
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
