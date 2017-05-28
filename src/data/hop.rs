// (c) 2017 Joost Yervante Damad <joost@damad.be>

use std::fmt;
use std::result;

use error::*;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Hop {
    // name of the hop
    #[serde(skip)]
    pub name:String,
    /// version of the fermentable format (normally 1)
    pub version:i64,
    /// percent alpha of hops
    pub alpha:f64,
    /// amount of hops, in kg
    pub amount:f64,
    /// use of the hop
    #[serde(rename="use")]
    pub use_:HopUse,
    /// time in minutes
    pub time:f64,
    /// notes
    #[serde(skip_serializing_if="Option::is_none")]
    pub notes:Option<String>,
    /// type of hop
    #[serde(rename="type", skip_serializing_if="Option::is_none")]
    pub type_:Option<HopType>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub form:Option<HopForm>,
    /// percent beta of hops
    #[serde(skip_serializing_if="Option::is_none")]
    pub beta:Option<f64>,
    /// Hop Stability Index - percent of hop alpha lost in 6 months of storage
    #[serde(skip_serializing_if="Option::is_none")]
    pub hsi:Option<f64>,
    /// Place of origin for the hops
    #[serde(skip_serializing_if="Option::is_none")]
    pub origin:Option<String>,
    /// Substitutes that can be used for this hop
    #[serde(skip_serializing_if="Option::is_none")]
    pub substitutes:Option<String>,
    /// Humelene leven in percent
    #[serde(skip_serializing_if="Option::is_none")]
    pub humulene:Option<f64>,
    /// Caryophyllene leven in percent
    #[serde(skip_serializing_if="Option::is_none")]
    pub caryophyllene:Option<f64>,
    /// Cohumylone leven in percent
    #[serde(skip_serializing_if="Option::is_none")]
    pub cohumulone:Option<f64>,
    /// Myrcene leven in percent
    #[serde(skip_serializing_if="Option::is_none")]
    pub myrcene:Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum HopUse {
    Aroma,
    Boil,
    #[serde(rename="Dry Hop")]
    DryHop,
    #[serde(rename="First Wort")]
    FirstWort,
    Mash,
}

impl Default for HopUse {
    fn default() -> HopUse {
        HopUse::Aroma
    }
}

impl fmt::Display for HopUse {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        let x = match *self {
            HopUse::Aroma => "Aroma",
            HopUse::Boil => "Boil",
            HopUse::DryHop => "Dry Hop",
            HopUse::FirstWort => "First Wort",
            HopUse::Mash => "Mash",
        };
        write!(f, "{}", x)
    }
}

impl HopUse {
    pub fn make(name:String) -> Result<HopUse> {
        match name.as_str() {
            "Aroma" => Ok(HopUse::Aroma),
            "Boil" => Ok(HopUse::Boil),
            "Dry Hop" => Ok(HopUse::DryHop),
            "First Wort" => Ok(HopUse::FirstWort),
            "Mash" => Ok(HopUse::Mash),
            _ => Err(format!("Unknown Hop use {}", name).into()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum HopType {
    Bittering,
    Aroma,
    Both,
}

impl fmt::Display for HopType {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        let x = match *self {
            HopType::Bittering => "Bittering",
            HopType::Aroma => "Aroma",
            HopType::Both => "Both",
        };
        write!(f, "{}", x)
    }
}

impl HopType {
    pub fn make(name:String) -> Result<HopType> {
        match name.as_str() {
            "Bittering" => Ok(HopType::Bittering),
            "Aroma" => Ok(HopType::Aroma),
            "Both" => Ok(HopType::Both),
            _ => Err(format!("Unknown hop type {}", name).into()),
        }
    }
}
        
#[derive(Serialize, Deserialize, Debug)]
pub enum HopForm {
    Pellet,
    Plug,
    Leaf,
}
        
impl fmt::Display for HopForm {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        let x = match *self {
            HopForm::Pellet => "Pellet",
            HopForm::Plug => "Plug",
            HopForm::Leaf => "Leaf",
        };
        write!(f, "{}", x)
    }
}

impl HopForm {
    pub fn make(name:String) -> Result<HopForm> {
        match name.as_str() {
            "Pellet" => Ok(HopForm::Pellet),
            "Plug" => Ok(HopForm::Plug),
            "Leaf" => Ok(HopForm::Leaf),
            _ => Err(format!("Unknown hop form {}", name).into()),
        }
    }
}
