use yew::prelude::*;
use yewprint::{Button, Collapse, Icon, Intent};

pub struct ExampleContainer {
    collapsed: bool,
}

pub enum ExampleContainerMsg {
    ToggleSource,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ExampleContainerProps {
    pub source: Html,
    pub children: Children,
    #[prop_or_default]
    pub props: Option<Html>,
}

impl Component for ExampleContainer {
    type Message = ExampleContainerMsg;
    type Properties = ExampleContainerProps;

    fn create(_ctx: &Context<Self>) -> Self {
        ExampleContainer { collapsed: true }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ExampleContainerMsg::ToggleSource => self.collapsed ^= true,
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class={classes!("docs-example-wrapper")}>
                <div class={classes!("docs-example-frame", "docs-example-frame-row")}>
                    <div class={classes!("docs-example")}>
                        {ctx.props().children.clone()}
                    </div>
                    {
                        if let Some(props) = ctx.props().props.clone() {
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
                        icon={Icon::Code}
                        fill={{true}}
                        intent={{Intent::Primary}}
                        minimal={{true}}
                        onclick={ctx.link().callback(|_| ExampleContainerMsg::ToggleSource)}
                    >
                        {"View source"}
                    </Button>
                    <Collapse
                        is_open={!self.collapsed}
                        keep_children_mounted=true
                    >
                        {ctx.props().source.clone()}
                    </Collapse>
                </div>
            </div>
        }
    }
}

/// The macro generates the component that will be used to render the editable
/// properties of an associated example.
#[macro_export]
macro_rules! build_example_prop_component {
    ($name:ident for $prop_component:ty => $($view:tt)*) => {
        #[derive(Clone, PartialEq, Properties)]
        pub struct $name {
            callback: Callback<$prop_component>,
            example_props: $prop_component
        }

        impl Component for $name {
            type Message = ();
            type Properties = Self;

            fn create(ctx: &Context<Self>) -> Self {
                ctx.props().clone()
            }

            $($view)*
        }

        impl $name {
            /// Propagate the prop changes to the parent so the example is
            /// re-rendered.
            fn update_props<T>(
                &self,
                ctx: &Context<Self>,
                updater: impl Fn($prop_component, T) -> $prop_component + 'static,
            ) -> Callback<T> {
                let example_props = ctx.props().example_props.clone();
                self.callback
                    .clone()
                    .reform(move |event| updater(example_props.clone(), event))
            }
        }
    };
}
