// (c) 2017 Joost Yervante Damad <joost@damad.be>

use std::fmt;
use std::result;
use std::str::FromStr;

use error::*;

/// a hop
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Hop {
    /// name of the hop
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
    /// form of the hop
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

/// the usage of the hop
#[derive(Serialize, Deserialize, Debug)]
pub enum HopUse {
    /// aroma hop usage
    Aroma,
    /// boil hop usage
    Boil,
    /// dry-hop hop usage
    #[serde(rename="Dry Hop")]
    DryHop,
    /// first wort hop usage
    #[serde(rename="First Wort")]
    FirstWort,
    /// mash hop usage
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

impl FromStr for HopUse {
    type Err = Error;
    fn from_str(name:&str) -> Result<HopUse> {
        match name {
            "Aroma" => Ok(HopUse::Aroma),
            "Boil" => Ok(HopUse::Boil),
            "Dry Hop" => Ok(HopUse::DryHop),
            "First Wort" => Ok(HopUse::FirstWort),
            "Mash" => Ok(HopUse::Mash),
            _ => Err(ErrorKind::ParseError("HopUse".into(), name.into()).into())
        }
    }
}

/// the type of a hop
#[derive(Serialize, Deserialize, Debug)]
pub enum HopType {
    /// a bittering hop
    Bittering,
    /// an aroma hop
    Aroma,
    /// a dual-purpose hop
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

impl FromStr for HopType {
    type Err = Error;
    fn from_str(name:&str) -> Result<HopType> {
        match name {
            "Bittering" => Ok(HopType::Bittering),
            "Aroma" => Ok(HopType::Aroma),
            "Both" => Ok(HopType::Both),
            _ => Err(ErrorKind::ParseError("HopType".into(), name.into()).into())
        }
    }
}

/// the form of a hop
#[derive(Serialize, Deserialize, Debug)]
pub enum HopForm {
    /// pellet hop
    Pellet,
    /// hop plugs
    Plug,
    /// leaf hop
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

impl FromStr for HopForm {
    type Err = Error;
    /// create a `HopForm` from a `String`
    fn from_str(name:&str) -> Result<HopForm> {
        match name {
            "Pellet" => Ok(HopForm::Pellet),
            "Plug" => Ok(HopForm::Plug),
            "Leaf" => Ok(HopForm::Leaf),
            _ => Err(ErrorKind::ParseError("HopForm".into(), name.into()).into())
        }
    }
}
