mod example;

use crate::ExampleContainer;
use example::*;
use yew::prelude::*;
use yewprint::{Slider, H1, H5};

pub struct BreadcrumbsDoc {
    callback: Callback<ExampleProps>,
    state: ExampleProps,
}

impl Component for BreadcrumbsDoc {
    type Message = ExampleProps;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        BreadcrumbsDoc {
            callback: link.callback(|x| x),
            state: ExampleProps { width: 100 },
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
                <H1 class=classes!("docs-title")>{"Breadcrumbs"}</H1>
                <div>
                    <ExampleContainer
                        source=source
                        props=Some(html! {
                            <BreadcrumbsProps
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
    BreadcrumbsProps for ExampleProps =>
    fn view(&self) -> Html {
        html! {
            <div>
                <H5>{"Props"}</H5>
                <Slider<u64>
                        selected=self.props.width
                        values={(0..=100)
                            .map(|x| (x, (x % 25 == 0)
                            .then(|| format!("{}%", x)) ))
                            .collect::<Vec<_>>()}
                        onchange=self.update_props(|props, width| ExampleProps {
                            width,
                            ..props
                        })
                    />
            </div>
        }
    }
}
