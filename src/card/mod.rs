use yew::prelude::*;
use yew::Classes;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Elevation {
    Level0 = 0,
    Level1,
    Level2,
    Level3,
    Level4,
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
            _ => None,
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
            Level0 => "bp3-elevation-0",
            Level1 => "bp3-elevation-1",
            Level2 => "bp3-elevation-2",
            Level3 => "bp3-elevation-3",
            Level4 => "bp3-elevation-4",
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
    pub onclick: Callback<MouseEvent>,
    #[prop_or(false)]
    pub interactive: bool,
    pub children: html::Children,
}

pub struct Card {
    props: CardProps,
}

impl Component for Card {
    type Message = ();
    type Properties = CardProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
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
        let mut class = Classes::from("bp3-card");
        class.push(self.props.elevation.as_css_class());
        if self.props.interactive {
            class.push("bp3-interactive");
        }
        class.extend(&self.props.class);

        html! {
            <div class=class onclick={self.props.onclick.clone()}>
                {self.props.children.clone()}
            </div>
        }
    }
}

#[cfg(feature = "doc")]
pub mod doc {
    use super::*;

    pub struct CardDoc {}

    impl Component for CardDoc {
        type Message = ();
        type Properties = ();

        fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
            CardDoc {}
        }

        fn update(&mut self, _msg: Self::Message) -> ShouldRender {
            true
        }

        fn change(&mut self, _props: Self::Properties) -> ShouldRender {
            true
        }

        fn view(&self) -> Html {
            let source = crate::include_example!("example.rs");

            html! {
                <div>
                    <h1>{"Card"}</h1>
                    <div>{source}</div>
                </div>
            }
        }
    }
}
