use web_sys::Element;
use yew::prelude::*;

#[derive(Debug, Default)]
pub struct Portal {
    container_element: Option<Element>,
    node_ref: NodeRef,
}

#[derive(Debug, PartialEq, Properties)]
pub struct PortalProps {
    #[prop_or_default]
    pub children: Children,
}

impl Component for Portal {
    type Properties = PortalProps;
    type Message = ();

    fn create(_: &Context<Self>) -> Self {
        Self::default()
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let contents = self
            .container_element
            .clone()
            .map(|container_element| {
                create_portal(
                    html! {
                        {for ctx.props().children.iter()}
                    },
                    container_element,
                )
            })
            .unwrap_or_default();

        html! {
            <div ref={self.node_ref.clone()}>
                {contents}
            </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let container_element = gloo::utils::document().create_element("div").unwrap();
            container_element.set_class_name("bp3-portal");
            gloo::utils::body()
                .append_child(&container_element)
                .unwrap();
            self.container_element.replace(container_element);
            ctx.link().send_message(());
        }
    }
}
