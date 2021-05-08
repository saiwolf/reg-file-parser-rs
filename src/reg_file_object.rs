#![allow(dead_code)]

use regex::{Regex, RegexSet};
use super::errors::{Error::HashMapEmpty, Result};

use super::reg_value_object::*;
use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::path::Path;
use super::utils::*;

/// Represents a registry file's properties
/// 
/// # Arguments
/// 
/// * `path` - The relative file path of the REG file
/// * `filename` - The file name of the REG file
/// * `encoding` - The file encoding of the REG file. Either ANSI or UTF8.
/// * `regvalues` - The `HashMap` containing parsed registry values
/// * `full_path` - The absolute path to the REG file
/// 
pub struct RegFileObject {
    /// The relative file path of the REG file
    pub path: String,
    /// The file name of the REG file
    pub filename: String,
    /// The file encoding of the REG file. ANSI or UTF8.
    pub encoding: String,
    /// The raw content of the REG file.
    pub content: String,
    /// The `HashMap` containing parsed registry values.
    pub regvalues: HashMap<String, HashMap<String, RegValueObject>>,
    /// The full path to the REG file.
    pub full_path: String,
}

///
/// `Default` implmentation of `RegFileObject`
/// Assigns defualt values.
/// `encoding` is set to UTF8
/// 
impl Default for RegFileObject {
    fn default() -> RegFileObject {
        RegFileObject {
            path: String::new(),
            filename: String::new(),
            encoding: "UTF8".to_string(),
            regvalues: HashMap::new(),
            full_path: String::new(),
            content: String::new(),
        }
    }
}

impl RegFileObject {    
    /**
    Constructs a new instance of `RegFileObject`


    # Arguments

    * `reg_file_name` - The name of the REG file to open

    # Returns
    A `Result` containing either `Ok<RegFileObject>` or an `Err` containing the error message.

    # Examples    

    Testing non-existing file error:
    ```
    use reg_file_parser::reg_file_object::RegFileObject;
    assert!(RegFileObject::new("./non-existant.reg").is_err());
    ```
    */
    pub fn new(reg_file_name: &str) -> Result<RegFileObject, Error>
    {
        let file = Path::new(reg_file_name);
        
        if !file.exists() {
            return Err(Error::new(ErrorKind::NotFound, format!("Error opening file: {}.", reg_file_name)));
        }

        let file_name = file.file_name().unwrap().to_str().unwrap().to_string();

        Ok(
            RegFileObject {
                path: reg_file_name.to_string(),
                filename: file_name,
                encoding: "UTF8".to_string(),
                regvalues: HashMap::new(),
                full_path: String::new(),
                content: String::new(),
            }
        )
    }

    /**
    # TODO
    Imports the REG file
    */
    pub fn read(&mut self) {
        // TODO
        strip_braces("TEST");
        strip_continue_char("TEST");
        strip_leading_chars("TEST", "T");
    }

    /**
    # TODO
    Parses the reg file for reg keys and reg values

    Returns a HashMap with reg keys as HashMap keys and a HashMap of (valuename, valuedata)

    */
    pub fn parse_file(&self) -> HashMap<String, HashMap<String, String>> {
        // TODO
        HashMap::new()
    }

    /**
    # NOT WORKING YET.

    Creates a HashMap using given search pattern.
    Takes the `content` field of `RegFileObject` and returns
    a HashMap with retrieved keys and remaining content
    */
    fn normalize_keys_dictionary(&self, content: &str) -> crate::errors::Result<HashMap<String, String>> {
        let re = Regex::new(r"(?m)^[\t ]*\[.+\][\r\n]+").unwrap();
    let encoding = "UTF8";
    let mut dict = HashMap::<String, String>::new();

    for caps in re.captures_iter(&content) {
        let mut skey: &str = &caps[0];
        if skey.ends_with("\r\n") {
            skey = &skey[0..skey.len() - 2];
        }
        if skey.ends_with('=') {
            skey = &skey[0..skey.len() - 1];
        }
        let mut key = strip_braces(&skey);
        
        key = strip_leading_chars(&key, "\\");
        
        let value = &content.replace("Windows Registry Editor Version 5.00", "")
        let value2 = &value.trim_start_matches("\n\n");
            let value3 = &value2.replace(skey, "")
            .trim();
        
        if value.ends_with("\r\n") {
            value = &&value.trim_end_matches("\r").trim_end_matches("\n");
        }
        
        dict.insert(key.to_string(), value.to_string());
    }

        match dict.keys().len() {
            0 => Err(HashMapEmpty),
            _ => Ok(dict)
        }
    }

    /**
    # TODO
    Creates a HashMap using given search pattern.
    Takes the `content` field of `RegFileObject` and returns
    a HashMap with retrieved keys and remaining content
    */
    fn normalize_values_dictionary(&self, content: String) -> Result<HashMap<String, String>, String> {
        let re = Regex::new(r#"(?m)^[\t ]*(".+"|@)=("[^"]*"|[^"]+)"#).unwrap();
        
        let mut dict = HashMap::<String, String>::new();

        for caps in re.captures_iter(&content) {

        }

        Ok(dict)
    }
}