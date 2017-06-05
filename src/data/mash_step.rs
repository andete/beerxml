// (c) 2017 Joost Yervante Damad <joost@damad.be>

/// a mash step is an internal record used within a mash profile to denote a separate step in a multi-step mash.  A mash step is not intended for use outside of a mash profile
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct MashStep {
    /// name of the style
    #[serde(skip)]
    pub name: String,
    /// version of the style format (normally 1)
    pub version: i64,
    /// may be “Infusion”, “Temperature” or “Decoction” depending on the type of step.  Infusion denotes adding hot water, Temperature denotes heating with an outside heat source, and decoction denotes drawing off some mash for boiling
    #[serde(rename="type")]
    pub type_: MashStepType,
    /// the volume of water in liters to infuse in this step.  Required only for infusion steps, though one may also add water for temperature mash steps.  One should not have an infusion amount for decoction steps
    #[serde(skip_serializing_if="Option::is_none")]
    pub infuse_amount: Option<f64>,
    /// target temperature for this step in degrees Celsius
    pub step_temp: f64,
    /// number of minutes to spend at this step – i.e. the amount of time we are to hold this particular step temperature
    pub step_time: f64,
    /// time in minutes to achieve the desired step temperature – useful particularly for temperature mashes where it may take some time to achieve the step temperature
    #[serde(skip_serializing_if="Option::is_none")]
    pub ramp_time: Option<f64>,
    /// the temperature you can expect the mash to fall to after a long mash step, measured in degrees Celsius
    #[serde(skip_serializing_if="Option::is_none")]
    pub end_temp: Option<f64>,
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
