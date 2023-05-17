use crate::{Icon, IconSize, Intent, Spinner};
use yew::prelude::*;
use yew::virtual_dom::AttrValue;

macro_rules! generate_with_common_props {
    (
        $(#[$attr:meta])*
        $vis:vis struct $name:ident {
            $(
                $(#[$field_attr:meta])*
                $field_vis:vis $field_name:ident : $field_ty:ty,
            )*
        }
    ) => {
        $(#[$attr])*
        $vis struct $name {
            #[prop_or_default]
            pub fill: bool,
            #[prop_or_default]
            pub minimal: bool,
            #[prop_or_default]
            pub small: bool,
            #[prop_or_default]
            pub outlined: bool,
            #[prop_or_default]
            pub loading: bool,
            #[prop_or_default]
            pub large: bool,
            #[prop_or_default]
            pub active: bool,
            #[prop_or_default]
            pub disabled: bool,
            #[prop_or_default]
            pub icon: Option<Icon>,
            #[prop_or_default]
            pub right_icon: Option<Icon>,
            #[prop_or_default]
            pub intent: Option<Intent>,
            #[prop_or_default]
            pub title: Option<AttrValue>,
            #[prop_or_default]
            pub class: Classes,
            #[prop_or_default]
            pub style: Option<AttrValue>,
            #[prop_or_default]
            pub button_ref: NodeRef,
            #[prop_or_default]
            pub left_element: Option<Html>,
            #[prop_or_default]
            pub right_element: Option<Html>,
            #[prop_or_default]
            pub aria_label: Option<AttrValue>,
            #[prop_or_default]
            pub children: Children,

            $(
                $(#[$field_attr])*
                $field_vis $field_name: $field_ty,
            )*
        }

        impl $name {
            fn common_classes(&self) -> Classes {
                let disabled = self.disabled || self.loading;

                classes!(
                    "bp3-button",
                    self.fill.then_some("bp3-fill"),
                    self.minimal.then_some("bp3-minimal"),
                    self.small.then_some("bp3-small"),
                    self.outlined.then_some("bp3-outlined"),
                    self.loading.then_some("bp3-loading"),
                    self.large.then_some("bp3-large"),
                    (self.active && !disabled).then_some("bp3-active"),
                    disabled.then_some("bp3-disabled"),
                    self.intent,
                    self.class.clone(),
                )
            }

            fn render_children(&self) -> Html {
                html! {
                    <>
                    {
                        self.loading
                            .then(|| html! {
                                <Spinner
                                    class={classes!("bp3-button-spinner")}
                                    size={IconSize::LARGE}
                                />
                            })
                    }
                    <Icon icon={self.icon.clone()} />
                    {self.left_element.clone()}
                    {
                        (!self.children.is_empty())
                            .then(|| html! {
                                <span class="bp3-button-text">
                                    {for self.children.iter()}
                                </span>
                            })
                    }
                    <Icon icon={self.right_icon.clone()} />
                    {self.right_element.clone()}
                    </>
                }
            }
        }
    };
}

generate_with_common_props! {
    #[derive(Clone, PartialEq, Properties)]
    pub struct ButtonProps {
        #[prop_or_default]
        pub onclick: Callback<MouseEvent>,
    }
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let ButtonProps {
        loading,
        disabled,
        title,
        style,
        button_ref,
        aria_label,
        onclick,
        ..
    } = props;

    let disabled = *disabled || *loading;

    html! {
        <button
            class={props.common_classes()}
            {style}
            {title}
            aria-label={aria_label}
            onclick={(!disabled).then_some(onclick.clone())}
            ref={button_ref.clone()}
        >
            {props.render_children()}
        </button>
    }
}

generate_with_common_props! {
    #[derive(Clone, PartialEq, Properties)]
    pub struct AnchorButtonProps {
        #[prop_or_default]
        pub href: AttrValue,
        #[prop_or_default]
        pub target: Option<AttrValue>,
    }
}

#[function_component(AnchorButton)]
pub fn anchor_button(props: &AnchorButtonProps) -> Html {
    let AnchorButtonProps {
        loading,
        disabled,
        title,
        style,
        button_ref,
        aria_label,
        href,
        target,
        ..
    } = props;

    let disabled = *disabled || *loading;

    html! {
        <a
            class={props.common_classes()}
            {style}
            {title}
            aria-label={aria_label}
            href={(!disabled).then_some(href.clone())}
            {target}
            ref={button_ref.clone()}
        >
            {props.render_children()}
        </a>
    }
}
