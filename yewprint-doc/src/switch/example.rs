use yew::prelude::*;
use yewprint::{Label, Switch};

pub struct Example {
    props: ExampleProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ExampleProps {
    pub disabled: bool,
    pub inline: bool,
    pub large: bool,
    pub align_right: bool,
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
                <Label>{"Privacy settings"}</Label>
                <Switch
                    disabled=self.props.disabled
                    inline=self.props.inline
                    large=self.props.large
                    label=html!{<strong>{"Enabled"}</strong>}
                    align_right=self.props.align_right
                />
                <Switch
                    disabled=self.props.disabled
                    inline=self.props.inline
                    large=self.props.large
                    label=html!{<em>{"Public"}</em>}
                    align_right=self.props.align_right
                />
                <Switch
                    disabled=self.props.disabled
                    inline=self.props.inline
                    large=self.props.large
                    checked=true
                    label=html!{<u>{"Cooperative"}</u>}
                    align_right=self.props.align_right
                />
                <Switch
                    disabled=self.props.disabled
                    inline=self.props.inline
                    large=self.props.large
                    label=html!{"Containing Text"}
                    inner_label_checked={"on".to_string()}
                    inner_label={"off".to_string()}
                    align_right=self.props.align_right
                />
            </div>
        }
    }
}
