use crate::{Button, Icon, IconSize, Overlay, H4};
use std::cell::Cell;
use yew::prelude::*;

#[derive(Debug)]
pub struct Dialog {
    title_id: AttrValue,
    cb: MsgCallbacks<Self>,
}

#[derive(Debug, PartialEq, Properties)]
pub struct DialogProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: Option<AttrValue>,
    #[prop_or_default]
    pub open: bool,
    #[prop_or_default]
    pub onclose: Callback<()>,
    #[prop_or(true)]
    pub show_close_button: bool,
    #[prop_or_default]
    pub icon: Option<Icon>,
    #[prop_or_default]
    pub title: Option<Html>,
    #[prop_or_default]
    pub container_ref: NodeRef,
    #[prop_or_default]
    pub aria_labelledby: Option<AttrValue>,
    #[prop_or_default]
    pub aria_describedby: Option<AttrValue>,
    #[prop_or_default]
    pub children: Children,
}

#[derive(yew_callbacks::Callbacks)]
pub enum Msg {
    OnClose(MouseEvent),
}

impl Component for Dialog {
    type Properties = DialogProps;
    type Message = Msg;

    fn create(ctx: &Context<Self>) -> Self {
        thread_local! {
            static ID: Cell<usize> = Default::default();
        }
        let id = ID.with(|x| {
            let next = x.get().wrapping_add(1);
            x.replace(next)
        });
        let title_id = AttrValue::from(format!("title-bp-dialog-{id}"));

        Self {
            title_id,
            cb: ctx.link().into(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::OnClose(_event) => {
                ctx.props().onclose.emit(());
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Self::Properties {
            class,
            style,
            open,
            onclose,
            show_close_button,
            icon,
            title,
            container_ref,
            aria_labelledby,
            aria_describedby,
            children,
        } = ctx.props();

        let aria_labelledby = aria_labelledby
            .clone()
            .or(title.is_some().then(|| self.title_id.clone()));

        let close_button = show_close_button.then(|| {
            html! {
                <Button
                    aria_label="Close"
                    class={classes!("bp3-dialog-close-button")}
                    left_element={html!(<Icon icon={Icon::SmallCross} size={IconSize::LARGE} />)}
                    minimal=true
                    onclick={self.cb.on_close()}
                />
            }
        });

        let header = title.clone().map(|title| {
            html! {
                <div class={classes!("bp3-dialog-header")}>
                    <Icon {icon} size={IconSize::LARGE} aria_hidden=true tab_index={-1} />
                    <H4 id={&self.title_id}>{title}</H4>
                    {close_button}
                </div>
            }
        });

        html! {
            <Overlay
                {container_ref}
                class={classes!("bp3-dialog-container")}
                {open}
                {onclose}
                scrollable=true
                backdrop=true
            >
                <div
                    class={classes!("bp3-dialog", class.clone())}
                    role="dialog"
                    aria-labelledby={aria_labelledby}
                    aria-describedby={aria_describedby}
                    {style}
                >
                    {header}
                    {for children.iter()}
                </div>
            </Overlay>
        }
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct DialogFooterProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: Option<AttrValue>,
    #[prop_or(false)]
    pub minimal: bool,
    #[prop_or_default]
    pub actions: Option<Html>,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(DialogFooter)]
pub fn dialog_footer(props: &DialogFooterProps) -> Html {
    let DialogFooterProps {
        class,
        style,
        minimal,
        actions,
        children,
    } = props;

    let actions_html = actions.clone().map(|html| {
        html! {
            <div class="bp3-dialog-footer-actions">{html}</div>
        }
    });

    html! {
        <div
            class={classes!(
                "bp3-dialog-footer",
                (!minimal).then_some("bp3-dialog-footer-fixed"),
                class.clone(),
            )}
            {style}
        >
            <div class="bp3-dialog-footer-main-section">
                {for children.iter()}
            </div>
            {actions_html}
        </div>
    }
}
