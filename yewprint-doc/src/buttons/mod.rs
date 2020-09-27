mod example;

use yew::prelude::*;
use yewprint::Switch;
use example::*;
use crate::ExampleContainer;

pub struct ButtonDoc {
    update: Callback<ExampleProps>,
    state: ExampleProps,
}

impl Component for ButtonDoc {
    type Message = ExampleProps;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        ButtonDoc {
            update: link.callback(|x| x),
            state: ExampleProps {
                active: true,
            },
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        self.state = msg;
        crate::log!("rerender 2");
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let source = crate::include_raw_html!(
            concat!(env!("OUT_DIR"), "/", file!(), ".html"),
            "bp3-code-block"
        );

        let example_props = self.state.clone();
        let example_container = html! {
            <ExampleContainer
                source=source
                example_props=Some(html!(<ButtonProps update={self.update.clone()} example_props=example_props.clone() />))
            >
                <Example with example_props />
            </ExampleContainer>
        };

        html! {
            <div>
                <h1>{"Button"}</h1>
                <div>{example_container}</div>
            </div>
        }
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct ButtonProps {
    update: Callback<ExampleProps>,
    example_props: ExampleProps,
}

impl Component for ButtonProps {
    type Message = ();
    type Properties = Self;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        props
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.example_props != props.example_props {
            self.example_props = props.example_props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let props = self.example_props.clone();

        html! {
            <div>
                <h1>{"Button props"}</h1>
                <Switch
                    onclick={self.update.clone().reform(move |_| ExampleProps { active: !props.active, ..props })}
                    checked=self.example_props.active
                />
            </div>
        }
    }
}
