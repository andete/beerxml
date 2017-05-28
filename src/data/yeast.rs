// (c) 2017 Joost Yervante Damad <joost@damad.be>

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Yeast {
    // name of the hop
    #[serde(skip)]
    pub name:String,
    /// version of the yeast format (normally 1)
    pub version:i64,
}
