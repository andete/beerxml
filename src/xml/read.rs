// (c) 2017 Joost Yervante Damad <joost@damad.be>

use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::path::Path;
use std::str::{from_utf8,FromStr};

use quick_xml::reader::Reader;
use quick_xml::events::Event;

use data::*;
use error::*;

fn read_value<B>(reader: &mut Reader<B>, name: &[u8]) -> Result<String>
    where B: BufRead
{
    let mut buf = vec![];
    let mut txt = String::new();
    loop {
        match reader.read_event(&mut buf)? {
            Event::Text(ref e) => {
                txt = e.unescape_and_decode(reader)?;
            }
            Event::End(ref e) if e.name() == name => break,
            Event::Eof => break,
            _ => (),
        }
        buf.clear();
    }
    Ok(txt)
}

fn read_value_o<B>(reader: &mut Reader<B>, name: &[u8]) -> Result<Option<String>>
    where B: BufRead
{
    let v = read_value(reader, name)?;
    if v.is_empty() {
        return Ok(None);
    }
    Ok(Some(v))
}

fn read_value_m<B,F,T>(reader: &mut Reader<B>, name: &[u8], make:F) -> Result<T>
    where B: BufRead, F:Fn(String) -> Result<T>
{
    let v = read_value(reader, name)?;
    make(v)
}

fn read_value_t<B,T>(reader: &mut Reader<B>, name: &[u8]) -> Result<T>
    where B: BufRead,
          T: FromStr,
          Error: ::std::convert::From<<T as FromStr>::Err>
{
    let s = read_value(reader, name)?;
    let x = s.parse()?;
    Ok(x)
}

fn read_value_b<B>(reader: &mut Reader<B>, name: &[u8]) -> Result<bool>
    where B: BufRead
{
    let v = read_value(reader, name)?;
    match v.as_str() {
        "TRUE" => Ok(true),
        "FALSE" => Ok(false),
        x => Err(format!("unknown boolean: {}", x).into()),
    }
}

fn read_fermentable<B>(reader: &mut Reader<B>) -> Result<Fermentable>
    where B: BufRead
{
    let mut buf = vec![];
    let mut f = Fermentable::default();
    loop {
        match reader.read_event(&mut buf)? {
            Event::Start(ref e) => {
                let name = e.name();
                match name {
                    b"NAME" => f.name = read_value(reader, name)?,
                    b"VERSION" => f.version = read_value_t(reader, name)?,
                    b"TYPE" => f.type_ = read_value_m(reader, name, FermentableType::make)?,
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
                    _ => warn!("Ignoring: {}", from_utf8(e.name()).unwrap()),
                }
            },
            Event::End(ref e) if e.name() == b"FERMENTABLE" => break,
            Event::Eof => break,
            _ => (),
        }
        buf.clear();
    }
    Ok(f)
}

fn read_fermentables<B>(reader: &mut Reader<B>) -> Result<HashMap<String, Fermentable>>
    where B: BufRead
{
    let mut buf = vec![];
    let mut fermentables = HashMap::new();
    loop {
        match reader.read_event(&mut buf)? {
            Event::Start(ref e) if e.name() == b"FERMENTABLE" => {
                let fermentable = read_fermentable(reader)?;
                fermentables.insert(fermentable.name.clone(), fermentable);
            }
            Event::Start(ref e) => {
                warn!("Ignoring: {}", from_utf8(e.name()).unwrap());
            }
            Event::End(ref e) if e.name() == b"FERMENTABLES" => break,
            Event::Eof => break,
            _ => (),
        }
        buf.clear();
    }
    Ok(fermentables)
}

pub fn read<B>(reader: B) -> Result<RecordSet>
    where B: BufRead
{
    let mut reader = Reader::from_reader(reader);
    reader.trim_text(true);
    let mut buf = vec![];
    let mut rs = RecordSet::Empty;
    loop {
        match reader.read_event(&mut buf)? {
            Event::Start(ref e) if e.name() == b"FERMENTABLES" => {
                let f = read_fermentables(&mut reader)?;
                rs = RecordSet::Fermentables(f);
                // info!("Fermentables.: {:?}", f);
            }
            Event::Start(ref e) => {
                warn!("Ignoring: {}", from_utf8(e.name()).unwrap());
            }
            Event::Eof => break,
            _ => (),
        }
        buf.clear();
    }
    Ok(rs)
}

pub fn read_file(filename: &Path) -> Result<RecordSet> {
    let f = File::open(filename)?;
    let reader = BufReader::new(f);
    read(reader)
}
