use yew::prelude::*;
use yewprint::{Button, Collapse, IconName, Intent};

pub struct ExampleContainer {
    collapsed: bool,
    props: ExampleContainerProps,
    link: &html::Scope<Self>,
}

pub enum Msg {
    ToggleSource,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ExampleContainerProps {
    pub source: yew::virtual_dom::VNode,
    pub children: html::Children,
    #[prop_or_default]
    pub props: Option<yew::virtual_dom::VNode>,
}

impl Component for ExampleContainer {
    type Message = Msg;
    type Properties = ExampleContainerProps;

    fn create(ctx: &Context<Self>) -> Self {
        ExampleContainer {
            collapsed: true,
            props,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleSource => self.collapsed ^= true,
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class={classes!("docs-example-wrapper")}>
                <div class={classes!("docs-example-frame", "docs-example-frame-row")}>
                    <div class={classes!("docs-example")}>
                        {self.props.children.clone()}
                    </div>
                    {
                        if let Some(props) = self.props.props.clone() {
                            html! {
                                <div class={classes!("docs-example-options")}>
                                    {props}
                                </div>
                            }
                        } else {
                            html!()
                        }
                    }
                </div>
                <div class={classes!("docs-source")}>
                    <Button
                        icon={IconName::Code}
                        fill={{true}}
                        intent={{Intent::Primary}}
                        minimal={{true}}
                        onclick={self.link.callback(|_| Msg::ToggleSource)}
                    >
                        {"View source"}
                    </Button>
                    <Collapse
                        is_open={!self.collapsed}
                        keep_children_mounted=true
                    >
                        {self.props.source.clone()}
                    </Collapse>
                </div>
            </div>
        }
    }
}

#[macro_export]
macro_rules! build_example_prop_component {
    ($name:ident for $prop_component:ty => $($view:tt)*) => {
        #[derive(Clone, PartialEq, Properties)]
        pub struct $name {
            callback: Callback<$prop_component>,
            props: $prop_component,
        }

        impl Component for $name {
            type Message = ();
            type Properties = Self;

            fn create(ctx: &Context<Self>) -> Self {
                props: ctx.props()
            }

            fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
                true
            }

            fn changed(&mut self, ctx: &Context<Self>) -> bool {
                if self.props != ctx.props().props || self.callback != ctx.props().callback {
                    self.props = ctx.props().props;
                    self.callback = ctx.props().callback;
                    true
                } else {
                    false
                }
            }

            $($view)*
        }

        impl $name {
            fn update_props<T>(
                &self,
                updater: impl Fn($prop_component, T) -> $prop_component + 'static,
            ) -> Callback<T> {
                let props = self.props.clone();
                self.callback.clone().reform(move |event| updater(props.clone(), event))
            }
        }
    };
}
