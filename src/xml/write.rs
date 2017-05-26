// (c) 2017 Joost Yervante Damad <joost@damad.be>

use std::collections::HashMap;
use std::io::Write;
use std::fmt::Display;
use std::fs::File;
use std::path::Path;

use data::*;
use error::*;

fn indent<T>(writer: &mut T, offset: usize) -> Result<()>
    where T: Write
{
    for _ in 0..offset {
        writer.write_all(b"  ")?;
    }
    Ok(())
}

fn write_tag<T,U>(writer:&mut T, offset:usize, tag:&'static str, value:U) -> Result<()>
    where T:Write, U:Display
{
    indent(writer, offset + 1)?;
    write!(writer, "<{}>{}</{}>\n", tag, value, tag)?;
    Ok(())
}
fn write_opt<T,U>(writer:&mut T, offset:usize, tag:&'static str, value:&Option<U>) -> Result<()>
    where T: Write, U:Display
{
    if let Some(ref value) = *value {
        indent(writer, offset + 1)?;
        write!(writer, "<{}>{}</{}>\n", tag, value, tag)?;
    }
    Ok(())
}

fn write_block<F,T>(writer: &mut T, offset: usize, tag:&'static str, containing:F) -> Result<()>
    where T: Write,
          F: Fn(&mut T, usize) -> Result<()>
{
    indent(writer, offset)?;
    write!(writer,"<{}>\n", tag)?;
    containing(writer, offset+1)?;
    indent(writer, offset)?;
    write!(writer,"</{}>\n", tag)?;
    Ok(())
}

fn write_fermentable<T>(writer: &mut T, f: &Fermentable, offset: usize) -> Result<()>
    where T: Write
{
    write_block(writer, offset, "FERMENTABLE", |writer, offset| {

        write_tag(writer, offset, "NAME", &f.name)?;
        write_tag(writer, offset, "TYPE", f.type_.as_str())?;
        write_tag(writer, offset, "AMOUNT", f.amount)?;
        write_tag(writer, offset, "YIELD", f.yield_)?;
        write_tag(writer, offset, "COLOR", f.color)?;
        if f.add_after_boil {
            write_tag(writer, offset, "ADD_AFTER_BOIL", f.add_after_boil)?
        }
        write_opt(writer, offset, "ORIGIN", &f.origin)?;
        write_opt(writer, offset, "SUPPLIER", &f.supplier)?;
        write_opt(writer, offset, "NOTES", &f.notes)?;
        write_opt(writer, offset, "COARSE_FINE_DIFF", &f.coarse_fine_diff)?;
        write_opt(writer, offset, "MOISTURE", &f.moisture)?;
        write_opt(writer, offset, "DIASTATIC_POWER", &f.diastatic_power)?;
        write_opt(writer, offset, "PROTEINE", &f.proteine)?;
        write_opt(writer, offset, "MAX_IN_BATCH", &f.max_in_batch)?;
        if f.recommended_mash {
            write_tag(writer, offset, "RECOMMENDED_MASH", f.recommended_mash)?;
        }
        write_opt(writer, offset, "IBU_GAL_PER_LB", &f.ibu_gal_per_lb)
    })
}

fn write_hop<T>(writer: &mut T, f: &Hop, offset: usize) -> Result<()> {
    // TODO
    Ok(())
}

fn write_fermentables<T>(writer: &mut T,
                         v: &HashMap<String, Fermentable>,
                         offset: usize)
                         -> Result<()>
    where T: Write
{
    indent(writer, offset)?;
    writer.write_all(b"<FERMENTABLES>\n")?;
    for f in v.values() {
        write_fermentable(writer, f, offset + 1)?;
    }
    indent(writer, offset)?;
    writer.write_all(b"</FERMENTABLES>\n")?;
    Ok(())
}

fn write_hops<T>(writer: &mut T,
                         v: &HashMap<String, Hop>,
                         offset: usize)
                         -> Result<()>
    where T: Write
{
    indent(writer, offset)?;
    writer.write_all(b"<HOPS>\n")?;
    for f in v.values() {
        write_hop(writer, f, offset + 1)?;
    }
    indent(writer, offset)?;
    writer.write_all(b"</HOPS>\n")?;
    Ok(())
}

pub fn write<T>(writer: &mut T, set: &RecordSet) -> Result<()>
    where T: Write
{
    writer.write_all(b"<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n")?;
    write!(writer,
           "<!-- written by brewcalc {}: http://brewcalc.org/ -->\n",
           env!("CARGO_PKG_VERSION"))
        ?;
    match *set {
        RecordSet::Empty => Ok(()),
        RecordSet::Fermentables(ref v) => write_fermentables(writer, v, 0),
        RecordSet::Hops(ref v) => write_hops(writer, v, 0),
    }
}

pub fn write_file(filename: &Path, set: &RecordSet) -> Result<()> {
    let mut f = File::create(filename)?;
    write(&mut f, set)
}
