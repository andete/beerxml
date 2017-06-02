// (c) 2017 Joost Yervante Damad <joost@damad.be>

use quick_xml::errors as xml;

error_chain! {

    errors {
        /// Parsing Error
        ParseError(what:String, unknown:String) {
            description("Unable to parse something")
            display("Unable to parse `{}` as {}", unknown, what)
        }
    }
    
    links {
        Xml(xml::Error, xml::ErrorKind) #[doc = "XML error"];
    }

    foreign_links {
        Io(::std::io::Error) #[doc = "IO error"];
        Utf8(::std::str::Utf8Error) #[doc = "Utf8 Error"];
        ParseFloat(::std::num::ParseFloatError) #[doc = "Parse Float Error"];
        ParseInt(::std::num::ParseIntError) #[doc = "Parse Int Error"];
        ParseString(::std::string::ParseError) #[doc = "Parse String Error"];
        Json(::serde_json::Error) #[doc = "Json Error"];
        Yaml(::serde_yaml::Error) #[doc = "Yaml Error"];
        TomlSer(::serde_toml::ser::Error) #[doc = "Toml Serialization Error"];
        TomlDe(::serde_toml::de::Error) #[doc = "Toml Deserialization Error"];
    }
}
