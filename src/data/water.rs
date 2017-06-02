// (c) 2017 Joost Yervante Damad

//use std::fmt;
//use std::result;

//use error::*;

/// water information
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Water {
    /// name of the misc item
    #[serde(skip)]
    pub name:String,
}
