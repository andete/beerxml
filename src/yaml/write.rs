// (c) 2017 Joost Yervante Damad <joost@damad.be>

use std::fs::File;
use std::io::Write;
use std::path::Path;

use serde_yaml;

use data::*;
use error::*;

/// try to write a `RecordSet` as yaml to a `writer`
pub fn write<T>(writer: &mut T, set: &RecordSet) -> Result<()>
    where T: Write
{
    serde_yaml::to_writer(writer, set)?;
    Ok(())
}

/// try to write a `RecordSet` as yaml to a file
pub fn write_file(filename: &Path, set: &RecordSet) -> Result<()> {
    let mut f = File::create(filename)?;
    write(&mut f, set)
}
