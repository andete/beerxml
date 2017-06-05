// (c) 2017 Joost Yervante Damad <joost@damad.be>

/// equipment
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Equipment {
    /// name of the style
    #[serde(skip)]
    pub name: String,
    /// version of the style format (normally 1)
    pub version: i64,
    /// pre-boil volume in liters used in this particular instance for this equipment setup.  Note that this may be a calculated value depending on the `calc_boil_volume` parameter
    pub boil_size: f64,
    /// targer volume in liters of the batch at the start of fermentation
    pub batch_size: f64,
    /// volume of the mash tun in liters.  This parameter can be used to calculate if a particular mash and grain profile will fit in the mash tun.  It may also be used for thermal calculations in the case of a partially full mash tun
    #[serde(skip_serializing_if="Option::is_none")]
    pub tun_volume: Option<f64>,
    /// weight of the mash tun in kilograms.  Used primarily to calculate the thermal parameters of the mash tun – in conjunction with the volume and specific heat
    #[serde(skip_serializing_if="Option::is_none")]
    pub tun_weight: Option<f64>,
    /// specific heat of the mash tun which is usually a function of the material it is made of.  Typical ranges are 0.1-0.25 for metal and 0.2-0.5 for plastic materials , in Cal/(gram-deg C)
    #[serde(skip_serializing_if="Option::is_none")]
    pub tun_specific_heat: Option<f64>,
    /// amount of top up water in liters normally added just prior to starting fermentation.  Usually used for extract brewing
    #[serde(skip_serializing_if="Option::is_none")]
    pub top_up_water: Option<f64>,
    /// percentage of wort lost to evaporation per hour of the boil
    #[serde(skip_serializing_if="Option::is_none")]
    pub evap_rate: Option<f64>,
    /// normal amount of time in hours one boils for this equipment setup.  This can be used with the evaporation rate to calculate the evaporation loss
    #[serde(skip_serializing_if="Option::is_none")]
    pub boil_time: Option<f64>,
    /// flag denoting that the program should calculate the boil size.  Flag may be `true` or `false`.  If `true`, then `boil_size` = (`batch_size` – `top_up_water` – `trub_chiller_loss`) * (1+`boil_time` * `evap_rate` )  If set then the boil size should match this value
    #[serde(skip_serializing_if="Option::is_none")]
    pub calc_boil_volume: Option<bool>,
    /// amount lost in liters to the lauter tun and equipment associated with the lautering process
    #[serde(skip_serializing_if="Option::is_none")]
    pub lauter_deadspace: Option<f64>,
    /// amount in liters normally added to the boil kettle before the boil
    #[serde(skip_serializing_if="Option::is_none")]
    pub top_up_kettle: Option<f64>,
    /// notes
    #[serde(skip_serializing_if="Option::is_none")]
    pub notes: Option<String>,
}
