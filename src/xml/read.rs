// (c) 2017 Joost Yervante Damad <joost@damad.be>

use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::path::Path;
use std::str::from_utf8;

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

fn empty_option(s:String) -> Option<String> {
    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}

fn read_value_f<B>(reader: &mut Reader<B>, name: &[u8]) -> Result<f64>
    where B: BufRead
{
    let v = read_value(reader, name)?;
    let f = v.parse()?;
    Ok(f)
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
            Event::Start(ref e) if e.name() == b"NAME" => {
                f.name = read_value(reader, e.name())?;
            }
            Event::Start(ref e) if e.name() == b"TYPE" => {
                let v = read_value(reader, e.name())?;
                f.type_ = FermentableType::make(&v)?;
            }
            Event::Start(ref e) if e.name() == b"AMOUNT" => {
                f.amount = read_value_f(reader, e.name())?;
            }
            Event::Start(ref e) if e.name() == b"YIELD" => {
                f.yield_ = read_value_f(reader, e.name())?;
            }
            Event::Start(ref e) if e.name() == b"COLOR" => {
                f.color = read_value_f(reader, e.name())?;
            }
            Event::Start(ref e) if e.name() == b"ADD_AFTER_BOIL" => {
                f.add_after_boil = read_value_b(reader, e.name())?;
            }
            Event::Start(ref e) if e.name() == b"ORIGIN" => {
                f.origin = empty_option(read_value(reader, e.name())?);
            }
            Event::Start(ref e) if e.name() == b"SUPPLIER" => {
                f.supplier = empty_option(read_value(reader, e.name())?);
            }
            Event::Start(ref e) if e.name() == b"NOTES" => {
                f.notes = Some(read_value(reader, e.name())?);
            }
            Event::Start(ref e) if e.name() == b"COARSE_FINE_DIFF" => {
                f.coarse_fine_diff = Some(read_value_f(reader, e.name())?);
            }
            Event::Start(ref e) if e.name() == b"MOISTURE" => {
                f.moisture = Some(read_value_f(reader, e.name())?);
            }
            Event::Start(ref e) if e.name() == b"DIASTATIC_POWER" => {
                f.diastatic_power = Some(read_value_f(reader, e.name())?);
            }
            Event::Start(ref e) if e.name() == b"PROTEINE" => {
                f.proteine = Some(read_value_f(reader, e.name())?);
            }
            Event::Start(ref e) if e.name() == b"MAX_IN_BATCH" => {
                f.max_in_batch = Some(read_value_f(reader, e.name())?);
            }
            Event::Start(ref e) if e.name() == b"RECOMMENDED_MASH" => {
                f.recommended_mash = read_value_b(reader, e.name())?;
            }
            Event::Start(ref e) if e.name() == b"IBU_GAL_PER_LB" => {
                f.ibu_gal_per_lb = Some(read_value_f(reader, e.name())?);
            }
            Event::Start(ref e) => {
                warn!("Ignoring: {}", from_utf8(e.name()).unwrap());
            }
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
