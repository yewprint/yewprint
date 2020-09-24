use yew::Classes;

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
    /// assert_eq(Elevation::Level1.above(), Elevation::Level2);
    /// assert_eq(Elevation::Level4.above(), Elevation::Level4);
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
    /// assert_eq(Elevation::Level3.below(), Elevation::Level2);
    /// assert_eq(Elevation::Level0.below(), Elevation::Level0);
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
