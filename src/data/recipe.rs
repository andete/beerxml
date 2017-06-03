// (c) 2017 Joost Yervante Damad <joost@damad.be>


/// a beer recipe
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Recipe {
    /// name of the recipe
    #[serde(skip)]
    pub name: String,
    /// version of the recipe format (normally 1)
    pub version: i64,
}
