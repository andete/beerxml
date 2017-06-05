// (c) 2017 Joost Yervante Damad <joost@damad.be>

use std::io::BufRead;
use std::str;

use quick_xml::reader::Reader;

use super::*;

pub fn read<B>(reader: &mut Reader<B>) -> Result<(String, Yeast)>
    where B: BufRead
{
    let mut f = Yeast::default();
    read_t(reader, b"YEAST", |reader, name| {
        match name {
             b"NAME" => f.name = read_value(reader, name)?,
            b"VERSION" => f.version = read_value_t(reader, name)?,
            b"TYPE" => f.type_ = read_value_t(reader, name)?,
            b"FORM" => f.form = read_value_t(reader, name)?,
            b"AMOUNT" => f.amount = read_value_t(reader, name)?,
            b"AMOUNT_IS_WEIGHT" => f.amount_is_weight = read_value_b(reader, name)?,
            b"LABORATORY" => f.laboratory = read_value_o(reader, name)?,
            b"PRODUCT_ID" => f.product_id = read_value_o(reader, name)?,
            b"MIN_TEMPERATURE" => f.min_temperature = Some(read_value_t(reader, name)?),
            b"MAX_TEMPERATURE" => f.max_temperature = Some(read_value_t(reader, name)?),
            b"FLOCCULATION" => f.flocculation = Some(read_value_t(reader, name)?),
            b"ATTENUATION" => f.attenuation = Some(read_value_t(reader, name)?),
            b"NOTES" => f.notes = read_value_o(reader, name)?,
            b"BEST_FOR" => f.best_for = read_value_o(reader, name)?,
            b"TIMES_CULTURED" => f.times_cultured = Some(read_value_t(reader, name)?),
            b"MAX_REUSE" => f.max_reuse = Some(read_value_t(reader, name)?),
            b"ADD_TO_SECONDARY" => f.add_to_secondary = read_value_b(reader, name)?,
            b"DISPLAY_AMOUNT" => f.display_amount = read_value_o(reader, name)?,
            b"DISP_MIN_TEMP" => f.display_min_temp = read_value_o(reader, name)?,
            b"DISP_MAX_TEMP" => f.display_max_temp = read_value_o(reader, name)?,
            b"INVENTORY" => f.inventory = read_value_o(reader, name)?,
            b"CULTURE_DATE" => f.culture_date = read_value_o(reader, name)?,
            _ => warn!("Ignoring: {}", str::from_utf8(name)?),
        }
        Ok(())
    })?;
    Ok((f.name.clone(), f))
}
