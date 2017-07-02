// (c) 2017 Joost Yervante Damad <joost@damad.be>

/// a yeast
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Yeast {
    /// name of the hop
    #[serde(skip)]
    pub name: String,
    /// version of the yeast format (normally 1)
    pub version: i64,
    /// yeast type
    #[serde(rename="type")]
    pub type_: YeastType,
    /// yeast form
    pub form: YeastForm,
    /// amount (liter or kg)
    pub amount: f64,
    /// if amount is in kg
    pub amount_is_weight: bool,
    /// name of the producer
    #[serde(skip_serializing_if="Option::is_none")]
    pub laboratory: Option<String>,
    /// manufacturer product id
    #[serde(skip_serializing_if="Option::is_none")]
    pub product_id: Option<String>,
    /// minimum recommended temperature for fermenting this yeast strain in degrees Celsius
    #[serde(skip_serializing_if="Option::is_none")]
    pub min_temperature: Option<f64>,
    /// maximum recommended temperature for fermenting this yeast strain in Celsius
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_temperature: Option<f64>,
    /// yeast flocculation
    #[serde(skip_serializing_if="Option::is_none")]
    pub flocculation: Option<YeastFlocculation>,
    /// attenuation of the yeast in percent
    #[serde(skip_serializing_if="Option::is_none")]
    pub attenuation: Option<f64>,
    /// notes
    #[serde(skip_serializing_if="Option::is_none")]
    pub notes: Option<String>,
    /// styles or types of beer this yeast strain is best suited for
    #[serde(skip_serializing_if="Option::is_none")]
    pub best_for: Option<String>,
    /// number of times this yeast has been reused as a harvested culture.  This number should be zero if this is a product directly from the manufacturer
    #[serde(skip_serializing_if="Option::is_none")]
    pub times_cultured: Option<i64>,
    /// recommended of times this yeast can be reused (recultured from a previous batch)
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_reuse: Option<i64>,
    /// flag denoting that this yeast was added for a secondary (or later) fermentation as opposed to the primary fermentation.  Useful if one uses two or more yeast strains for a single brew (eg: Lambic).  Default value is false
    pub add_to_secondary: bool,
    /// amount
    #[serde(skip_serializing_if="Option::is_none")]
    pub display_amount: Option<String>,
    /// recommended minimum temperature
    #[serde(skip_serializing_if="Option::is_none")]
    pub display_min_temp: Option<String>,
    /// recommended maximum temperature
    #[serde(skip_serializing_if="Option::is_none")]
    pub display_max_temp: Option<String>,
    /// inventory
    #[serde(skip_serializing_if="Option::is_none")]
    pub inventory: Option<String>,
    /// date the culture was made
    #[serde(skip_serializing_if="Option::is_none")]
    pub culture_date: Option<String>,
}

/// the type of a yeast
#[derive(ToString, EnumString, Serialize, Deserialize, Debug)]
pub enum YeastType {
    /// ale (top-fermenting) yeast
    Ale,
    /// lager (bottom-fermenting) yeast
    Lager,
    /// wheat yeast
    Wheat,
    /// wine yeast
    Wine,
    /// champage yeast
    Champagne,
}

impl Default for YeastType {
    fn default() -> YeastType {
        YeastType::Ale
    }
}

/// the form of the yeast
#[derive(ToString, EnumString, Serialize, Deserialize, Debug)]
pub enum YeastForm {
    /// liquid yeast
    Liquid,
    /// dry yeast
    Dry,
    /// slated yeast
    Slate,
    /// a yeast culture
    Culture,
}

impl Default for YeastForm {
    fn default() -> YeastForm {
        YeastForm::Liquid
    }
}

/// flocculation of a yeast
#[derive(ToString, EnumString, Serialize, Deserialize, Debug)]
pub enum YeastFlocculation {
    /// low flocculation
    Low,
    /// medium flocculation
    Medium,
    /// high flocculation
    High,
    /// very flocculation
    #[serde(rename = "Very High")]
    #[strum(serialize = "Very High")]
    VeryHigh,
}

impl Default for YeastFlocculation {
    fn default() -> YeastFlocculation {
        YeastFlocculation::Low
    }
}
