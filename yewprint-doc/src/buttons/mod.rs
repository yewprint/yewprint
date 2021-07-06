mod example;

use crate::ExampleContainer;
use example::*;
use yew::prelude::*;
use yewprint::{Switch, Text, H1, H5};

macro_rules! build_source_code_component {
    ($constant_name:ident, $url:expr, $test_name:ident) => {
        const $constant_name: &'static str = $url

        pub type SourceCodeUrl {
            props: SourceCodeUrlProps,
        }

        #[derive(Clone, PartialEq, Properties)]
        pub struct SourceCodeUrlProps {
            #[prop_or_default]
            pub url: &'static str,
        }

        impl Component for SourceCodeUrl {
            type Message = ();
            type Properties = ();

            fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
                Self { props }
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
                    <a
                        class=classes!("bp3-text-muted")
                        href=self.props.url
                        target="_blank"
                    >
                        <Text>{"Go to the source code"}</Text>
                    </a>
                }
            }
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn $testname() {
                let get_url = reqwest::blocking::get($constant_name).unwrap();

                assert!(get_url.status().is_success())
            }
        }
    }
}


pub struct ButtonDoc {
    callback: Callback<ExampleProps>,
    state: ExampleProps,
}

impl Component for ButtonDoc {
    type Message = ExampleProps;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        ButtonDoc {
            callback: link.callback(|x| x),
            state: ExampleProps {
                minimal: false,
                fill: false,
                small: false,
                outlined: false,
                loading: false,
                large: false,
                active: false,
                disabled: false,
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
        let constant_name = "BUTTONS_URL";
        let source_code_url = build_source_code_component!(
            BUTTONS_URL,
            url: format!(
                "{}{}",
                "https://github.com/yewprint/yewprint/blob/main/yewprint/src/",
                "buttons.rs",
            ),
            check_buttons_url,
        );

        html! {
            <div>
                <H1 class=classes!("docs-title")>{"Button"}</H1>
                {source_code_url}
                <div>
                    <ExampleContainer
                        source=source
                        props=Some(html! {
                            <ButtonProps
                                callback={self.callback.clone()}
                                props=example_props.clone()
                            />
                        })
                    >
                        <Example with example_props />
                    </ExampleContainer>
                </div>
            </div>
        }
    }
}

crate::build_example_prop_component! {
    ButtonProps for ExampleProps =>
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
                    onclick=self.update_props(|props, _| ExampleProps {
                        fill: !props.fill,
                        ..props
                    })
                    checked=self.props.fill
                    label=html!("Fill")
                />
                <Switch
                    onclick=self.update_props(|props, _| ExampleProps {
                        small: !props.small,
                        ..props
                    })
                    checked=self.props.small
                    label=html!("Small")
                />
                <Switch
                    onclick=self.update_props(|props, _| ExampleProps {
                        outlined: !props.outlined,
                        ..props
                    })
                    checked=self.props.outlined
                    label=html!("Outlined")
                />
                <Switch
                    onclick=self.update_props(|props, _| ExampleProps {
                        loading: !props.loading,
                        ..props
                    })
                    checked=self.props.loading
                    label=html!("Loading")
                />
                <Switch
                    onclick=self.update_props(|props, _| ExampleProps {
                        large: !props.large,
                        ..props
                    })
                    checked=self.props.large
                    label=html!("Large")
                />
                <Switch
                    onclick=self.update_props(|props, _| ExampleProps {
                        active: !props.active,
                        ..props
                    })
                    checked=self.props.active
                    label=html!("Active")
                />
                <Switch
                    onclick=self.update_props(|props, _| ExampleProps {
                        disabled: !props.disabled,
                        ..props
                    })
                    checked=self.props.disabled
                    label=html!("Disabled")
                />
            </div>
        }
    }
}
