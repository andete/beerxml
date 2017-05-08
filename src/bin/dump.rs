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
    let m = App::new("dump")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Joost Yervante Damad <joost@damad.be>")
        .about("Dump test program")
        .arg(Arg::with_name("FILE")
             .required(true)
             .index(1)
             .help("filename")).get_matches();

    env::set_var("RUST_LOG","info");
    env_logger::init().unwrap(); 

    
    let filename = m.value_of("FILE").unwrap();
    
    let content = beerxml::xml::read_file(Path::new(filename)).unwrap();
    info!("Content: {:?}", content);
}
