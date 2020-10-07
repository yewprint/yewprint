mod button_group;
mod buttons;
mod callout;
mod card;
mod collapse;
mod divider;
mod html_elements;
mod icon;
mod menu;
mod progressbar;
mod switch;
mod tag;
mod text;
#[cfg(feature = "tree")]
mod tree;

pub use button_group::*;
pub use buttons::*;
pub use callout::*;
pub use card::*;
pub use collapse::*;
pub use divider::*;
pub use html_elements::*;
pub use icon::*;
#[cfg(feature = "tree")]
pub use id_tree;
pub use menu::*;
pub use progressbar::*;
pub use switch::*;
pub use tag::*;
pub use text::*;
#[cfg(feature = "tree")]
pub use tree::*;

use std::ops::{Deref, DerefMut, Not};
use yew::virtual_dom::{Classes, Transformer, VComp};

// NOTE: this class needs to become deprecated when the feature bool_to_option lands in stable
//
//       https://github.com/rust-lang/rust/issues/64260
#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct ConditionalClass(bool);

impl Transformer<bool, ConditionalClass> for VComp {
    fn transform(value: bool) -> ConditionalClass {
        ConditionalClass(value)
    }
}

impl Transformer<ConditionalClass, bool> for VComp {
    fn transform(value: ConditionalClass) -> bool {
        value.0
    }
}

impl From<bool> for ConditionalClass {
    fn from(value: bool) -> Self {
        ConditionalClass(value)
    }
}

impl ConditionalClass {
    pub fn map_some<T>(&self, value: T) -> Option<T> {
        if self.0 {
            Some(value)
        } else {
            None
        }
    }

    pub fn and<U>(&self, optb: Option<U>) -> Option<U> {
        if self.0 {
            optb
        } else {
            None
        }
    }
}

impl Deref for ConditionalClass {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ConditionalClass {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Not for ConditionalClass {
    type Output = ConditionalClass;

    fn not(self) -> Self::Output {
        ConditionalClass(!self.0)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Intent {
    Primary,
    Success,
    Warning,
    Danger,
}

impl From<Intent> for Classes {
    fn from(intent: Intent) -> Self {
        Classes::from(intent.as_ref())
    }
}

impl AsRef<str> for Intent {
    fn as_ref(&self) -> &'static str {
        match self {
            Intent::Primary => "bp3-intent-primary",
            Intent::Success => "bp3-intent-success",
            Intent::Warning => "bp3-intent-warning",
            Intent::Danger => "bp3-intent-danger",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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
        Elevation::Level0
    }
}

impl AsRef<str> for Elevation {
    fn as_ref(&self) -> &str {
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

impl From<Elevation> for Classes {
    fn from(elevation: Elevation) -> Self {
        Classes::from(elevation.as_ref())
    }
}
