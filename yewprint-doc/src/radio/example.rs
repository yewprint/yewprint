use yew::prelude::*;
use yewprint::{Radio, RadioGroup};

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
                <RadioGroup
                    disabled=self.props.disabled
                    inline=self.props.inline
                    label=html!("Determine lunch")
                >
                    <Radio
                        label=html!("Soup")
                    />
                    <Radio
                        label=html!("Salad")
                    />
                    <Radio
                        label=html!("Sandwich")
                    />
                </RadioGroup>
            </div>
        }
    }
}
