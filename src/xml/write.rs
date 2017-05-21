// (c) 2017 Joost Yervante Damad <joost@damad.be>

use std::collections::HashMap;
use std::io::Write;
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

fn write_fermentable<T>(writer: &mut T, f: &Fermentable, offset: usize) -> Result<()>
    where T: Write
{
    indent(writer, offset)?;
    writer.write_all(b"<FERMENTABLE>\n")?;

    indent(writer, offset + 1)?;
    write!(writer, "<NAME>{}</NAME>\n", f.name)?;

    indent(writer, offset + 1)?;
    write!(writer, "<TYPE>{}</TYPE>\n", f.type_.as_str())?;

    indent(writer, offset + 1)?;
    write!(writer, "<AMOUNT>{}</AMOUNT>\n", f.amount)?;

    indent(writer, offset + 1)?;
    write!(writer, "<YIELD>{}</YIELD>\n", f.yield_)?;

    indent(writer, offset + 1)?;
    write!(writer, "<COLOR>{}</COLOR>\n", f.color)?;

    if f.add_after_boil {
        indent(writer, offset + 1)?;
        write!(writer,
               "<ADD_AFTER_BOIL>{}</ADD_AFTER_BOIL>\n",
               f.add_after_boil)
            ?;
    }

    if let Some(ref origin) = f.origin {
        indent(writer, offset + 1)?;
        write!(writer, "<ORIGIN>{}</ORIGIN>\n", origin)?;
    }

    if let Some(ref supplier) = f.supplier {
        indent(writer, offset + 1)?;
        write!(writer, "<SUPPLIER>{}</SUPPLIER>\n", supplier)?;
    }

    if let Some(ref notes) = f.notes {
        indent(writer, offset + 1)?;
        write!(writer, "<NOTES>{}</NOTES>\n", notes)?;
    }

    if let Some(coarse_fine_diff) = f.coarse_fine_diff {
        indent(writer, offset + 1)?;
        write!(writer,
               "<COARSE_FINE_DIFF>{}</COARSE_FINE_DIFF>\n",
               coarse_fine_diff)
            ?;
    }

    if let Some(moisture) = f.moisture {
        indent(writer, offset + 1)?;
        write!(writer, "<MOISTURE>{}</MOISTURE>\n", moisture)?;
    }

    if let Some(diastatic_power) = f.diastatic_power {
        indent(writer, offset + 1)?;
        write!(writer,
               "<DIASTATIC_POWER>{}</DIASTATIC_POWER>\n",
               diastatic_power)
            ?;
    }

    if let Some(proteine) = f.proteine {
        indent(writer, offset + 1)?;
        write!(writer, "<PROTEINE>{}</PROTEINE>\n", proteine)?;
    }

    if let Some(max_in_batch) = f.max_in_batch {
        indent(writer, offset + 1)?;
        write!(writer, "<MAX_IN_BATCH>{}</MAX_IN_BATCH>\n", max_in_batch)?;
    }

    if f.recommended_mash {
        indent(writer, offset + 1)?;
        write!(writer,
               "<RECOMMENDED_MASH>{}</RECOMMENDED_MASH>\n",
               f.recommended_mash)
            ?;
    }

    if let Some(ibu_gal_per_lb) = f.ibu_gal_per_lb {
        indent(writer, offset + 1)?;
        write!(writer,
               "<IBU_GAL_PER_LB>{}</IBU_GAL_PER_LB>\n",
               ibu_gal_per_lb)
            ?;
    }

    indent(writer, offset)?;
    writer.write_all(b"</FERMENTABLE>\n")?;
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
    }
}

pub fn write_file(filename: &Path, set: &RecordSet) -> Result<()> {
    let mut f = File::create(filename)?;
    write(&mut f, set)
}
