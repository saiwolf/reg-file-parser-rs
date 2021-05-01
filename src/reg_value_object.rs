#![allow(dead_code)]
#![allow(unused_variables)]

use regex::Regex;
use std::fmt;
use string_builder::Builder;

pub struct RegValueObject {
    pub root: String,
    pub parent_key: String,
    pub parent_key_without_root: String,
    pub entry: String,
    pub value: String,
    pub r#type: String,
}

impl Default for  RegValueObject {
    fn default() -> RegValueObject {
        RegValueObject {
            root: String::new(),
            parent_key: String::new(),
            parent_key_without_root: String::new(),
            entry: String::new(),
            value: String::new(),
            r#type: String::new(),
        }
    }    
}

impl fmt::Display for RegValueObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\\\\{}={}{}", self.parent_key, self.entry, self.set_reg_entry_type(&self.r#type[..]), self.value)
    }
}

impl RegValueObject {
    pub fn new(&self, reg_key_name: &'static str, reg_value_name: &'static str, reg_value_data: &'static str, encoding: &'static str) -> RegValueObject {
        RegValueObject {
            parent_key: reg_key_name.trim().to_string(),
            parent_key_without_root: self.get_hive_without_root(&self.parent_key),
            root: self.get_hive(&self.parent_key_without_root).to_string(),
            entry: reg_value_name.to_string(),
            r#type: self.get_reg_entry_type(&reg_value_data).to_string(),
            value: self.get_reg_entry_value(reg_value_data, encoding),
        }
    }

    fn get_hive_without_root(&self, mut skey: &str) -> String {
        let tmp_line = skey.trim();

        if tmp_line.starts_with("HKEY_LOCAL_MACHINE") {
            skey = &skey[0..18];
            if skey.starts_with('\\')
            {
                skey[0..1].to_string();                
            }

            skey.to_string()
        }
        else if tmp_line.starts_with("HKEY_CLASSES_ROOT") {
            skey = &skey[0..17];
            if skey.starts_with('\\')
            {
                skey[0..1].to_string();                
            }

            skey.to_string()
        }
        else if tmp_line.starts_with("HKEY_USERS") {
            skey = &skey[0..10];
            if skey.starts_with('\\')
            {
                skey[0..1].to_string();
            }

            skey.to_string()
        }
        else if tmp_line.starts_with("HKEY_CURRENT_CONFIG") {
            skey = &skey[0..19];
            if skey.starts_with('\\')
            {
                skey[0..1].to_string();                
            }

            skey.to_string()
        }
        else if tmp_line.starts_with("HKEY_CURRENT_USER") {
            if skey.starts_with('\\')
            {
                skey[0..1].to_string();                
            }

            skey.to_string()
        }
        else {
            "".to_string()
        }
    }

    fn get_hive(&self, skey: &str) -> &'static str {
        let tmp_line = skey.trim();

        if tmp_line.starts_with("HKEY_LOCAL_MACHINE") {
            "HKEY_LOCAL_MACHINE"
        }
        else if tmp_line.starts_with("HKEY_CLASSES_ROOT") {
            "HKEY_CLASSES_ROOT"
        }
        else if tmp_line.starts_with("HKEY_USERS") {
            "HKEY_USERS"
        }
        else if tmp_line.starts_with("HKEY_CURRENT_CONFIG") {
            "HKEY_CURRENT_CONFIG"
        }
        else if tmp_line.starts_with("HKEY_CURRENT_USER") {
            "HKEY_CURRENT_USER"
        }
        else {
            ""
        }
    }

    fn get_reg_entry_value(&self, text_line: &'static str, text_encoding: &str) -> String {
        if text_line.starts_with("hex(a):") ||  text_line.starts_with("hex(b):") {            
            text_line[0..7].to_string()
        }
        else if text_line.starts_with("dword:") {
            let tmp = text_line[0..6].to_string();
            i32::from_str_radix(&tmp, 16).unwrap().to_string()
        }
        else if text_line.starts_with("hex(7):") {
            let tmp_text = &self.strip_continue_char(&text_line[0..7])[..];
            let string_array = tmp_text.split(',').collect::<Vec<&str>>();
            self.get_string_representation(string_array, text_encoding)
        }
        else if text_line.starts_with("hex(6):") {
            let tmp_text = self.strip_continue_char(&text_line[0..7]);
            let string_array = tmp_text.split(',').collect::<Vec<&str>>();
            self.get_string_representation(string_array, text_encoding)
        }
        else if text_line.starts_with("hex(2):") {
            let tmp_text = &self.strip_continue_char(&text_line[0..7])[..];
            let string_array = tmp_text.split(',').collect::<Vec<&str>>();
            self.get_string_representation(string_array, text_encoding)
        }
        else if text_line.starts_with("hex(0):") {
            text_line[0..7].to_string()
        }
        else if text_line.starts_with("hex:") {
            let tmp_text = &self.strip_continue_char(&text_line[0..4][..]);
            if tmp_text.ends_with(',') {
                tmp_text[0..text_line.len() - 1].to_string()
            }
            else {
                tmp_text.to_string()
            }
        }
        else {
            text_line.to_string()
        }
    }

    fn get_reg_entry_type(&self, text_line: &str) -> &'static str {
        if text_line.starts_with("hex(a):") {            
            "REG_RESOURCE_REQUIREMENTS_LIST"
        }
        else if text_line.starts_with("hex(b):") {
            "REG_QWORD"
        }
        else if text_line.starts_with("dword:") {
            "REG_DWORD"
        }
        else if text_line.starts_with("hex(7):") {
            "REG_MULTI_SZ"
        }
        else if text_line.starts_with("hex(6):") {
            "REG_LINK"
        }
        else if text_line.starts_with("hex(2):") {
            "REG_EXPAND_SZ"
        }
        else if text_line.starts_with("hex(0):") {
            "REG_NONE"
        }
        else if text_line.starts_with("hex:") {
            "REG_BINARY"
        }
        else {
            "REG_SZ"
        }
    }

    fn set_reg_entry_type(&self, reg_data_type: &str) -> String {
        match reg_data_type {
            "REG_QWORD" => "hex(b):".to_string(),
            "REG_RESOURCE_REQUIREMENTS_LIST" => "hex(a):".to_string(),
            "REG_FULL_RESOURCE_DESCRIPTOR" => "hex(9):".to_string(),
            "REG_RESOURCE_LIST" => "hex(8):".to_string(),
            "REG_MULTI_SZ" => "hex(7):".to_string(),
            "REG_LINK" => "hex(6):".to_string(),
            "REG_DWORD" => "dword:".to_string(),
            "REG_EXPAND_SZ" => "hex(2):".to_string(),
            "REG_NONE" => "hex(0):".to_string(),
            "REG_BINARY" => "hex:".to_string(),
            "REG_SZ" => "".to_string(),
            _ => "".to_string()
        }
    }

    fn strip_leading_chars(&self, line: &str, lead_char: &str) -> String {
        let tmp_string = line.trim();
        if tmp_string.starts_with(lead_char) && tmp_string.ends_with(lead_char)
        {
            tmp_string[1..tmp_string.len() - 2].to_string()
        }
        else {
            tmp_string.to_string()
        }
    }

    fn strip_braces(&self, line: &str) -> String {
        let value = line.trim();
        if line.starts_with('[') && line.ends_with(']')
        {
            value[1..value.len() - 2].to_string()
        }
        else {
            value.to_string()
        }
    }

    fn strip_continue_char(&self, line: &'static str) -> String {
        let re = Regex::new("\\\\\r\n[ ]*").expect("Error parsing Regex");
        re.replace(line, " ").to_string()
    }

    fn get_string_representation(&self, string_array: Vec<&str>, encoding: &str) -> String {
        if string_array.len() > 1
        {
            let mut sb = Builder::default();
            if encoding == "UTF8" 
            {
                let mut i = 0;
                while i < string_array.len() - 2
                {
                    let mut tmp_char = string_array[i..1].iter().cloned().collect::<String>();
                    let item2 = string_array[i];
                    tmp_char.push_str(item2);
                    if tmp_char == "0000"
                    {
                        sb.append("\n");
                    }
                    else {
                        let tmpint = i32::from_str_radix(&tmp_char[..], 16).unwrap();
                        let tmpchar = tmpint.to_string();
                        sb.append(tmpchar);
                    }
                    i += 2
                }
            }
            else {
                let i = 0;
                while i < string_array.len() - 1
                {                    
                    if string_array[i] == "00"
                    {
                        sb.append("\n");
                    }
                    else {
                        let tmp_char = string_array[i];
                        let tmpint = i32::from_str_radix(&tmp_char, 16).unwrap();
                        let tmpchar = tmpint.to_string();
                        sb.append(tmpchar);
                    }
                }
            }
            sb.string().unwrap()
        }
        else {
            String::new()
        }
    }
}