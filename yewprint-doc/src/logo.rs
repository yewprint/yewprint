use yew::prelude::*;

pub struct Logo {
    props: LogoProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct LogoProps {
    #[prop_or_default]
    pub class: Classes,
}

impl Component for Logo {
    type Message = ();
    type Properties = LogoProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        todo!()
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        if props != self.props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        crate::include_raw_html!("logo.svg", &self.props.class.to_string())
    }
}
