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

    fn update(&mut self, _msg: Self::Message) -> bool {
        true
    }

    fn change(&mut self, props: Self::Properties) -> bool {
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
                fill={self.props.fill}
                vertical={self.props.vertical}
            >
                <HtmlSelect<Option<Sorting>>
                    options={vec![
                        (None, "Filter".to_string()),
                        (Some(Sorting::NameAscending), "Name - ascending".to_string()),
                        (Some(Sorting::NameDescending), "Name - descending".to_string()),
                        (Some(Sorting::PriceAscending), "Price - ascending".to_string()),
                        (Some(Sorting::PriceDescending), "Price - descending".to_string()),
                    ]}
                />
                <InputGroup placeholder="Find filters..." />
                <Button icon={IconName::ArrowRight} />
            </ControlGroup>
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq)]
pub enum Sorting {
    NameAscending,
    NameDescending,
    PriceAscending,
    PriceDescending,
}
