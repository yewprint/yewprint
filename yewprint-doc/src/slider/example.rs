use implicit_clone::{unsync::IArray, ImplicitClone};
use yew::prelude::*;
use yewprint::{Intent, Slider, Tag};

pub struct Example {
    float: f64,
    integer: i32,
    log_level: Option<LogLevel>,
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

    fn create(_ctx: &Context<Self>) -> Self {
        Example {
            float: 1.2,
            integer: 30,
            log_level: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
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

    fn view(&self, ctx: &Context<Self>) -> Html {
        let percentage_labels = (0..=100)
            .step_by(1)
            .map(|x| (x, (x % 10 == 0).then(|| format!("{}%", x).into())))
            .collect::<IArray<_>>();

        html! {
            <>
                <div
                    style="display: flex; align-items: flex-start; width: 100%"
                >
                    <Slider<f64>
                        selected={self.float}
                        values={[
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
                        ].into_iter().collect::<IArray<_>>()}
                        intent={ctx.props().intent}
                        onchange={ctx.link().callback(|x| Msg::FloatUpdate(x))}
                    />
                    <Tag
                        style={"width: 32px; margin-left: 16px"}
                        minimal=true
                        intent={ctx.props().intent}
                    >
                        {format!("{:.1}", self.float)}
                    </Tag>
                </div>
                <Slider<i32>
                    values={percentage_labels}
                    selected={self.integer}
                    intent={ctx.props().intent}
                    value_label={format!("{}%", self.integer)}
                    onchange={ctx.link().callback(|x| Msg::IntegerUpdate(x))}
                />
                <Slider<LogLevel>
                    values={[
                        (LogLevel::Off, Some("OFF".into())),
                        (LogLevel::Error, Some("ERROR".into())),
                        (LogLevel::Warn, Some("WARN".into())),
                        (LogLevel::Info, Some("INFO".into())),
                        (LogLevel::Debug, Some("DEBUG".into())),
                        (LogLevel::Trace, Some("TRACE".into())),
                    ].into_iter().collect::<IArray<_>>()}
                    intent={ctx.props().intent}
                    selected={self.log_level}
                    onchange={ctx.link().callback(|x| Msg::LogLevelUpdate(x))}
                />
                <Slider<()>
                    values={[
                        ((), Some("Neo".into()))
                    ].into_iter().collect::<IArray<_>>()}
                    intent={ctx.props().intent}
                    selected={()}
                    onchange={ctx.link().callback(|_| Msg::Noop)}
                />
            </>
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Off,
}

impl ImplicitClone for LogLevel {}
