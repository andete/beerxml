// (c) 2017 Joost Yervante Damad <joost@damad.be>

use std::io::BufRead;
use std::str;

use quick_xml::reader::Reader;

use super::*;

pub fn read<B>(reader: &mut Reader<B>) -> Result<(String, MashStep)>
    where B: BufRead
{
    let mut f = MashStep::default();
    read_t(reader, b"MASH_STEP", |reader, name| {
            match name {
                b"NAME" => f.name = read_value(reader, name)?,
                b"VERSION" => f.version = read_value_t(reader, name)?,
                b"TYPE" => f.type_ = read_value_t(reader, name)?,
                b"INFUSE_AMOUNT" => f.infuse_amount = read_value_t_o(reader, name)?,
                b"STEP_TEMP" => f.step_temp = read_value_t(reader, name)?,
                b"STEP_TIME" => f.step_time = read_value_t(reader, name)?,
                b"RAMP_TIME" => f.ramp_time = read_value_t_o(reader, name)?,
                b"END_TEMP" => f.end_temp = read_value_t_o(reader, name)?,
                _ => warn!("Ignoring: {}", str::from_utf8(name)?),
            }
            Ok(())
        })
        ?;
    Ok((f.name.clone(), f))
}
