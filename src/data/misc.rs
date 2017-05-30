// (c) 2017 Joost Yervante Damad

use std::fmt;
use std::result;

use error::*;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Misc {
    // name of the misc item
    #[serde(skip)]
    pub name:String,
    /// version of the misc format (normally 1)
    pub version:i64,
    /// misc type
    #[serde(rename="type")]
    pub type_:MiscType,
    /// misc type
    #[serde(rename="use")]
    pub use_:MiscUse,
    // time in minutes it is used
    pub time:f64,
    /// amount (liter or kg)
    pub amount: f64,
    /// if amount is in kg
    pub amount_is_weight: bool,
    /// short description what the ingredient is used for
    pub use_for: Option<String>,
    /// detailed notes
    pub notes: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub enum MiscType {
    Spice,
    Fining,
    #[serde(rename="Water Agent")]
    WaterAgent,
    Herb,
    Flavor,
    Other,
}

impl Default for MiscType {
    fn default() -> MiscType {
        MiscType::Spice
    }
}

impl MiscType {
    pub fn make(name:String) -> Result<MiscType> {
        match name.as_str() {
            "Spice" => Ok(MiscType::Spice),
            "Fining" => Ok(MiscType::Fining),
            "Water Agent" => Ok(MiscType::WaterAgent),
            "Herb" => Ok(MiscType::Herb),
            "Flavor" => Ok(MiscType::Flavor),
            "Other" => Ok(MiscType::Other),
            _ => Err(format!("Unknown misc type {}", name).into()),
        }
    }
}

impl fmt::Display for MiscType {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        let x = match *self {
            YeastType::Spice => "Spice",
            YeastType::Fining => "Fining",
            YeastType::WaterAgent => "Water Agent",
            YeastType::Herb => "Herb",
            YeastType::Flavor => "Flavor",
            YeastType::Other => "Other",
        };
        write!(f, "{}", x)
    }
}
