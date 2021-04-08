use yew::prelude::*;
use yewprint::{Intent, Slider, Tag};

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
    Noop,
}

impl Component for Example {
    type Message = Msg;
    type Properties = ExampleProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Example {
            props,
            float: 1.2,
            integer: 30,
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
            Msg::Noop => {}
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
        let percentage_labels = (0..=100)
            .step_by(1)
            .map(|x| (x, (x % 10 == 0).then(|| format!("{}%", x))))
            .collect::<Vec<(i32, Option<String>)>>();

        html! {
            <>
                <div
                    style="display: flex; align-items: flex-start; width: 100%"
                >
                    <Slider<f64>
                        value=self.float
                        options=vec![
                            (0.0, Some(String::from("0"))),
                            (0.1, None),
                            (0.2, None),
                            (0.3, None),
                            (0.4, None),
                            (0.5, Some(String::from("0.5"))),
                            (0.6, None),
                            (0.7, None),
                            (0.8, None),
                            (0.9, None),
                            (1.0, Some(String::from("1"))),
                        ]
                        intent=self.props.intent
                        onchange=self.link.callback(|x| Msg::FloatUpdate(x))
                    />
                    <Tag
                        style="width: 32px; margin-left: 16px"
                        minimal=true
                        intent=self.props.intent
                    >
                        {format!("{:.1}", self.float)}
                    </Tag>
                </div>
                <Slider<i32>
                    options=percentage_labels
                    value=self.integer
                    intent=self.props.intent
                    value_label=format!("{}%", self.integer)
                    onchange=self.link.callback(|x| Msg::IntegerUpdate(x))
                />
                <Slider<LogLevel>
                    options=vec![
                        (LogLevel::Off, Some("OFF".to_string())),
                        (LogLevel::Error, Some("ERROR".to_string())),
                        (LogLevel::Warn, Some("WARN".to_string())),
                        (LogLevel::Info, Some("INFO".to_string())),
                        (LogLevel::Debug, Some("DEBUG".to_string())),
                        (LogLevel::Trace, Some("TRACE".to_string())),
                    ]
                    intent=self.props.intent
                    value=self.log_level.clone()
                    onchange=self.link.callback(|x| Msg::LogLevelUpdate(x))
                />
                <Slider<()>
                    options=vec![((), Some("Neo".to_string()))]
                    intent=self.props.intent
                    value=()
                    onchange=self.link.callback(|_| Msg::Noop)
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
