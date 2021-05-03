#![allow(dead_code)]
#![allow(unused_variables)]

use std::fmt;
use string_builder::Builder;
use super::utils::*;

/// Represents a Registry Value
/// 
/// # Arguments
/// 
/// * `root` - Registry Root HIVE
/// * `parent_key` - Registry Value parent key, if applicable
/// * `parent_key_without_root` - Registry Value parent key, without root HIVE
/// * `entry` - Registry Value name
/// * `value` - Registry Value data
/// * `reg_type` - Registry Value type
/// 
pub struct RegValueObject {
    /// Registry Root HIVE
    pub root: String,
    /// Registry Value parent key, if applicable
    pub parent_key: String,
    /// Registry Value parent key, without root HIVE
    pub parent_key_without_root: String,
    /// Registry Value Name
    pub entry: String,
    /// Registry Value Data
    pub value: String,
    /// Registry Value Type
    pub reg_type: String,
}

impl Default for  RegValueObject {
    fn default() -> RegValueObject {
        RegValueObject {
            root: String::new(),
            parent_key: String::new(),
            parent_key_without_root: String::new(),
            entry: String::new(),
            value: String::new(),
            reg_type: String::new(),
        }
    }    
}

// Implements `to_string()` method
// Ex: let reg = RegValueObject::new("./settings.reg").to_string();
impl fmt::Display for RegValueObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\\\\{}={}{}", self.parent_key, self.entry, self.set_reg_entry_type(&self.reg_type[..]), self.value)
    }
}

impl RegValueObject {
    /// Returns a `RegValueObject` given the following parameters:
    /// 
    /// # Arguments
    /// 
    /// * `reg_key_name` - Name of Registry Key
    /// * `reg_value_name` - Name of Registry Value
    /// * `reg_value_data` - Data of Registry Value
    /// * `encoding` - Encoding of data. Either UTF8 or ANSI
    /// 
    /// # Example
    /// 
    /// ```
    /// use reg-file-parser::reg_value_object::RegValueObject;
    /// let reg_value = RegValueObject::new("Settings", "BackgroundColor", "#000000", "UTF8");
    /// assert_eq!(reg_value.value, "#000000");
    /// ```
    /// 
    pub fn new(&self, reg_key_name: &'static str, reg_value_name: &'static str, reg_value_data: &'static str, encoding: &'static str) -> RegValueObject {
        RegValueObject {
            parent_key: reg_key_name.trim().to_string(),
            parent_key_without_root: self.get_hive_without_root(&self.parent_key),
            root: self.get_hive(&self.parent_key_without_root).to_string(),
            entry: reg_value_name.to_string(),
            reg_type: self.get_reg_entry_type(&reg_value_data).to_string(),
            value: self.get_reg_entry_value(reg_value_data, encoding),
        }
    }

    /// Gets registry hive without the root hive
    /// 
    /// * `skey` - Registry Key to get root hive of
    /// 
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

    /// Returns Registry Root Hive
    /// 
    /// * `skey` - Registry Key to get root hive of
    ///
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

    /// Returns the registry value without the type declaration in front.
    /// 
    /// * `text_line` - The line of text to parse
    /// * `text_encoding` - The line encoding
    /// 
    fn get_reg_entry_value(&self, text_line: &'static str, text_encoding: &str) -> String {
        if text_line.starts_with("hex(a):") ||  text_line.starts_with("hex(b):") {            
            text_line[0..7].to_string()
        }
        else if text_line.starts_with("dword:") {
            let tmp = text_line[0..6].to_string();
            i32::from_str_radix(&tmp, 16).unwrap().to_string()
        }
        else if text_line.starts_with("hex(7):") {
            let tmp_text = &strip_continue_char(&text_line[0..7])[..];
            let string_array = tmp_text.split(',').collect::<Vec<&str>>();
            self.get_string_representation(string_array, text_encoding)
        }
        else if text_line.starts_with("hex(6):") {
            let tmp_text = strip_continue_char(&text_line[0..7]);
            let string_array = tmp_text.split(',').collect::<Vec<&str>>();
            self.get_string_representation(string_array, text_encoding)
        }
        else if text_line.starts_with("hex(2):") {
            let tmp_text = &strip_continue_char(&text_line[0..7])[..];
            let string_array = tmp_text.split(',').collect::<Vec<&str>>();
            self.get_string_representation(string_array, text_encoding)
        }
        else if text_line.starts_with("hex(0):") {
            text_line[0..7].to_string()
        }
        else if text_line.starts_with("hex:") {
            let tmp_text = &strip_continue_char(&text_line[0..4][..]);
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

    /// Retrieves the reg value type, parsing the prefix of the value
    ///
    /// * `text_line` - The line of text to parse
    /// 
    /// This function will return "REG_SZ" if a match is not found.
    /// 
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

    /// Sets the registry entry's type from the given data string
    /// 
    /// * `reg_data_type` - Data type. See Types below
    /// 
    /// # Types
    /// 
    /// hex: REG_BINARY
    /// 
    /// hex(0): REG_NONE
    /// 
    /// hex(1): REG_SZ
    /// 
    /// hex(2): EXPAND_SZ
    /// 
    /// hex(3): REG_BINARY
    /// 
    /// hex(4): REG_DWORD
    /// 
    /// hex(5): REG_DWORD_BIG_ENDIAN - invalid type?
    /// 
    /// hex(6): REG_LINK
    /// 
    /// hex(7): REG_MULTI_SZ
    /// 
    /// hex(8): REG_RESOURCE_LIST
    /// 
    /// hex(9): REG_FULL_RESOURCE_DESCRIPTOR
    /// 
    /// hex(a): REG_RESOURCE_REQUIREMENTS_LIST
    /// 
    /// hex(b): REG_QWORD
    /// 
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

    /// Converts the byte arrays (`Vec<&str>`) into string
    /// 
    /// Hews close to C# implementation by using `string_builder` crate
    /// 
    /// # Arguments
    /// 
    /// * `string_array` - Vec array to convert
    /// * `encoding` - File Encoding
    /// 
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