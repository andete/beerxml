// (c) 2017 Joost Yervante Damad <joost@damad.be>

use super::MashStep;

/// a mash profile
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Mash {
    /// name of the style
    #[serde(skip)]
    pub name: String,
    /// version of the style format (normally 1)
    pub version: i64,
    /// the temperature of the grain before adding it to the mash in degrees Celsius
    pub grain_temp: f64,
    /// mash steps
    pub mash_steps: Vec<MashStep>,
    /// notes
    #[serde(skip_serializing_if="Option::is_none")]
    pub notes: Option<String>,
    /// grain tun temperature – may be used to adjust the infusion temperature for equipment if the program supports it.  Measured in degrees C
    #[serde(skip_serializing_if="Option::is_none")]
    pub tun_temp: Option<f64>,
    /// temperature of the sparge water used in degrees Celsius
    #[serde(skip_serializing_if="Option::is_none")]
    pub sparge_temp: Option<f64>,
    #[serde(skip_serializing_if="Option::is_none")]
    /// PH of the sparge
    pub ph: Option<f64>,
    /// weight of the mash tun in kilograms
    #[serde(skip_serializing_if="Option::is_none")]
    pub tun_weight: Option<f64>,
    /// specific heat of the tun material in calories per gram-degree C
    #[serde(skip_serializing_if="Option::is_none")]
    pub tun_specific_heat: Option<f64>,
    /// if `true`, mash infusion and decoction calculations should take into account the temperature effects of the equipment (tun specific heat and tun weight), if `false`, the tun is assumed to be pre-heated
    #[serde(skip_serializing_if="Option::is_none")]
    pub equip_adjust: Option<bool>,
}

/// type of the mash step
#[derive(Serialize, Deserialize, Debug)]
pub enum MashStepType {
    /// adding hot water
    Infusion,
    /// heating
    Temperature,
    /// drawing of some mash for boiling
    Decoction,
}

impl Default for MashStepType {
    fn default() -> MashStepType {
        MashStepType::Infusion
    }
}
