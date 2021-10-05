mod example;

use crate::ExampleContainer;
use example::*;
use yew::prelude::*;
use yewprint::{NumericInput, Switch, H1, H5};

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
                first_min_value: Default::default(),
                first_max_value: Default::default(),
                first_increment: Default::default(),
                second_min_value: Default::default(),
                second_max_value: Default::default(),
                second_increment: Default::default(),
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
                    value=self.props.first_min_value
                    start_value=-10
                    placeholder=String::from("Minimum value for the first example")
                />
                <NumericInput<i32>
                    min_value=i32::MIN
                    max_value=i32::MAX
                    increment=1
                    value=self.props.first_max_value
                    start_value=10
                    placeholder=String::from("Maximum value for the first example")
                 />
                <NumericInput<i32>
                    min_value=i32::MIN
                    max_value=i32::MAX
                    increment=1
                    value=self.props.first_increment
                    start_value=1
                    placeholder=String::from("Increment value for the first example")
                />
                <NumericInput<f32>
                    min_value=f32::MIN
                    max_value=f32::MAX
                    increment=0.5
                    value=self.props.second_min_value
                    start_value=-1.0
                    placeholder=String::from("Minimum value for the second example")
                />
                <NumericInput<f32>
                    min_value=f32::MIN
                    max_value=f32::MAX
                    increment=0.5
                    value=self.props.second_max_value
                    start_value=1.0
                    placeholder=String::from("Maximum value for the second example")
                 />
                <NumericInput<f32>
                    min_value=f32::MIN
                    max_value=f32::MAX
                    increment=0.5
                    value=self.props.second_increment
                    start_value=0.1
                    placeholder=String::from("Increment value for the second example")
                />
            </div>
        }
    }
}
