// (c) 2017 Joost Yervante Damad <joost@damad.be>

use std::collections::HashMap;

pub use self::fermentable::*;
pub use self::hop::*;
pub use self::yeast::*;

#[derive(Serialize, Deserialize, Debug)]
pub enum RecordSet {
    Empty,
    Fermentables(HashMap<String, Fermentable>),
    Hops(HashMap<String, Hop>),
    Yeasts(HashMap<String, Yeast>),
}

mod fermentable;
mod hop;
mod yeast;
