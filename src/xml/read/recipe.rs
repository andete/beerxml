// (c) 2017 Joost Yervante Damad <joost@damad.be>

use std::io::BufRead;
use quick_xml::reader::Reader;

use super::*;

pub fn read<B>(reader: &mut Reader<B>) -> Result<(String, Recipe)>
    where B: BufRead
{
    let mut f = Recipe::default();
    read_t(reader, b"RECIPE", |reader, name| {
            match name {
                b"NAME" => f.name = read_value(reader, name)?,
                b"VERSION" => f.version = read_value_t(reader, name)?,
                b"TYPE" => f.type_ = read_value_t(reader, name)?,
                b"BREWER" => f.brewer = read_value_t(reader, name)?,
                b"ASST_BREWER" => f.asst_brewer = read_value_o(reader, name)?,
                b"BATCH_SIZE" => f.batch_size = read_value_t(reader, name)?,
                b"BOIL_SIZE" => f.boil_size = read_value_t(reader, name)?,
                b"BOIL_TIME" => f.boil_time = read_value_t(reader, name)?,
                b"EFFICIENCY" => f.efficiency = Some(read_value_t(reader, name)?),
                b"HOPS" => f.hops = read_map(reader, "HOPS", "HOP", hop::read)?,
                b"FERMENTABLES" => {
                    f.fermentables =
                        read_map(reader, "FERMENTABLES", "FERMENTABLE", fermentable::read)?
                }
                b"MISCS" => f.miscs = read_map(reader, "MISCS", "MISC", misc::read)?,
                b"YEASTS" => f.yeasts = read_map(reader, "YEASTS", "YEAST", yeast::read)?,
                b"WATERS" => f.waters = read_map(reader, "WATERS", "WATER", water::read)?,
                b"NOTES" => f.notes = read_value_o(reader, name)?,
                b"OG" => f.og = Some(read_value_t(reader, name)?),
                b"FG" => f.fg = Some(read_value_t(reader, name)?),
                _ => read_ignore(reader, name)?,
            }
            Ok(())
        })
        ?;
    Ok((f.name.clone(), f))
}
