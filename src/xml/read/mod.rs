// (c) 2017 Joost Yervante Damad <joost@damad.be>

use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::path::Path;
use std::str;
use std::fmt;

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
          T: str::FromStr,
          Error: ::std::convert::From<<T as str::FromStr>::Err>
{
    let v = read_value(reader, name)?;
    let res = v.parse::<T>()?;
    Ok(res)
}

fn read_value_t_o<B, T>(reader: &mut Reader<B>, name: &[u8]) -> Result<Option<T>>
    where B: BufRead,
          T: str::FromStr,
          Error: ::std::convert::From<<T as str::FromStr>::Err>,
          <T as str::FromStr>::Err: fmt::Debug
{
    let v = read_value(reader, name)?;
    let res = match v.parse::<T>() {
        Ok(res) => Some(res),
        Err(e) => {
            warn!("Ignoring parse error for optional field {} with value {}: {:?}",
                  str::from_utf8(name)?,
                  v,
                  e);
            None
        }
    };
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

fn read_value_b_o<B>(reader: &mut Reader<B>, name: &[u8]) -> Result<Option<bool>>
    where B: BufRead
{
    let v = read_value(reader, name)?;
    match v.as_str() {
        "TRUE" => Ok(Some(true)),
        _ => Ok(None),
    }
}

fn read_t<B, F>(reader: &mut Reader<B>, name: &[u8], mut do_element: F) -> Result<()>
    where B: BufRead,
          F: FnMut(&mut Reader<B>, &[u8]) -> Result<()>
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
                warn!("Ignoring: {}", str::from_utf8(e.name()).unwrap());
            }
            Event::End(ref e) if e.name() == elements_name => break,
            Event::Eof => break,
            _ => (),
        }
        buf.clear();
    }
    Ok(map)
}

fn read_vec<B, F, T>(reader: &mut Reader<B>,
                     elements_name: &'static str,
                     element_name: &'static str,
                     read_element: F)
                     -> Result<Vec<T>>
    where B: BufRead,
          F: Fn(&mut Reader<B>) -> Result<(String, T)>
{
    let element_name = element_name.as_bytes();
    let elements_name = elements_name.as_bytes();
    let mut buf = vec![];
    let mut res = vec![];
    loop {
        match reader.read_event(&mut buf)? {
            Event::Start(ref e) if e.name() == element_name => {
                let (_, element) = read_element(reader)?;
                res.push(element);
            }
            Event::Start(ref e) => {
                warn!("Ignoring: {}", str::from_utf8(e.name()).unwrap());
            }
            Event::End(ref e) if e.name() == elements_name => break,
            Event::Eof => break,
            _ => (),
        }
        buf.clear();
    }
    Ok(res)
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
                let f = read_map(&mut reader,
                                 "FERMENTABLES",
                                 "FERMENTABLE",
                                 fermentable::read)
                    ?;
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
                let f = read_map(&mut reader, "RECIPES", "RECIPE", recipe::read)?;
                rs = RecordSet::Recipes(f);
                // info!("Recipes: {:?}", f);
            }
            Event::Start(ref e) if e.name() == b"STYLES" => {
                let f = read_map(&mut reader, "STYLES", "STYLE", style::read)?;
                rs = RecordSet::Styles(f);
                // info!("Styles: {:?}", f);
            }
            Event::Start(ref e) if e.name() == b"MASHS" => {
                let f = read_map(&mut reader, "MASHS", "MASH", mash::read)?;
                rs = RecordSet::Mashs(f);
                // info!("Mashs: {:?}", f);
            }
            Event::Start(ref e) => {
                read_ignore(&mut reader, e.name())?;
            }
            Event::Eof => break,
            _ => (),
        }
        buf.clear();
    }
    Ok(rs)
}

fn read_ignore<B>(reader: &mut Reader<B>, name: &[u8]) -> Result<()>
    where B: BufRead
{
    warn!("Ignoring: {}", str::from_utf8(name)?);
    let mut buf = vec![];
    loop {
        match reader.read_event(&mut buf)? {
            Event::Start(ref e) => {
                warn!("Ignoring: {}", str::from_utf8(e.name())?);
            }
            Event::End(ref e) if e.name() == name => break,
            Event::Eof => break,
            _ => (),
        }
        buf.clear();
    }
    Ok(())
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
mod recipe;
mod style;
mod mash;
mod mash_step;
