// (c) 2017 Joost Yervante Damad <joost@damad.be>

use std::io::BufRead;
use std::str;

use quick_xml::reader::Reader;

use super::*;

pub fn read_fermentable<B>(reader: &mut Reader<B>) -> Result<(String, Fermentable)>
    where B: BufRead
{
    let mut f = Fermentable::default();
    read_t(reader, b"FERMENTABLE", |reader, name| {
        match name {
            b"NAME" => f.name = read_value(reader, name)?,
            b"VERSION" => f.version = read_value_t(reader, name)?,
            b"TYPE" => f.type_ = read_value_t(reader, name)?,
            b"AMOUNT" => f.amount = read_value_t(reader, name)?,
            b"YIELD" => f.yield_ = read_value_t(reader, name)?,
            b"COLOR" => f.color = read_value_t(reader, name)?,
            b"ADD_AFTER_BOIL" => f.add_after_boil = read_value_b(reader, name)?,
            b"ORIGIN" => f.origin = read_value_o(reader, name)?,
            b"SUPPLIER" => f.supplier = read_value_o(reader, name)?,
            b"NOTES" => f.notes = read_value_o(reader, name)?,
            b"COARSE_FINE_DIFF" => f.coarse_fine_diff = Some(read_value_t(reader, name)?),
            b"MOISTURE" => f.moisture = Some(read_value_t(reader, name)?),
            b"DIASTATIC_POWER" => f.diastatic_power = Some(read_value_t(reader, name)?),
            b"PROTEIN" => f.protein = Some(read_value_t(reader, name)?),
            b"MAX_IN_BATCH" => f.max_in_batch = Some(read_value_t(reader, name)?),
            b"RECOMMEND_MASH" => f.recommend_mash = read_value_b(reader, name)?,
            b"IBU_GAL_PER_LB" => f.ibu_gal_per_lb = Some(read_value_t(reader, name)?),
            b"DISPLAY_AMOUNT" => f.display_amount = read_value_o(reader, name)?,
            b"INVENTORY" => f.inventory = read_value_o(reader, name)?,
            b"POTENTIAL" => f.potential = Some(read_value_t(reader, name)?),
            b"DISPLAY_COLOR" => f.display_color = Some(read_value_t(reader, name)?),
            _ => warn!("Ignoring: {}", str::from_utf8(name)?),
        }
        Ok(())
    })?;
    Ok((f.name.clone(), f))
}
