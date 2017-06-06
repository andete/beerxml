// (c) 2017 Joost Yervante Damad <joost@damad.be>

use std::io::BufRead;
use std::str;

use quick_xml::reader::Reader;

use super::*;

pub fn read<B>(reader: &mut Reader<B>) -> Result<(String, Mash)>
    where B: BufRead
{
    let mut f = Mash::default();
    read_t(reader, b"MASH", |reader, name| {
            match name {
                b"NAME" => f.name = read_value(reader, name)?,
                b"VERSION" => f.version = read_value_t(reader, name)?,
                b"GRAIN_TEMP" => f.grain_temp = read_value_t(reader, name)?,
                b"MASH_STEPS" => f.mash_steps = read_vec(reader, "MASH_STEPS", "MASH_STEP", mash_step::read)?,
                b"NOTES" => f.notes = read_value_o(reader, name)?,
                b"TUN_TEMP" => f.tun_temp = read_value_t_o(reader, name)?,
                b"SPARGE_TEMP" => f.sparge_temp = read_value_t_o(reader, name)?,
                b"PH" => f.ph = read_value_t_o(reader, name)?,
                b"TUN_WEIGHT" => f.tun_weight = read_value_t_o(reader, name)?,
                b"TUN_SPECIFIC_HEAT" => f.tun_specific_heat = read_value_t_o(reader, name)?,
                b"EQUIP_ADJUST" => f.equip_adjust = read_value_b_o(reader, name)?,
                _ => warn!("Ignoring: {}", str::from_utf8(name)?),
            }
            Ok(())
        })
        ?;
    Ok((f.name.clone(), f))
}
