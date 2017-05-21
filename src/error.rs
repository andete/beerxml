// (c) 2017 Joost Yervante Damad <joost@damad.be>

use quick_xml::errors as xml;

error_chain! {
    links {
        Xml(xml::Error, xml::ErrorKind);
    }

    foreign_links {
        Io(::std::io::Error);
        Utf8(::std::str::Utf8Error);
        ParseFloat(::std::num::ParseFloatError);
        Json(::serde_json::Error);
        Yaml(::serde_yaml::Error);
        TomlSer(::serde_toml::ser::Error);
        TomlDe(::serde_toml::de::Error);
    }
}
