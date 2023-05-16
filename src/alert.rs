use crate::{Button, Dialog, Icon, Intent};
use yew::prelude::*;

#[derive(Debug)]
pub struct Alert {
    cb: MsgCallbacks<Self>,
}

#[derive(Debug, PartialEq, Properties)]
pub struct AlertProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: Option<AttrValue>,
    #[prop_or_default]
    pub open: bool,
    #[prop_or_default]
    pub icon: Option<Icon>,
    #[prop_or_default]
    pub intent: Option<Intent>,
    #[prop_or_default]
    pub loading: bool,
    #[prop_or(html!("OK"))]
    pub confirm_button: Html,
    #[prop_or_default]
    pub cancel_button: Option<Html>,
    #[prop_or_default]
    pub onclose: Callback<bool>,
    #[prop_or_default]
    pub children: Children,
}

#[derive(yew_callbacks::Callbacks)]
pub enum Msg {
    OnCancel,
    OnConfirmClick(MouseEvent),
    OnCancelClick(MouseEvent),
}

impl Component for Alert {
    type Properties = AlertProps;
    type Message = Msg;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            cb: ctx.link().into(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let Self::Properties { onclose, .. } = ctx.props();
        let Self::Properties { loading, .. } = ctx.props();

        match msg {
            Msg::OnCancel => {
                if !loading {
                    onclose.emit(false);
                }
                false
            }
            Msg::OnConfirmClick(_event) => {
                if !loading {
                    onclose.emit(true);
                }
                false
            }
            Msg::OnCancelClick(_event) => {
                if !loading {
                    onclose.emit(false);
                }
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Self::Properties {
            class,
            style,
            open,
            icon,
            intent,
            loading,
            confirm_button,
            cancel_button,
            onclose: _,
            children,
        } = ctx.props();

        let cancel_button_html = cancel_button.clone().map(|x| {
            html! {
                <Button
                    disabled={loading}
                    onclick={self.cb.on_cancel_click()}
                >
                    {x}
                </Button>
            }
        });

        html! {
            <Dialog
                class={classes!("bp3-alert", class.clone())}
                {style}
                {open}
                onclose={self.cb.on_cancel()}
            >
                <div class={classes!("bp3-alert-body")}>
                    <Icon {icon} size={40} {intent} />
                    <div class={classes!("bp3-alert-contents")}>
                        {for children.iter()}
                    </div>
                </div>
                <div class={classes!("bp3-alert-footer")}>
                    <Button
                        {loading}
                        {intent}
                        onclick={self.cb.on_confirm_click()}
                    >
                        {confirm_button.clone()}
                    </Button>
                    {cancel_button_html}
                </div>
            </Dialog>
        }
    }
}
