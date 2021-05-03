use regex::Regex;
use super::reg_value_object::*;
use std::collections::HashMap;
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
    pub path: String,
    pub filename: String,
    pub encoding: String,
    pub content: String,
    pub regvalues: HashMap<String, HashMap<String, RegValueObject>>,
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
    /// Constructs a new instance of `RegFileObject`
    ///
    /// # Arguments
    /// 
    /// * `reg_file_name` - The name of the REG file to open
    /// 
    /// # Examples
    /// 
    /// ```
    /// use reg-file-parser::reg_file_object::RegFileObject;
    /// let reg_file_object = RegFileObject::new("./settings.reg");
    /// ```
    pub fn new(&self, reg_file_name: &str) -> RegFileObject
    {
        let file_name = Path::new(reg_file_name)
                            .file_name()
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .to_string();

        RegFileObject {
            path: reg_file_name.to_string(),
            filename: file_name,
            encoding: "UTF8".to_string(),
            regvalues: HashMap::new(),
            full_path: String::new(),
            content: String::new(),
        }
    }

    /// # TODO
    /// Imports the REG file
    pub fn read(&mut self) {
        // TODO
    }

    /// # TODO
    /// Parses the reg file for reg keys and reg values
    /// 
    /// Returns a HashMap with reg keys as HashMap keys and a HashMap of (valuename, valuedata)
    /// 
    pub fn parse_file(&self) -> HashMap<String, HashMap<String, String>> {
        // TODO
        HashMap::new()
    }

    /// # NOT WORKING YET.
    /// 
    /// Creates a HashMap using given search pattern.
    /// Takes the `content` field of `RegFileObject` and returns
    /// a HashMap with retrieved keys and remaining content
    fn normalize_keys_dictionary(&self) -> HashMap<String, String> {
        let re = Regex::new(r"(?m)^[\t ]*\\[.+\\][\r\n]+").unwrap();
        for caps in re.captures_iter(&self.content[..]) {
            let mut skey: &str = &caps[0];
            if skey.ends_with("\r\n") {
                skey = &skey[0..skey.len() - 2];
            }
            if skey.ends_with('=') {
                skey = &skey[0..skey.len() - 1];
            }

            let mut key = &strip_braces(skey);

            if key == "@" {
                key = &"".to_string();
            }
            else {
                key = &strip_leading_chars(skey, "\\");
            }
            let bytes = &caps[1].as_bytes();
            let start_index = bytes[0] as usize + &caps.len();
            let next_match = &caps[2].as_bytes();
        }

        HashMap::new()
    }

    /// # TODO
    /// Creates a HashMap using given search pattern.
    /// Takes the `content` field of `RegFileObject` and returns
    /// a HashMap with retrieved keys and remaining content
    fn normalize_values_dictionary(&self, content: String) -> HashMap<String, String> {
        // TODO
        HashMap::new()
    }
}