use yew::prelude::*;
use yewprint::{Switch, H5};

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
                <H5>{"Privacy settings"}</H5>
                <Switch
                    disabled=self.props.disabled
                    inline=self.props.inline
                    large=self.props.large
                    label=html!("Enabled")
                />
                <Switch
                    disabled=self.props.disabled
                    inline=self.props.inline
                    large=self.props.large
                    label=html!(<em>{"Public"}</em>)
                />
                <Switch
                    disabled=self.props.disabled
                    inline=self.props.inline
                    large=self.props.large
                    label=html!{<strong>{"Cooperative"}</strong>}
                />
                <Switch
                    disabled=self.props.disabled
                    inline=self.props.inline
                    large=self.props.large
                    label=html!(<u>{"Containing Text"}</u>)
                />
            </div>
        }
    }
}