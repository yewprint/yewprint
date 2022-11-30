use yew::prelude::*;
use yewprint::{Button, ControlGroup, HtmlSelect, IconName, InputGroup};

#[derive(Clone, PartialEq, Properties)]
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
                    (None, "Filter".into()),
                    (Some(Sorting::NameAscending), "Name - ascending".into()),
                    (Some(Sorting::NameDescending), "Name - descending".into()),
                    (Some(Sorting::PriceAscending), "Price - ascending".into()),
                    (Some(Sorting::PriceDescending), "Price - descending".into()),
                ]}
            />
            <InputGroup placeholder="Find filters..." />
            <Button icon={IconName::ArrowRight} />
        </ControlGroup>
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Sorting {
    NameAscending,
    NameDescending,
    PriceAscending,
    PriceDescending,
}
