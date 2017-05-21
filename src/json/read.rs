// (c) 2017 Joost Yervante Damad <joost@damad.be>

/*
pub fn read<B>(reader: B) -> Result<RecordSet>
    where B: BufRead
{
    serde_json::from_reader(reader)?;
}

pub fn read_file(filename: &Path) -> Result<RecordSet> {
    let f = File::open(filename)?;
    let reader = BufReader::new(f);
    read(reader)
}
*/
