use regex::Regex;
use super::reg_value_object::*;
use std::fmt;
use std::collections::HashMap;
use std::path::Path;
use std::io::prelude::*;
use string_builder::Builder;
use super::utils::*;

pub struct RegFileObject {
    pub path: String,
    pub filename: String,
    pub encoding: String,
    pub content: String,
    pub regvalues: HashMap<String, HashMap<String, RegValueObject>>,
    pub full_path: String,
}

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

    pub fn read(&mut self) {
        if Path::new(&self.path).exists() {
        }
    }

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

            skey = &strip_braces(skey);

            if skey == "@" {
                skey = "";
            }
            else {
                skey = &strip_leading_chars(skey, "\\");
            }
            let bytes = &caps[1].as_bytes();
            let start_index = bytes[0] as usize + &caps.len();
            let next_match = &caps[2].as_bytes();
        }

        HashMap<String, String>::new()
    }

    fn get_encoding(&self) -> String {
        let re = Regex::new(r"(?i)([ ]*(\r\n)*)(?i)REGEDIT4").unwrap();
        match re.is_match(&self.content[..]) {
            true => "ANSI".to_string(),
            false => "UTF8".to_string(),
        }
    }
}