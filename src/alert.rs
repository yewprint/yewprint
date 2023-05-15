use crate::Dialog;
use yew::prelude::*;

#[derive(Debug)]
pub struct Alert {}

#[derive(Debug, PartialEq, Properties)]
pub struct AlertProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: Option<AttrValue>,
    #[prop_or_default]
    pub open: bool,
    #[prop_or_default]
    pub onclose: Callback<()>,
    #[prop_or_default]
    pub children: Children,
}

pub enum Msg {}

impl Component for Alert {
    type Properties = AlertProps;
    type Message = Msg;

    fn create(ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Self::Properties {
            class,
            style,
            open,
            onclose,
            children,
        } = ctx.props();

        html! {
            <Dialog
                class={class.clone()}
                {style}
                {open}
                {onclose}
            >
                {for children.iter()}
            </Dialog>
        }
    }
}
