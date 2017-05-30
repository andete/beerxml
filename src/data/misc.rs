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
    pub notes: Option<String>,
    /// display string for time
    pub display_time: Option<String>,
    /// display string for amount
    pub display_amount: Option<String>,
    /// inventory
    pub inventory: Option<String>,
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
            MiscType::Spice => "Spice",
            MiscType::Fining => "Fining",
            MiscType::WaterAgent => "Water Agent",
            MiscType::Herb => "Herb",
            MiscType::Flavor => "Flavor",
            MiscType::Other => "Other",
        };
        write!(f, "{}", x)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum MiscUse {
    Boil,
    Mash,
    Primary,
    Secondary,
    Bottling,
}

impl Default for MiscUse {
    fn default() -> MiscUse {
        MiscUse::Boil
    }
}

impl MiscUse {
    pub fn make(name:String) -> Result<MiscUse> {
        match name.as_str() {
            "Boil" => Ok(MiscUse::Boil),
            "Mash" => Ok(MiscUse::Mash),
            "Primary" => Ok(MiscUse::Primary),
            "Secondary" => Ok(MiscUse::Secondary),
            "Bottling" => Ok(MiscUse::Bottling),
            _ => Err(format!("Unknown misc use {}", name).into()),
        }
    }
}

impl fmt::Display for MiscUse {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        let x = match *self {
            MiscUse::Boil => "Boil",
            MiscUse::Mash => "Mash",
            MiscUse::Primary => "Primary",
            MiscUse::Secondary => "Secondary",
            MiscUse::Bottling => "Bottling",
        };
        write!(f, "{}", x)
    }
}
