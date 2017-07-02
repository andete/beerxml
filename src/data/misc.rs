// (c) 2017 Joost Yervante Damad

/// misc beer ingredient
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Misc {
    /// name of the misc item
    #[serde(skip)]
    pub name: String,
    /// version of the misc format (normally 1)
    pub version: i64,
    /// misc type
    #[serde(rename="type")]
    pub type_: MiscType,
    /// misc type
    #[serde(rename="use")]
    pub use_: MiscUse,
    /// time in minutes it is used
    pub time: f64,
    /// amount (liter or kg)
    pub amount: f64,
    /// if amount is in kg
    pub amount_is_weight: bool,
    /// short description what the ingredient is used for
    pub use_for: Option<String>,
    /// detailed notes
    pub notes: Option<String>,
    /// display string for time
    pub display_time: Option<String>,
    /// display string for amount
    pub display_amount: Option<String>,
    /// inventory
    pub inventory: Option<String>,
}

/// misc type
#[derive(ToString, EnumString, Serialize, Deserialize, Debug)]
pub enum MiscType {
    /// a spice
    Spice,
    /// a fining agent
    Fining,
    /// a water agent
    #[serde(rename="Water Agent")]
    #[strum(serialize="Water Agent")]
    WaterAgent,
    /// a herb
    Herb,
    /// a flavor
    Flavor,
    /// an other misc item
    Other,
}

impl Default for MiscType {
    fn default() -> MiscType {
        MiscType::Spice
    }
}

/// usage for a misc item
#[derive(ToString, EnumString, Serialize, Deserialize, Debug)]
pub enum MiscUse {
    /// use in boil
    Boil,
    /// use in mash
    Mash,
    /// use in primary fermenter
    Primary,
    /// use in secondary fermenter
    Secondary,
    /// use at bottling
    Bottling,
}

impl Default for MiscUse {
    fn default() -> MiscUse {
        MiscUse::Boil
    }
}
