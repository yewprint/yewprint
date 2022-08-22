use yew::prelude::*;
use yewprint::{Button, ControlGroup, HtmlSelect, IconName, InputGroup};

#[derive(Clone, PartialEq, Eq, Properties)]
pub struct ExampleProps {
    pub fill: bool,
    pub vertical: bool,
}

#[function_component(Example)]
pub fn example(props: &ExampleProps) -> Html {
    html! {
        <ControlGroup
            fill={props.fill}
            vertical={props.vertical}
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

#[derive(Debug, Clone, Copy, Hash, PartialEq)]
pub enum Sorting {
    NameAscending,
    NameDescending,
    PriceAscending,
    PriceDescending,
}
