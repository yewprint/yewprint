use yew::prelude::*;
use yewprint::{Button, ControlGroup, HtmlSelect, IconName, InputGroup};

pub struct Example {
    props: ExampleProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ExampleProps {
    pub fill: bool,
    pub vertical: bool,
}

impl Component for Example {
    type Message = ();
    type Properties = ExampleProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Example { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
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
            <ControlGroup
                fill=self.props.fill
                vertical=self.props.vertical
            >
                <HtmlSelect<Sorting>
                    options={vec![
                        (Sorting::Filter, "Filter".to_string()),
                        (Sorting::NameAscending, "Name-Ascending".to_string()),
                        (Sorting::NameDescending, "Name-Descending".to_string()),
                        (Sorting::PriceAscending, "Price-Ascending".to_string()),
                        (Sorting::PriceDescending, "Price-Descending".to_string()),
                    ]}
                />
                <InputGroup placeholder={"Find filters..."} />
                <Button icon=IconName::ArrowRight>
                    {""}
                </Button>
            </ControlGroup>
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq)]
pub enum Sorting {
    Filter,
    NameAscending,
    NameDescending,
    PriceAscending,
    PriceDescending,
}
