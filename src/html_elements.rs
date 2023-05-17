use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ChildrenOnlyProps {
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
}

macro_rules! build_component {
    ($name:ident, $tag:tt, $class:literal) => {
        #[function_component($name)]
        pub fn $tag(
            ChildrenOnlyProps {
                id,
                class,
                children,
            }: &ChildrenOnlyProps,
        ) -> Html {
            html! {
                <$tag {id} class={classes!($class, class.clone())}>
                    {children.clone()}
                </$tag>
            }
        }
    };
}

build_component!(H1, h1, "bp3-heading");
build_component!(H2, h2, "bp3-heading");
build_component!(H3, h3, "bp3-heading");
build_component!(H4, h4, "bp3-heading");
build_component!(H5, h5, "bp3-heading");
build_component!(H6, h6, "bp3-heading");

build_component!(Blockquote, blockquote, "bp3-blockquote");
build_component!(Code, code, "bp3-code");
build_component!(Label, label, "bp3-label");
build_component!(Pre, pre, "bp3-pre");
build_component!(Ol, ol, "bp3-ol");
build_component!(Ul, ul, "bp3-ul");
