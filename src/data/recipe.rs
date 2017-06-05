// (c) 2017 Joost Yervante Damad <joost@damad.be>

// WIP
use std::fmt;
use std::result;
use std::str::FromStr;
use std::collections::HashMap;

use error::*;
use super::*;

/// a beer recipe
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Recipe {
    /// name of the recipe
    #[serde(skip)]
    pub name: String,
    /// version of the recipe format (normally 1)
    pub version: i64,
    /// type of the recipe
    #[serde(rename="type")]
    pub type_: RecipeType,
    // /// style of the recipe
    // pub style:Style,
    // /// an optional equipment record
    // pub equiment:Option<Equipment>,
    /// name of the brewer
    pub brewer: String,
    /// optional name of the assistant brewer
    pub asst_brewer: Option<String>,
    /// target size of the finished batch in liters
    pub batch_size: f64,
    /// starting size for the main boil of the wort in liters
    pub boil_size: f64,
    /// total boil time in minutes
    pub boil_time: f64,
    /// the percent brewhouse efficiency to be used for estimating the starting gravity of the beer; not required for “Extract” recipes, but is required for “Partial Mash” and “All Grain” recipes
    pub efficiency: Option<f64>,
    // /// mash profile
    // pub mash: Option<Mash>,
    /// notes
    #[serde(skip_serializing_if="Option::is_none")]
    pub notes: Option<String>,
    /// original gravity (pre-fermentation)
    #[serde(skip_serializing_if="Option::is_none")]
    pub og: Option<f64>,
    /// final gravity of the finished beer
    #[serde(skip_serializing_if="Option::is_none")]
    pub fg: Option<f64>,

    // TABLES NEED TO BE LAST TO WORK WITH TOML
    /// hop ingredient records
    pub hops: HashMap<String, Hop>,
    /// fermentable ingredient records
    pub fermentables: HashMap<String, Fermentable>,
    /// misc ingredient records
    pub miscs: HashMap<String, Misc>,
    /// yeast ingredient records
    pub yeasts: HashMap<String, Yeast>,
    /// water info records
    pub waters: HashMap<String, Water>, // TODO: complete
}

/// recipe type
#[derive(Serialize, Deserialize, Debug)]
pub enum RecipeType {
    /// extract recipe
    Extract,
    /// paritial mash recipe
    #[serde(rename = "Partial Mash")]
    PartialMash,
    /// all-grain recipe
    #[serde(rename = "All Grain")]
    AllGrain,
}

impl Default for RecipeType {
    fn default() -> RecipeType {
        RecipeType::AllGrain
    }
}

impl FromStr for RecipeType {
    type Err = Error;
    fn from_str(name: &str) -> Result<RecipeType> {
        match name {
            "Extract" => Ok(RecipeType::Extract),
            "Partial Mash" => Ok(RecipeType::PartialMash),
            "All Grain" => Ok(RecipeType::AllGrain),
            _ => Err(ErrorKind::ParseError("RecipeType".into(), name.into()).into()),
        }
    }
}

impl fmt::Display for RecipeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        let x = match *self {
            RecipeType::Extract => "Extract",
            RecipeType::PartialMash => "Partial Mash",
            RecipeType::AllGrain => "All Grain",
        };
        write!(f, "{}", x)
    }
}
