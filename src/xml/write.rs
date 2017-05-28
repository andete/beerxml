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

fn write_bool<T>(writer:&mut T, offset:usize, tag:&'static str, value:bool) -> Result<()>
    where T:Write
{
    if value {
        indent(writer, offset + 1)?;
        write!(writer, "<{}>{}</{}>\n", tag, value, tag)?;
    }
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
        write_tag(writer, offset, "VERSION", f.version)?;
        write_tag(writer, offset, "TYPE", &f.type_)?;
        write_tag(writer, offset, "AMOUNT", f.amount)?;
        write_tag(writer, offset, "YIELD", f.yield_)?;
        write_tag(writer, offset, "COLOR", f.color)?;
        write_bool(writer, offset, "ADD_AFTER_BOIL", f.add_after_boil)?;
        write_opt(writer, offset, "ORIGIN", &f.origin)?;
        write_opt(writer, offset, "SUPPLIER", &f.supplier)?;
        write_opt(writer, offset, "NOTES", &f.notes)?;
        write_opt(writer, offset, "COARSE_FINE_DIFF", &f.coarse_fine_diff)?;
        write_opt(writer, offset, "MOISTURE", &f.moisture)?;
        write_opt(writer, offset, "DIASTATIC_POWER", &f.diastatic_power)?;
        write_opt(writer, offset, "PROTEIN", &f.protein)?;
        write_opt(writer, offset, "MAX_IN_BATCH", &f.max_in_batch)?;
        write_bool(writer, offset, "RECOMMEND_MASH", f.recommend_mash)?;
        write_opt(writer, offset, "IBU_GAL_PER_LB", &f.ibu_gal_per_lb)?;
        write_opt(writer, offset, "DISPLAY_AMOUNT", &f.display_amount)?;
        write_opt(writer, offset, "INVENTORY", &f.inventory)?;
        write_opt(writer, offset, "POTENTIAL", &f.potential)?;
        write_opt(writer, offset, "DISPLAY_COLOR", &f.display_color)
    })
}

fn write_hop<T>(writer: &mut T, h: &Hop, offset: usize) -> Result<()>
    where T:Write
{
    write_block(writer, offset, "HOP", |writer, offset| {
        write_tag(writer, offset, "NAME", &h.name)?;
        write_tag(writer, offset, "VERSION", h.version)?;
        write_tag(writer, offset, "ALPHA", h.alpha)?;
        write_tag(writer, offset, "AMOUNT", h.amount)?;
        write_tag(writer, offset, "USE", &h.use_)?;
        write_tag(writer, offset, "TIME", h.time)?;
        write_opt(writer, offset, "NOTES", &h.notes)?;
        write_opt(writer, offset, "TYPE", &h.type_)?;
        write_opt(writer, offset, "FORM", &h.form)?;
        write_opt(writer, offset, "BETA", &h.beta)?;
        write_opt(writer, offset, "HSI", &h.hsi)?;
        write_opt(writer, offset, "ORIGIN", &h.origin)?;
        write_opt(writer, offset, "SUBSTITUTES", &h.substitutes)?;
        write_opt(writer, offset, "HUMULENE", &h.humulene)?;
        write_opt(writer, offset, "CARYOPHYLLENE", &h.caryophyllene)?;
        write_opt(writer, offset, "COHUMULONE", &h.cohumulone)?;
        write_opt(writer, offset, "MYRCENE", &h.myrcene)
    })
}

fn write_yeast<T>(writer: &mut T, y: &Yeast, offset: usize) -> Result<()>
    where T:Write
{
    write_block(writer, offset, "YEAST", |writer, offset| {
        write_tag(writer, offset, "NAME", &y.name)?;
        write_tag(writer, offset, "VERSION", y.version)?;
        // TODO
        Ok(())
    })
}

fn write_map<E,F,T>(writer: &mut T,
                    v: &HashMap<String, E>,
                    offset: usize,
                    name:&'static str,
                    write_element:F) -> Result<()>
    where T: Write,
          F:Fn(&mut T, &E, usize) -> Result<()>
{
    write_block(writer, offset, name, |writer, offset| {
        for f in v.values() {
            write_element(writer, f, offset + 1)?;
        }
        Ok(())
    })
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
        RecordSet::Fermentables(ref v) => write_map(writer, v, 0, "FERMENTABLES", write_fermentable),
        RecordSet::Hops(ref v) => write_map(writer, v, 0, "HOPS", write_hop),
        RecordSet::Yeasts(ref v) => write_map(writer, v, 0, "YEASTS", write_yeast),
    }
}

pub fn write_file(filename: &Path, set: &RecordSet) -> Result<()> {
    let mut f = File::create(filename)?;
    write(&mut f, set)
}
