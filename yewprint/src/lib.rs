mod button_group;
mod buttons;
mod callout;
mod card;
mod collapse;
mod divider;
mod html_elements;
mod html_select;
mod icon;
mod input_group;
mod menu;
mod numeric_input;
mod progressbar;
mod switch;
mod tabs;
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
pub use html_select::*;
pub use icon::*;
#[cfg(feature = "tree")]
pub use id_tree;
pub use input_group::*;
pub use menu::*;
pub use numeric_input::*;
pub use progressbar::*;
pub use switch::*;
pub use tabs::*;
pub use tag::*;
pub use text::*;
#[cfg(feature = "tree")]
pub use tree::*;

use yew::virtual_dom::Classes;

#[macro_export]
macro_rules! if_html {
    (let $pat:pat = $cond:expr => $($body:tt)+) => {
        if let $pat = $cond {
            html!($($body)+)
        } else {
            html!()
        }
    };
    ($cond:expr => $($body:tt)+) => {
        if $cond {
            html($(body)+)
        } else {
            html!()
        }
    };
}

#[derive(Debug, Copy, Clone, PartialEq, Hash)]
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
