// (c) 2017 Joost Yervante Damad <joost@damad.be>

use std::io::{BufRead, BufReader};
use std::fs::File;
use std::path::Path;
use std::str::from_utf8;

use quick_xml::reader::Reader;
use quick_xml::events::Event;

use data::*;
use error::*;

fn read_value<B>(reader:&mut Reader<B>, name:&[u8]) -> Result<String>
    where B:BufRead
{
    let mut buf = vec![];
    let mut txt = String::new();
    loop {
        match reader.read_event(&mut buf)? {
            Event::Text(ref e) => {
                txt = e.unescape_and_decode(&reader)?;
            },
            Event::End(ref e) if e.name() == name => {
                break
            },
            Event::Eof => break,
            _ => (),
        }
        buf.clear();
    }
    Ok(txt)
}

fn read_value_f<B>(reader:&mut Reader<B>, name:&[u8]) -> Result<f64>
    where B:BufRead
{
    let v = read_value(reader, name)?;
    let f = v.parse()?;
    Ok(f)
}

fn read_fermentable<B>(reader:&mut Reader<B>) -> Result<Fermentable>
    where B:BufRead
{
    let mut buf = vec![];
    let mut f = Fermentable::default();
    loop {
        match reader.read_event(&mut buf)? {
            Event::Start(ref e) if e.name() == b"NAME" => {
                f.name = read_value(reader, e.name())?;
            },
            Event::Start(ref e) if e.name() == b"TYPE" => {
                let v = read_value(reader, e.name())?;
                f.type_ = FermentableType::make(&v)?;
            },
            Event::Start(ref e) if e.name() == b"AMOUNT" => {
                f.amount = read_value_f(reader, e.name())?;
            },
            Event::Start(ref e) => {
                warn!("Ignoring: {}", from_utf8(e.name()).unwrap());
            },
            Event::End(ref e) if e.name() == b"FERMENTABLE" => {
                break
            },
            Event::Eof => break,
            _ => (),
        }
        buf.clear();
    }
    Ok(f)
}

fn read_fermentables<B>(reader:&mut Reader<B>) -> Result<Vec<Fermentable>>
    where B:BufRead
{
    let mut buf = vec![];
    let mut fermentables = vec![];
    loop {
        match reader.read_event(&mut buf)? {
            Event::Start(ref e) if e.name() == b"FERMENTABLE" => {
                let fermentable = read_fermentable(reader)?;
                fermentables.push(fermentable);
            },
            Event::Start(ref e) => {
                warn!("Ignoring: {}", from_utf8(e.name()).unwrap());
            },
            Event::End(ref e) if e.name() == b"FERMENTABLES" => {
                break
            },
            Event::Eof => break,
            _ => (),
        }
        buf.clear();
    }
    Ok(fermentables)
}

pub fn read<B>(reader:B) -> Result<RecordSet>
    where B:BufRead
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
            },
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

pub fn read_file(filename:&Path) -> Result<RecordSet> {
    let f = File::open(filename)?;
    let reader = BufReader::new(f);
    read(reader)
}
