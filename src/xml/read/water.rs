// (c) 2017 Joost Yervante Damad <joost@damad.be>

use std::io::BufRead;
use std::str;

use quick_xml::reader::Reader;

use super::*;

pub fn read<B>(reader: &mut Reader<B>) -> Result<(String, Water)>
    where B: BufRead
{
    let mut f = Water::default();
    read_t(reader, b"WATER", |reader, name| {
            match name {
                b"NAME" => f.name = read_value(reader, name)?,
                b"VERSION" => f.version = read_value_t(reader, name)?,
                b"AMOUNT" => f.amount = read_value_t(reader, name)?,
                b"CALCIUM" => f.calcium = read_value_t(reader, name)?,
                b"BICARBONATE" => f.bicarbonate = read_value_t(reader, name)?,
                b"SULFATE" => f.sulfate = read_value_t(reader, name)?,
                b"CHLORIDE" => f.chloride = read_value_t(reader, name)?,
                b"SODIUM" => f.sodium = read_value_t(reader, name)?,
                b"MAGNESIUM" => f.magnesium = read_value_t(reader, name)?,
                b"PH" => f.ph = Some(read_value_t(reader, name)?),
                b"NOTES" => f.notes = read_value_o(reader, name)?,
                _ => warn!("Ignoring: {}", str::from_utf8(name)?),
            }
            Ok(())
        })
        ?;
    Ok((f.name.clone(), f))
}
