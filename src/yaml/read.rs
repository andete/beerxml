// (c) 2017 Joost Yervante Damad <joost@damad.be>

use std::io::{BufRead, BufReader};
use std::fs::File;
use std::path::Path;

use serde_yaml;

use data::*;
use error::*;

pub fn read<B>(reader: B) -> Result<RecordSet>
    where B: BufRead
{
    let rs = serde_yaml::from_reader(reader)?;
    Ok(rs)
}

pub fn read_file(filename: &Path) -> Result<RecordSet> {
    let f = File::open(filename)?;
    let reader = BufReader::new(f);
    read(reader)
}
