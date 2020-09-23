use yew::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Elevation {
    Level0 = 0,
    Level1,
    Level2,
    Level3,
    Level4
}

impl Elevation {
    /// Return the numeric value of the `Elevation`.
    /// ```
    /// assert_eq(Elevation::Level0.value(), 0);
    /// assert_eq(Elevation::Level4.value(), 4);
    /// ```
    pub fn value(&self) -> u8 {
        use Elevation::*;
        match self {
            Level0 => 0,
            Level1 => 1,
            Level2 => 2,
            Level3 => 3,
            Level4 => 4,
        }
    }

    /// Convert the provided value into an `Elevation` if the value is between 0 and 4.
    /// ```
    /// assert_eq(Elevation::try_from_value(1), Some(Elevation::Level1));
    /// assert_eq(Elevation::try_from_value(5), None);
    /// ```
    pub fn try_from_value(value: u8) -> Option<Self> {
        use Elevation::*;
        match value {
            0 => Some(Level0),
            1 => Some(Level1),
            2 => Some(Level2),
            3 => Some(Level3),
            4 => Some(Level4),
            _ => None
        }
    }

    /// Convert the provided value into the appropriate `Elevation`, with values greater
    /// than 4 being converted to `Elevation::Level4`.
    /// ```
    /// let level_1 = Elevation::Level1;
    /// assert_eq(Elevation::from_value_clamped(level_1 as u8 + 10), Elevation::Level4);
    /// ```
    pub fn from_value_clamped(value: u8) -> Self {
        use Elevation::*;
        match value {
            0 => Level0,
            1 => Level1,
            2 => Level2,
            3 => Level3,
            _ => Level4,
        }
    }

    fn as_css_class(&self) -> &'static str {
        use Elevation::*;
        match self {
            Level0 => ".bp3-elevation-0",
            Level1 => ".bp3-elevation-1",
            Level2 => ".bp3-elevation-2",
            Level3 => ".bp3-elevation-3",
            Level4 => ".bp3-elevation-4",
        }
    }
}

impl Default for Elevation {
    fn default() -> Self {
        Elevation::Level0
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct CardProps {
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub elevation: Elevation,
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
    #[prop_or(false)]
    pub interactive: bool,
    pub children: html::Children,
}

pub struct Card {
    props: CardProps,
    link: ComponentLink<Self>
}

impl Card {
    const BASE_CSS_CLASS: &'static str = ".bp3-card";
    const INTERACTIVE_CSS_CLASS: &'static str = ".bp3-interactive";

    fn classes(&self) -> String {
        let mut combined_classes = format!("{} {}", Self::BASE_CSS_CLASS, self.props.elevation.as_css_class());

        if self.props.interactive {
            combined_classes.push_str(" ");
            combined_classes.push_str(Self::INTERACTIVE_CSS_CLASS);
        }

        if let Some(ref classes) = self.props.class {
            combined_classes.push_str(" ");
            combined_classes.push_str(classes);
        }

        combined_classes
    }
}

impl Component for Card {
    type Message = ();
    type Properties = CardProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
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
        html!{
            <div class=self.classes() {onclick=self.props.onclick.clone().unwrap_or("")}>
                {self.props.children.clone()}
            </div>
        }
    }
}

#[cfg(feature = "dev")]
pub mod doc {
    use super::*;

    pub struct CardDoc {
        link: ComponentLink<Self>,
        elevation: Elevation,
    }

    pub enum Msg {
        AddOne,
    }

    impl Component for CardDoc {
        type Message = ();
        type Properties = ();

        fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
            CardDoc { link, elevation: Elevation::Level0 }
        }

        fn update(&mut self, _msg: Self::Message) -> ShouldRender {
            match msg {
                Msg::AddOne => self.elevation = Elevation::from_value_clamped(self.elevation as u8 + 1),
            }
            true
        }

        fn change(&mut self, _props: Self::Properties) -> ShouldRender {
            true
        }

        fn view(&self) -> Html {
            html! {
                <Card elevation={Elevation::Level0}>
                    <p>This is a card component with elevation {self.elevation as u8}.
                    Click the card to increase the elevation.</p>
                </Card>
            }
        }
    }
}