// (c) 2017 Joost Yervante Damad <joost@damad.be>

use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::path::Path;
use std::str::{from_utf8, FromStr};

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

fn read_value_t<B, T>(reader: &mut Reader<B>, name: &[u8]) -> Result<T>
    where B: BufRead,
          T: FromStr,
          Error: ::std::convert::From<<T as FromStr>::Err>
{
    let v = read_value(reader, name)?;
    let res = v.parse::<T>()?;
    Ok(res)
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

fn read_t<B,F>(reader: &mut Reader<B>, name: &[u8], mut do_element:F) -> Result<()>
    where B: BufRead,
          F:FnMut(&mut Reader<B>, &[u8]) -> Result<()>
{
    let mut buf = vec![];
    loop {
        match reader.read_event(&mut buf)? {
            Event::Start(ref e) => {
                let name = e.name();
                do_element(reader, name)?;
            }
            Event::End(ref e) if e.name() == name => break,
            Event::Eof => break,
            _ => (),
        }
        buf.clear();
    }
    Ok(())
}

fn read_recipe<B>(_reader: &mut Reader<B>) -> Result<(String, Recipe)>
    where B: BufRead
{
    Ok(("".into(), Recipe::default()))
        /* TODO
    let mut buf = vec![];
    let mut f = Recipe::default();
    loop {
        match reader.read_event(&mut buf)? {
            Event::Start(ref e) => {
                let name = e.name();
                match name {
                    b"NAME" => f.name = read_value(reader, name)?,
                    b"VERSION" => f.version = read_value_t(reader, name)?,
                    b"AMOUNT" => f.amount = read_value_t(reader, name)?,
                    b"CALCIUM" => f.calcium = read_value_t(reader, name)?,
                    b"BICARBONATE" => f.bicarbonate = read_value_t(reader, name)?,
                    b"SULFATE" => f.sulfate = read_value_t(reader, name)?,
                    b"CHLORIDE" => f.chloride = read_value_t(reader, name)?,
                    b"SODIUM" => f.sodium = read_value_t(reader, name)?,
                    b"MAGNESIUM" => f.magnesium = read_value_t(reader, name)?,
                    b"PH" => f.ph = Some(read_value_t(reader, name)?),
                    b"NOTES" => f.notes = read_value_o(reader, name)?,
                    _ => warn!("Ignoring: {}", from_utf8(e.name()).unwrap()),
                }
            }
            Event::End(ref e) if e.name() == b"RECIPE" => break,
            Event::Eof => break,
            _ => (),
        }
        buf.clear();
    }
    Ok((f.name.clone(), f))*/
}

fn read_map<B, F, T>(reader: &mut Reader<B>,
                     elements_name: &'static str,
                     element_name: &'static str,
                     read_element: F)
                     -> Result<HashMap<String, T>>
    where B: BufRead,
          F: Fn(&mut Reader<B>) -> Result<(String, T)>
{
    let element_name = element_name.as_bytes();
    let elements_name = elements_name.as_bytes();
    let mut buf = vec![];
    let mut map = HashMap::new();
    loop {
        match reader.read_event(&mut buf)? {
            Event::Start(ref e) if e.name() == element_name => {
                let (name, element) = read_element(reader)?;
                map.insert(name, element);
            }
            Event::Start(ref e) => {
                warn!("Ignoring: {}", from_utf8(e.name()).unwrap());
            }
            Event::End(ref e) if e.name() == elements_name => break,
            Event::Eof => break,
            _ => (),
        }
        buf.clear();
    }
    Ok(map)
}

/// try to read a `RecordSet` from a `reader`
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
                let f = read_map(&mut reader, "FERMENTABLES", "FERMENTABLE", fermentable::read)?;
                rs = RecordSet::Fermentables(f);
                // info!("Fermentables: {:?}", f);
            }
            Event::Start(ref e) if e.name() == b"HOPS" => {
                let f = read_map(&mut reader, "HOPS", "HOP", hop::read)?;
                rs = RecordSet::Hops(f);
                // info!("Hops: {:?}", f);
            }
            Event::Start(ref e) if e.name() == b"YEASTS" => {
                let f = read_map(&mut reader, "YEASTS", "YEAST", yeast::read)?;
                rs = RecordSet::Yeasts(f);
                // info!("Yeasts: {:?}", f);
            }
            Event::Start(ref e) if e.name() == b"MISCS" => {
                let f = read_map(&mut reader, "MISCS", "MISC", misc::read)?;
                rs = RecordSet::Miscs(f);
                // info!("Miscs: {:?}", f);
            }
            Event::Start(ref e) if e.name() == b"WATERS" => {
                let f = read_map(&mut reader, "WATERS", "WATER", water::read)?;
                rs = RecordSet::Waters(f);
                // info!("Waters: {:?}", f);
            }
             Event::Start(ref e) if e.name() == b"RECIPES" => {
                let f = read_map(&mut reader, "RECIPES", "RECIPE", read_recipe)?;
                rs = RecordSet::Recipes(f);
                // info!("Recipes: {:?}", f);
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

/// try to read a `RecordSet` from a file
pub fn read_file(filename: &Path) -> Result<RecordSet> {
    let f = File::open(filename)?;
    let reader = BufReader::new(f);
    read(reader)
}

mod fermentable;
mod hop;
mod yeast;
mod misc;
mod water;
