// (c) 2017 Joost Yervante Damad <joost@damad.be>

use std::io::Write;
use std::fs::File;
use std::path::Path;

use data::*;
use error::*;

fn indent<T>(writer: &mut T, offset: usize) -> Result<()>
    where T: Write
{
    for _ in 0..offset {
        writer.write(b"  ")?;
    }
    Ok(())
}

fn write_fermentable<T>(writer: &mut T, v: &Fermentable, offset: usize) -> Result<()>
    where T: Write
{
    indent(writer, offset)?;
    writer.write(b"<FERMENTABLE>\n")?;
    indent(writer, offset)?;
    writer.write(b"</FERMENTABLE>\n")?;
    Ok(())
}

fn write_fermentables<T>(writer: &mut T, v: &Vec<Fermentable>, offset: usize) -> Result<()>
    where T: Write
{
    indent(writer, offset)?;
    writer.write(b"<FERMENTABLES>\n")?;
    for f in v {
        write_fermentable(writer, f, offset + 1)?;
    }
    indent(writer, offset)?;
    writer.write(b"</FERMENTABLES>\n")?;
    Ok(())
}

pub fn write<T>(writer: &mut T, set: &RecordSet) -> Result<()>
    where T: Write
{
    match *set {
        RecordSet::Empty => Ok(()),
        RecordSet::Fermentables(ref v) => write_fermentables(writer, v, 0),
    }
}

pub fn write_file(filename: &Path, set: &RecordSet) -> Result<()> {
    let mut f = File::create(filename)?;
    write(&mut f, set)
}
