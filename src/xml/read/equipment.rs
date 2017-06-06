// (c) 2017 Joost Yervante Damad <joost@damad.be>

use std::io::BufRead;
use std::str;

use quick_xml::reader::Reader;

use super::*;

pub fn read<B>(reader: &mut Reader<B>) -> Result<(String, Equipment)>
    where B: BufRead
{
    let mut f = Equipment::default();
    read_t(reader, b"EQUIPMENT", |reader, name| {
            match name {
                b"NAME" => f.name = read_value(reader, name)?,
                b"VERSION" => f.version = read_value_t(reader, name)?,
                b"BOIL_SIZE" => f.boil_size = read_value_t(reader, name)?,
                b"BATCH_SIZE" => f.batch_size = read_value_t(reader, name)?,
                b"TUN_VOLUME" => f.tun_volume = read_value_t_o(reader, name)?,
                b"TUN_WEIGHT" => f.tun_weight = read_value_t_o(reader, name)?,
                b"TUN_SPECIFIC_HEAT" => f.tun_specific_heat = read_value_t_o(reader, name)?,
                b"TOP_UP_WATER" => f.top_up_water = read_value_t_o(reader, name)?,
                b"EVAP_RATE" => f.evap_rate = read_value_t_o(reader, name)?,
                b"BOIL_TIME" => f.boil_time = read_value_t_o(reader, name)?,
                b"CALC_BOIL_VOLUME" => f.calc_boil_volume = read_value_b_o(reader, name)?,
                b"LAUTER_DEADSPACE" => f.lauter_deadspace = read_value_t_o(reader, name)?,
                b"TOP_UP_KETTLE" => f.top_up_kettle = read_value_t_o(reader, name)?,
                b"NOTES" => f.notes = read_value_o(reader, name)?,
                _ => warn!("Ignoring: {}", str::from_utf8(name)?),
            }
            Ok(())
        })
        ?;
    Ok((f.name.clone(), f))
}
