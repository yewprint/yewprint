use yew::prelude::*;
use yewprint::{Checkbox, Label};

pub struct Example {
    props: ExampleProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ExampleProps {
    pub disabled: bool,
    pub inline: bool,
    pub large: bool,
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
            <div>
                <Label>{"Assign responsability"}</Label>
                <Checkbox
                    disabled=self.props.disabled
                    inline=self.props.inline
                    large=self.props.large
                    label=html!("Gilad Gray")
                />
                <Checkbox
                    disabled=self.props.disabled
                    inline=self.props.inline
                    large=self.props.large
                    label=html!("Jason Killian")
                />
                <Checkbox
                    disabled=self.props.disabled
                    inline=self.props.inline
                    large=self.props.large
                    label=html!("Antoine Llorca")
                />
            </div>
        }
    }
}
