// (c) 2017 Joost Yervante Damad <joost@damad.be>

use error::*;

#[derive(Debug)]
pub enum RecordSet {
    Empty,
    Fermentables(Vec<Fermentable>),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum FermentableType {
    /// grain
    Grain,
    /// sugar
    Sugar,
    /// liquid extract
    Extract,
    /// dry extract
    DryExtract,
    /// adjunct
    Adjunct,
}

impl Default for FermentableType {
    fn default() -> FermentableType {
        FermentableType::Grain
    }
}

impl FermentableType {
    pub fn make(name: &str) -> Result<FermentableType> {
        match name {
            "Grain" => Ok(FermentableType::Grain),
            "Sugar" => Ok(FermentableType::Sugar),
            "Extract" => Ok(FermentableType::Extract),
            "Dry Extract" => Ok(FermentableType::DryExtract),
            "Adjunct" => Ok(FermentableType::Adjunct),
            _ => Err(format!("unknown Fermentable Type {}", name).into()),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match *self {
            FermentableType::Grain => "Grain",
            FermentableType::Sugar => "Sugar",
            FermentableType::Extract => "Extract",
            FermentableType::DryExtract => "Dry Extract",
            FermentableType::Adjunct => "Adjunct",
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Fermentable {
    /// name of the fermentable
    pub name: String,
    /// type of fermentable
    pub type_: FermentableType,
    /// amount in kg
    pub amount: f64,
    /// percent dry yield (fine grain) for the grain,
    /// or the raw yield by weight if this is an extract adjunct or sugar
    pub yield_: f64,
    /// the color of the item in Lovibond Units (SRM for liquid extracts)
    pub color: f64,
    /// may be true if this item is normally added after the boil.
    /// The default value is false since most grains are added
    /// during the mash or boil
    pub add_after_boil: bool,
    /// country or place of origin
    #[serde(skip_serializing_if="Option::is_none")]
    pub origin: Option<String>,
    /// supplier of the grain/extract/sugar
    #[serde(skip_serializing_if="Option::is_none")]
    pub supplier: Option<String>,
    /// textual noted describing this ingredient and its use.
    /// May be multiline
    #[serde(skip_serializing_if="Option::is_none")]
    pub notes: Option<String>,
    /// percent difference between the coarse grain yield and
    /// fine grain yield.  Only appropriate for a "Grain" or
    /// "Adjunct" type, otherwise this value is ignored
    #[serde(skip_serializing_if="Option::is_none")]
    pub coarse_fine_diff: Option<f64>,
    /// percent moisture in the grain.  Only appropriate for a
    /// "Grain" or "Adjunct" type, otherwise this value is ignored
    #[serde(skip_serializing_if="Option::is_none")]
    pub moisture: Option<f64>,
    /// the diastatic power of the grain as measured in "Lintner"
    /// units. Only appropriate for a "Grain" or "Adjunct" type,
    /// otherwise this value is ignored
    #[serde(skip_serializing_if="Option::is_none")]
    pub diastatic_power: Option<f64>,
    /// the percent protein in the grain.  Only appropriate for
    /// a "Grain" or "Adjunct" type, otherwise this value is ignored
    #[serde(skip_serializing_if="Option::is_none")]
    pub proteine: Option<f64>,
    /// the recommended maximum percentage (by weight) this
    /// ingredient should represent in a batch of beer
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_in_batch: Option<f64>,
    /// true if it is recommended the grain be mashed, false
    /// if it can be steeped.  A value of true is only appropriate
    /// for a "Grain" or "Adjunct" types.  The default value is false.
    /// Note that this does NOT indicate whether the grain is mashed
    /// or not – it is only a recommendation used in recipe formulation
    pub recommended_mash: bool,
    /// for hopped extracts only - an estimate of the number of IBUs
    /// per pound of extract in a gallon of water.  To convert to IBUs
    /// we multiply this number by the "AMOUNT" field (in pounds)
    /// and divide by the number of gallons in the batch.  Based on
    /// a sixty minute boil.  Only suitable for use with an "Extract"
    /// type, otherwise this value is ignored
    #[serde(skip_serializing_if="Option::is_none")]
    pub ibu_gal_per_lb: Option<f64>,
}
