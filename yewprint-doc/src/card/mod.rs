mod example;

use crate::ExampleContainer;
use example::*;
use implicit_clone::unsync::IArray;
use std::rc::Rc;
use yew::prelude::*;
use yewprint::{Elevation, HtmlSelect, Switch, H1, H5};

pub struct CardDoc {
    callback: Callback<ExampleProps>,
    state: ExampleProps,
}

impl Component for CardDoc {
    type Message = ExampleProps;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        CardDoc {
            callback: ctx.link().callback(|x| x),
            state: ExampleProps {
                elevation: Elevation::Level0,
                interactive: false,
            },
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        self.state = msg;
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let example_props = self.state.clone();
        let source = crate::include_raw_html!(
            concat!(env!("OUT_DIR"), "/", file!(), ".html"),
            "bp3-code-block"
        );

        html! {
            <div>
                <H1 class={classes!("docs-title")}>{"Card"}</H1>
                <SourceCodeUrl />
                <ExampleContainer
                    source={source}
                    props={Some(html! {
                        <CardProps
                            callback={self.callback.clone()}
                            example_props={example_props.clone()}
                        />
                    })}
                >
                    <Example ..example_props />
                </ExampleContainer>
            </div>
        }
    }
}

crate::build_example_prop_component! {
    CardProps for ExampleProps =>
        fn view(&self, ctx: &Context<Self>) -> Html {
            html! {
                <div>
                    <H5>{"Props"}</H5>
                    <div>
                        <Switch
                            onclick={self.update_props(ctx, |props, _| ExampleProps {
                                interactive: !props.interactive,
                                ..props
                            })}
                            checked={ctx.props().example_props.interactive}
                            label={html!("Toggle interaction")}
                        />
                        <p>{"Elevation:"}</p>
                        <HtmlSelect<Elevation>
                            options={IArray::<(Elevation, AttrValue)>::Rc(Rc::new([
                                (Elevation::Level0, "Level 0".into()),
                                (Elevation::Level1, "Level 1".into()),
                                (Elevation::Level2, "Level 2".into()),
                                (Elevation::Level3, "Level 3".into()),
                                (Elevation::Level4, "Level 4".into()),
                            ]))}
                            value={ctx.props().example_props.elevation}
                            onchange={self.update_props(ctx, |props, elevation| ExampleProps {
                                elevation,
                                ..props
                            })}
                        />
                    </div>
                </div>
            }
        }
}

crate::build_source_code_component!();
