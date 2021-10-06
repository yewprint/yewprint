mod example;

use crate::ExampleContainer;
use example::*;
use yew::prelude::*;
use yewprint::{Button, IconName, NumericInput, Switch, H1, H5};

pub struct NumericInputDoc {
    callback: Callback<ExampleProps>,
    state: ExampleProps,
}

impl Component for NumericInputDoc {
    type Message = ExampleProps;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        NumericInputDoc {
            callback: link.callback(|x| x),
            state: ExampleProps {
                fill: false,
                disabled: false,
                large: false,
                integer_value: 0,
                integer_min_value: -10,
                integer_max_value: 10,
                integer_increment: 1,
                float_value: 0.0,
                float_min_value: -1.0,
                float_max_value: 1.0,
                float_increment: 0.1,
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
                <H1 class=classes!("docs-title")>{"NumericInput"}</H1>
                <div>
                    <ExampleContainer
                        source=source
                        props=Some(html! {
                            <NumericInputProps
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
    NumericInputProps for ExampleProps =>
    fn view(&self) -> Html {
        html! {
            <div>
                <H5>{"Props"}</H5>
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
                        disabled: !props.disabled,
                        ..props
                    })
                    checked=self.props.disabled
                    label=html!("Disabled")
                />
                <Switch
                    onclick=self.update_props(|props, _| ExampleProps {
                        large: !props.large,
                        ..props
                    })
                    checked=self.props.large
                    label=html!("Large")
                />
                <NumericInput<i32>
                    min_value=i32::MIN
                    max_value=i32::MAX
                    increment=1
                    value=self.props.integer_min_value
                    placeholder=String::from("Minimum value for (integer)")
                    onchange=self.update_props(|props, integer_min_value| ExampleProps {
                        integer_min_value,
                        ..props
                    })
                />
                <NumericInput<i32>
                    min_value=i32::MIN
                    max_value=i32::MAX
                    increment=1
                    value=self.props.integer_max_value
                    placeholder=String::from("Maximum value (integer)")
                    onchange=self.update_props(|props, integer_max_value| ExampleProps {
                        integer_max_value,
                        ..props
                    })
                 />
                <NumericInput<i32>
                    min_value=i32::MIN
                    max_value=i32::MAX
                    increment=1
                    value=self.props.integer_increment
                    placeholder=String::from("Increment value (integer)")
                    onchange=self.update_props(|props, integer_increment| ExampleProps {
                        integer_increment,
                        ..props
                    })
                />
                <NumericInput<f32>
                    min_value=f32::MIN
                    max_value=f32::MAX
                    increment=0.5
                    value=self.props.float_min_value
                    placeholder=String::from("Minimum value (float)")
                    onchange=self.update_props(|props, float_min_value| ExampleProps {
                        float_min_value,
                        ..props
                    })
                />
                <NumericInput<f32>
                    min_value=f32::MIN
                    max_value=f32::MAX
                    increment=0.5
                    value=self.props.float_max_value
                    placeholder=String::from("Maximum value (float)")
                    onchange=self.update_props(|props, float_max_value| ExampleProps {
                        float_max_value,
                        ..props
                    })
                 />
                <NumericInput<f32>
                    min_value=f32::MIN
                    max_value=f32::MAX
                    increment=0.5
                    value=self.props.float_increment
                    placeholder=String::from("Increment (float)")
                    onchange=self.update_props(|props, float_increment| ExampleProps {
                        float_increment,
                        ..props
                    })
                />
                <Button
                    icon=IconName::Refresh
                    onclick=self.update_props(|_, _| ExampleProps {
                        fill: false,
                        disabled: false,
                        large: false,
                        integer_value: 0,
                        integer_min_value: -10,
                        integer_max_value: 10,
                        integer_increment: 1,
                        float_value: 0.0,
                        float_min_value: -1.0,
                        float_max_value: 1.0,
                        float_increment: 0.1,
                    })
                >
                    {"Reset"}
                </Button>
            </div>
        }
    }
}
