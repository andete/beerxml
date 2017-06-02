// (c) 2017 Joost Yervante Damad

/// water information
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Water {
    /// name of the misc item
    #[serde(skip)]
    pub name: String,
    /// version of the record, should be 1
    pub version: i64,
    /// Volume of water to use in a recipe in liters
    pub amount: f64,
    /// the amount of Ca in ppm
    pub calcium: f64,
    /// the amount of HCO3 in ppm
    pub bicarbonate: f64,
    /// the amount of SO4 in ppm
    pub sulfate: f64,
    /// the amount of Cl in ppm
    pub chloride: f64,
    /// the amount of Na in ppm
    pub sodium: f64,
    /// the amount of Mg in ppm
    pub magnesium: f64,
    /// the pH of the water
    #[serde(skip_serializing_if="Option::is_none")]
    pub ph: Option<f64>,
    /// notes
    #[serde(skip_serializing_if="Option::is_none")]
    pub notes: Option<String>,
}
