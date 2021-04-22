use yew::prelude::*;
use yewprint::{BreadcrumbItem, Breadcrumbs, Card, IconName};

pub struct Example {
    link: ComponentLink<Self>,
    props: ExampleProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ExampleProps {
    pub width: u64,

}

impl Component for Example {
    type Message = ();
    type Properties = ExampleProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Example {
            link,
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <Card 
                style=format!("width: {}%", self.props.width)>
                    <Breadcrumbs
                    >
                    <BreadcrumbItem text="All files".to_string() icon=IconName::FolderClose/>
                    <BreadcrumbItem text="Users".to_string() icon=IconName::FolderClose/>
                    <BreadcrumbItem text="Janet".to_string() icon=IconName::FolderClose/>
                    <BreadcrumbItem text="Photos".to_string() icon=IconName::FolderClose href="#".to_string()/>
                    <BreadcrumbItem text="Wednesday".to_string() icon=IconName::FolderClose href="#".to_string()/>
                    <BreadcrumbItem text="image.jpg".to_string() icon=IconName::Document current=true/>
                    </Breadcrumbs>
            </Card>
        }
    }
}
