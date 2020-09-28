mod buttons;
mod callout;
mod collapse;
mod html_elements;
mod icon;
mod menu;
mod progressbar;
mod switch;
mod tree;

pub use buttons::*;
pub use callout::*;
pub use collapse::*;
pub use html_elements::*;
pub use icon::*;
pub use id_tree;
pub use menu::*;
pub use progressbar::*;
pub use switch::*;
pub use tree::*;

use std::ops::{Deref, DerefMut};
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
