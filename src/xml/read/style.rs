// (c) 2017 Joost Yervante Damad <joost@damad.be>

use std::io::BufRead;
use std::str;

use quick_xml::reader::Reader;

use super::*;

pub fn read<B>(reader: &mut Reader<B>) -> Result<(String, Style)>
    where B: BufRead
{
    let mut f = Style::default();
    read_t(reader, b"STYLE", |reader, name| {
            match name {
                b"NAME" => f.name = read_value(reader, name)?,
                b"VERSION" => f.version = read_value_t(reader, name)?,
                b"CATEGORY" => f.category = read_value(reader, name)?,
                b"CATEGORY_NUMBERR" => f.category_number = read_value(reader, name)?,
                b"STYLE_LETTER" => f.style_letter = read_value(reader, name)?,
                b"STYLE_GUIDE" => f.style_guide = read_value(reader, name)?,
                b"TYPE" => f.type_ = read_value_t(reader, name)?,
                b"OG_MIN" => f.og_min = read_value_t(reader, name)?,
                b"OG_MAX" => f.og_max = read_value_t(reader, name)?,
                b"FG_MIN" => f.fg_min = read_value_t(reader, name)?,
                b"FG_MAX" => f.fg_max = read_value_t(reader, name)?,
                b"UBI_MIN" => f.ibu_min = read_value_t(reader, name)?,
                b"UBI_MAX" => f.ibu_max = read_value_t(reader, name)?,
                b"COLOR_MIN" => f.color_min = read_value_t(reader, name)?,
                b"COLOR_MAX" => f.color_max = read_value_t(reader, name)?,
                b"CARB_MIN" => f.carb_min = read_value_t_o(reader, name)?,
                b"CARB_MAX" => f.carb_max = read_value_t_o(reader, name)?,
                b"ABV_MIN" => f.abv_min = read_value_t_o(reader, name)?,
                b"ABV_MAX" => f.abv_max = read_value_t_o(reader, name)?,
                b"NOTES" => f.notes = read_value_o(reader, name)?,
                b"PROFILE" => f.profile = read_value_o(reader, name)?,
                b"INGREDIENTS" => f.ingredients = read_value_o(reader, name)?,
                b"EXAMPLES" => f.examples = read_value_o(reader, name)?,
                _ => warn!("Ignoring: {}", str::from_utf8(name)?),
            }
            Ok(())
        })
        ?;
    Ok((f.name.clone(), f))
}
