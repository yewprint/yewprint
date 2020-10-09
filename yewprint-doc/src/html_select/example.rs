use yew::prelude::*;
use yewprint::HtmlSelect;

pub struct Example {
    props: ExampleProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ExampleProps {
    pub minimal: bool,
    pub fill: bool,
    pub disabled: bool,
    pub large: bool,
}

impl Component for Example {
    type Message = ();
    type Properties = ExampleProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Example { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
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
        html! {
            <div style="width: 400px; text-align: center;">
                <HtmlSelect
                    options={vec![
                        ("trace".to_string(), "TRACE".to_string()),
                        ("debug".to_string(), "DEBUG".to_string()),
                        ("info".to_string(), "INFO".to_string()),
                        ("warn".to_string(), "WARN".to_string()),
                        ("error".to_string(), "ERROR".to_string()),
                        ("off".to_string(), "OFF".to_string()),
                        ]}
                    minimal=self.props.minimal
                    fill=self.props.fill
                    disabled=self.props.disabled
                    large=self.props.large
                />
            </div>
        }
    }
}
