// (c) 2017 Joost Yervante Damad <joost@damad.be>

use std::collections::HashMap;

pub use self::equipment::*;
pub use self::fermentable::*;
pub use self::hop::*;
pub use self::mash::*;
pub use self::mash_step::*;
pub use self::misc::*;
pub use self::recipe::*;
pub use self::style::*;
pub use self::water::*;
pub use self::yeast::*;

/// a record set
#[derive(Serialize, Deserialize, Debug)]
pub enum RecordSet {
    /// the default empty record set
    Empty,
    /// a set of named fermentables
    Fermentables(HashMap<String, Fermentable>),
    /// a set of named hops
    Hops(HashMap<String, Hop>),
    /// a set of named miscelaneous items
    Miscs(HashMap<String, Misc>),
    /// a set of recipes
    Recipes(HashMap<String, Recipe>),
    /// a set of named water profiles
    Waters(HashMap<String, Water>),
    /// a set of named yeasts
    Yeasts(HashMap<String, Yeast>),
    /// a set of named styles
    Styles(HashMap<String, Style>),
}

mod equipment;
mod fermentable;
mod hop;
mod mash;
mod mash_step;
mod misc;
mod recipe;
mod style;
mod water;
mod yeast;
