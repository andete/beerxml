// (c) 2017 Joost Yervante Damad <joost@damad.be>

use std::io::BufRead;
use std::str;

use quick_xml::reader::Reader;

use super::*;

pub fn read_hop<B>(reader: &mut Reader<B>) -> Result<(String, Hop)>
    where B: BufRead
{
    let mut f = Hop::default();
    read_t(reader, b"HOP", |reader, name| {
        match name {
            b"NAME" => f.name = read_value(reader, name)?,
            b"VERSION" => f.version = read_value_t(reader, name)?,
            b"ALPHA" => f.alpha = read_value_t(reader, name)?,
            b"AMOUNT" => f.amount = read_value_t(reader, name)?,
            b"USE" => f.use_ = read_value_t(reader, name)?,
            b"TIME" => f.time = read_value_t(reader, name)?,
            b"NOTES" => f.notes = read_value_o(reader, name)?,
            b"TYPE" => f.type_ = Some(read_value_t(reader, name)?),
            b"FORM" => f.form = Some(read_value_t(reader, name)?),
            b"BETA" => f.beta = Some(read_value_t(reader, name)?),
            b"HSI" => f.hsi = Some(read_value_t(reader, name)?),
            b"ORIGIN" => f.origin = read_value_o(reader, name)?,
            b"SUBSTITUTES" => f.substitutes = read_value_o(reader, name)?,
            b"HUMULENE" => f.humulene = Some(read_value_t(reader, name)?),
            b"CARYOPHYLLENE" => f.caryophyllene = Some(read_value_t(reader, name)?),
            b"COHUMULONE" => f.cohumulone = Some(read_value_t(reader, name)?),
            b"MYRCENE" => f.myrcene = Some(read_value_t(reader, name)?),
            _ => warn!("Ignoring: {}", str::from_utf8(name)?),
        }
        Ok(())
    })?;
    Ok((f.name.clone(), f))
}
