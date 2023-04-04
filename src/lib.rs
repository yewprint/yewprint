#![allow(
    clippy::redundant_closure,
    clippy::needless_update,
    clippy::inconsistent_struct_constructor,
    clippy::type_complexity,
    clippy::derive_partial_eq_without_eq,
    clippy::uninlined_format_args,
    clippy::derivable_impls
)]

mod button_group;
mod buttons;
mod callout;
mod card;
mod checkbox;
mod collapse;
mod control_group;
mod divider;
mod html_elements;
mod html_select;
mod icon;
mod input_group;
mod menu;
mod numeric_input;
mod overlay;
mod panel_stack;
mod portal;
mod progress_bar;
mod radio;
mod radio_group;
mod slider;
mod spinner;
mod switch;
mod tabs;
mod tag;
mod text;
mod text_area;
#[cfg(feature = "tree")]
mod tree;

pub use button_group::*;
pub use buttons::*;
pub use callout::*;
pub use card::*;
pub use checkbox::*;
pub use collapse::*;
pub use control_group::*;
pub use divider::*;
pub use html_elements::*;
pub use html_select::*;
pub use icon::*;
#[cfg(feature = "tree")]
pub use id_tree;
pub use input_group::*;
pub use menu::*;
pub use numeric_input::*;
pub use overlay::*;
pub use panel_stack::*;
pub use portal::*;
pub use progress_bar::*;
pub use radio::*;
pub use radio_group::*;
pub use slider::*;
pub use spinner::*;
pub use switch::*;
pub use tabs::*;
pub use tag::*;
pub use text::*;
pub use text_area::*;
#[cfg(feature = "tree")]
pub use tree::*;

use implicit_clone::ImplicitClone;
use yew::Classes;

// See https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/buttons
#[allow(dead_code)]
const MOUSE_EVENT_BUTTONS_NONE: u16 = 0;
#[allow(dead_code)]
const MOUSE_EVENT_BUTTONS_PRIMARY: u16 = 1;
#[allow(dead_code)]
const MOUSE_EVENT_BUTTONS_SECONDARY: u16 = 2;
#[allow(dead_code)]
const MOUSE_EVENT_BUTTONS_AUXILIARY: u16 = 4;
#[allow(dead_code)]
const MOUSE_EVENT_BUTTONS_FOURTH: u16 = 8;
#[allow(dead_code)]
const MOUSE_EVENT_BUTTONS_FIFTH: u16 = 16;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Intent {
    Primary,
    Success,
    Warning,
    Danger,
}

impl ImplicitClone for Intent {}

impl From<Intent> for Classes {
    fn from(intent: Intent) -> Self {
        use Intent::*;
        Classes::from(match intent {
            Primary => "bp3-intent-primary",
            Success => "bp3-intent-success",
            Warning => "bp3-intent-warning",
            Danger => "bp3-intent-danger",
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Elevation {
    Level0,
    Level1,
    Level2,
    Level3,
    Level4,
}

impl Elevation {
    /// Return the next highest `Elevation`.
    /// ```
    /// # use yewprint::Elevation;
    /// assert_eq!(Elevation::Level1.above(), Elevation::Level2);
    /// assert_eq!(Elevation::Level4.above(), Elevation::Level4);
    /// ```
    pub fn above(&self) -> Self {
        use Elevation::*;
        match self {
            Level0 => Level1,
            Level1 => Level2,
            Level2 => Level3,
            Level3 => Level4,
            Level4 => Level4,
        }
    }

    /// Return the next lowest `Elevation`.
    /// ```
    /// # use yewprint::Elevation;
    /// assert_eq!(Elevation::Level3.below(), Elevation::Level2);
    /// assert_eq!(Elevation::Level0.below(), Elevation::Level0);
    /// ```
    pub fn below(&self) -> Self {
        use Elevation::*;
        match self {
            Level0 => Level0,
            Level1 => Level0,
            Level2 => Level1,
            Level3 => Level2,
            Level4 => Level3,
        }
    }
}

impl Default for Elevation {
    fn default() -> Self {
        Self::Level0
    }
}

impl ImplicitClone for Elevation {}

impl From<Elevation> for Classes {
    fn from(elevation: Elevation) -> Self {
        use Elevation::*;
        Self::from(match elevation {
            Level0 => "bp3-elevation-0",
            Level1 => "bp3-elevation-1",
            Level2 => "bp3-elevation-2",
            Level3 => "bp3-elevation-3",
            Level4 => "bp3-elevation-4",
        })
    }
}

impl From<&Elevation> for Classes {
    fn from(elevation: &Elevation) -> Self {
        Self::from(*elevation)
    }
}
