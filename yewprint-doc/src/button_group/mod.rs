mod example;
use crate::ExampleContainer;
use example::*;
use yew::prelude::*;
use yewprint::{Switch, Text, H1, H5};

pub struct ButtonGroupDoc {
    callback: Callback<ExampleProps>,
    state: ExampleProps,
}

impl Component for ButtonGroupDoc {
    type Message = ExampleProps;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        ButtonGroupDoc {
            callback: link.callback(|x| x),
            state: ExampleProps {
                minimal: false,
                fill: false,
                large: false,
                vertical: false,
            },
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        self.state = msg;
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let example_props = self.state.clone();
        let source = crate::include_raw_html!(
            concat!(env!("OUT_DIR"), "/", file!(), ".html"),
            "bp3-code-block"
        );

        html! {
            <div>
                <H1 class=classes!("docs-title")>{"Button Group"}</H1>
                <a
                    class=classes!("bp3-text-muted")
                    href=BUTTON_GROUP_URL
                    target="_blank"
                >
                    <Text>{"Go to the source code"}</Text>
                </a>
                <ExampleContainer
                    source=source
                    props=Some(html! {
                        <ButtonGroupProps
                            callback={self.callback.clone()}
                            props=example_props.clone()
                        >
                        </ButtonGroupProps>
                    })
                >
                    <Example with example_props />
                </ExampleContainer>
            </div>
        }
    }
}

crate::build_example_prop_component! {
    ButtonGroupProps for ExampleProps =>
    fn view(&self) -> Html {
        html! {
            <div>
                <H5>{"Props"}</H5>
                <Switch
                    onclick=self.update_props(|props, _| ExampleProps {
                        minimal: !props.minimal,
                        ..props
                    })
                    checked=self.props.minimal
                    label=html!("Minimal")
                />
                <Switch
                    onclick=self.update_props(|props, _| ExampleProps{
                        fill: !props.fill,
                        ..props
                    })
                    checked=self.props.fill
                    label=html!("Fill")
                />
                <Switch
                    onclick=self.update_props(|props, _| ExampleProps{
                        large: !props.large,
                        ..props
                    })
                    checked=self.props.large
                    label=html!("Large")
                />
                <Switch
                    onclick=self.update_props(|props, _| ExampleProps {
                        vertical: !props.vertical,
                        ..props
                    })
                    checked=self.props.vertical
                    label=html!("Vertical")
                />
            </div>
        }
    }
}

const BUTTON_GROUP_URL: &'static str =
    "https://github.com/yewprint/yewprint/blob/main/yewprint/src/button_group.rs";

pub struct SourceCodeUrl {
    props: SourceCodeUrlProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct SourceCodeUrlProps {
    #[prop_or_default]
    pub url: &'static str,
}


impl Component for SourceCodeUrl {
    type Message = ();
    type Properties = SourceCodeUrlProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let get_url = reqwest::blocking::get(BUTTON_GROUP_URL).expect("Cannot get url");
        let url = if get_url.status().is_success() {
            BUTTON_GROUP_URL;
        } else {
            "https://github.com/yewprint/yewprint"
        };

        html! {
            <a
                class=classes!("bp3-text-muted")
                href=url
                target="_blank"
            >
                <Text>{"Go to the source code"}</Text>
            </a>
        }
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn button_group_url() {
        let get_url = reqwest::blocking::get(BUTTON_GROUP_URL).unwrap();

        assert!(get_url.status().is_success())
    }
}
*/
