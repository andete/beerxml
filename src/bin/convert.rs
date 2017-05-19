// (c) 2017 Joost Yervante Damad <joost@damad.be>

extern crate clap;
extern crate env_logger;

#[macro_use]
extern crate log;

extern crate beerxml;

use std::path::Path;
use std::env;

use clap::{Arg, App};

fn main() {
    let m = App::new("convert")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Joost Yervante Damad <joost@damad.be>")
        .about("Dump test program")
        .arg(Arg::with_name("FILE1")
            .required(true)
            .index(1)
            .help("filename to convert from"))
        .arg(Arg::with_name("FILE2")
            .required(true)
            .index(2)
            .help("filename to convert to"))
        .get_matches();

    env::set_var("RUST_LOG", "info");
    env_logger::init().unwrap();


    let filename_from = m.value_of("FILE1").unwrap();
    let filename_to = m.value_of("FILE2").unwrap();

    let content = beerxml::read_file(Path::new(filename_from)).unwrap();
    info!("Content: {:?}", content);
    beerxml::write_file(Path::new(filename_to), &content).unwrap();
    info!("{} created.", filename_to);
}
