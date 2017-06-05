// (c) 2017 Joost Yervante Damad <joost@damad.be>

use std::io::BufRead;
use std::str;

use quick_xml::reader::Reader;

use super::*;

pub fn read<B>(reader: &mut Reader<B>) -> Result<(String, Misc)>
    where B: BufRead
{
    let mut f = Misc::default();
    read_t(reader, b"MISC", |reader, name| {
            match name {
                b"NAME" => f.name = read_value(reader, name)?,
                b"VERSION" => f.version = read_value_t(reader, name)?,
                b"TYPE" => f.type_ = read_value_t(reader, name)?,
                b"USE" => f.use_ = read_value_t(reader, name)?,
                b"TIME" => f.time = read_value_t(reader, name)?,
                b"AMOUNT" => f.amount = read_value_t(reader, name)?,
                b"AMOUNT_IS_WEIGHT" => f.amount_is_weight = read_value_b(reader, name)?,
                b"USE_FOR" => f.use_for = read_value_o(reader, name)?,
                b"NOTES" => f.notes = read_value_o(reader, name)?,
                b"DISPLAY_AMOUNT" => f.display_amount = read_value_o(reader, name)?,
                b"DISPLAY_TIME" => f.display_time = read_value_o(reader, name)?,
                b"INVENTORY" => f.inventory = read_value_o(reader, name)?,  
                _ => warn!("Ignoring: {}", str::from_utf8(name)?),
            }
            Ok(())
        })
        ?;
    Ok((f.name.clone(), f))
}
