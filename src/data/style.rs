// (c) 2017 Joost Yervante Damad <joost@damad.be>

use std::fmt;
use std::result;
use std::str::FromStr;

use error::*;

/// a beer style
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Style {
    /// name of the style
    #[serde(skip)]
    pub name: String,
    /// version of the style format (normally 1)
    pub version: i64,
    /// Category that this style belongs to – usually associated with a group of styles such as “English Ales” or “Amercian Lagers”
    pub category: String,
    /// number or identifier associated with this style category.  For example in the BJCP style guide, the “American Lager” category has a category number of “1”
    pub category_number: String,
    /// specific style number or subcategory letter associated with this particular style.  For example in the BJCP style guide, an American Standard Lager would be style letter “A” under the main category.  Letters should be upper case
    pub style_letter: String,
    /// name of the style guide that this particular style or category belongs to.  For example “BJCP” might denote the BJCP style guide, and “AHA” would be used for the AHA style guide
    pub style_guide: String,
    /// may be “Lager”, “Ale”, “Mead”, “Wheat”, “Mixed” or “Cider”.  Defines the type of beverage associated with this category
    #[serde(rename="type")]
    pub type_: StyleType,
    /// minimum specific gravity as measured relative to water.  For example “1.040” might be a reasonable minimum for a Pale Ale
    pub og_min: f64,
    /// maximum specific gravity as measured relative to water
    pub og_max: f64,
    /// minimum final gravity as measured relative to water
    pub fg_min: f64,
    /// maximum final gravity as measured relative to water
    pub fg_max: f64,
    /// recommended minimum bitterness for this style as measured in International Bitterness Units (IBUs)
    pub ibu_min: f64,
    /// recommended maximum bitterness for this style as measured in International Bitterness Units (IBUs)
    pub ibu_max: f64,
    /// minimum recommended color in SRM
    pub color_min: f64,
    /// maximum recommended color in SRM
    pub color_max: f64,
    /// minimum recommended carbonation for this style in volumes of CO2
    #[serde(skip_serializing_if="Option::is_none")]
    pub carb_min: Option<f64>,
    /// maximum recommended carbonation for this style in volumes of CO2
    #[serde(skip_serializing_if="Option::is_none")]
    pub carb_max: Option<f64>,
    /// minimum recommended alcohol by volume as a percentage
    #[serde(skip_serializing_if="Option::is_none")]
    pub abv_min: Option<f64>,
    /// maximum recommended alcohol by volume as a percentage
    #[serde(skip_serializing_if="Option::is_none")]
    pub abv_max: Option<f64>,
    /// description of the style, history
    #[serde(skip_serializing_if="Option::is_none")]
    pub notes: Option<String>,
    /// flavor and aroma profile for this style
    #[serde(skip_serializing_if="Option::is_none")]
    pub profile: Option<String>,
    /// suggested ingredients for this style
    #[serde(skip_serializing_if="Option::is_none")]
    pub ingredients: Option<String>,
    /// example beers of this style
    #[serde(skip_serializing_if="Option::is_none")]
    pub examples: Option<String>,
}

/// defines the type of beverage associated with this style
#[derive(Serialize, Deserialize, Debug)]
pub enum StyleType {
    /// Lager beer
    Lager,
    /// Ale beer
    Ale,
    /// Mead
    Mead,
    /// Wheat beer
    Wheat,
    /// Mixed style
    Mixed,
    /// Cider
    Cider,
}

impl Default for StyleType {
    fn default() -> StyleType {
        StyleType::Ale
    }
}

impl FromStr for StyleType {
    type Err = Error;
    fn from_str(name: &str) -> Result<StyleType> {
        match name {
            "Lager" => Ok(StyleType::Lager),
            "Ale" => Ok(StyleType::Ale),
            "Mead" => Ok(StyleType::Mead),
            "Wheat" => Ok(StyleType::Wheat),
            "Mixed" => Ok(StyleType::Mixed),
            "Cider" => Ok(StyleType::Cider),
            _ => Err(ErrorKind::ParseError("StyleType".into(), name.into()).into()),
        }
    }
}

impl fmt::Display for StyleType {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        let x = match *self {
            StyleType::Lager => "Lager",
            StyleType::Ale => "Ale",
            StyleType::Mead => "Mead",
            StyleType::Wheat => "Wheat",
            StyleType::Mixed => "Mixed",
            StyleType::Cider => "Cider",
        };
        write!(f, "{}", x)
    }
}
