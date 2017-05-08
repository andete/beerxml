// (c) 2017 Joost Yervante Damad <joost@damad.be>

extern crate quick_xml;
extern crate serde;
extern crate serde_json;
extern crate serde_yaml;

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}

pub mod data;
pub mod error;
pub mod xml;
