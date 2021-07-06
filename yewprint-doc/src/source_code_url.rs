use yew::prelude::*;
use yewprint::Text;

pub const BUTTON_GROUP_URL: &'static str =
    "https://github.com/yewprint/yewprint/blob/main/yewprint/src/button_group.rs";

pub struct SourceCodeUrl {
    props: SourceCodeUrlProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct SourceCodeUrlProps {
    #[prop_or_default]
    pub url: &'static str,
}

impl Component for SourceCodeUrl {
    type Message = ();
    type Properties = SourceCodeUrlProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <a
                class=classes!("bp3-text-muted")
                href=self.props.url
                target="_blank"
            >
                <Text>{"Go to the source code"}</Text>
            </a>
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_url() {
        let get_url = reqwest::blocking::get(BUTTON_GROUP_URL).unwrap();

        assert!(get_url.status().is_success())
    }
}
