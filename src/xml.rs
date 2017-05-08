// (c) 2017 Joost Yervante Damad <joost@damad.be>

use std::io::{BufRead, BufReader};
use std::fs::File;
use std::path::Path;
use std::str::from_utf8;

use quick_xml::reader::Reader;
use quick_xml::events::Event;

use data::*;
use error::*;

pub fn read<B:BufRead>(reader:B) -> Result<RecordSet> {
    let mut reader = Reader::from_reader(reader);
    reader.trim_text(true);
    let mut buf = Vec::new();
    loop {
        match reader.read_event(&mut buf)? {
            Event::Start(ref e) if e.name() == b"FERMENTABLES" => {
                info!("Fermentables.");
            },
            Event::Start(ref e) => {
                warn!("Ignoring: {}", from_utf8(e.name()).unwrap());
            }
            Event::Eof => break,
            _ => (),
        }
        buf.clear();
    }
    Ok(RecordSet::Empty) // TODO
}

pub fn read_file(filename:&Path) -> Result<RecordSet> {
    let f = File::open(filename)?;
    let reader = BufReader::new(f);
    read(reader)
}
