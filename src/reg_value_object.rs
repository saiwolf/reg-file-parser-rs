use std::fmt;

pub struct RegValueObject {
    pub root: &'static str,
    pub parent_key: &'static str,
    pub parent_key_without_root: &'static str,
    pub entry: &'static str,
    pub value: &'static str,
    pub r#type: &'static str,
}

impl Default for  RegValueObject {
    fn default() -> RegValueObject {
        RegValueObject {
            root: "",
            parent_key: "",
            parent_key_without_root: "",
            entry: "",
            value: "",
            r#type: "",
        }
    }    
}

impl fmt::Display for RegValueObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\\\\{}={}{}", self.parent_key, self.entry, self.set_reg_entry_type(self.r#type), self.value)
    }
}

impl RegValueObject {
    pub fn new(&self, reg_key_name: &'static str, reg_value_name: &'static str, reg_value_data: &'static str, encoding: &'static str) -> RegValueObject {
        RegValueObject {
            parent_key: reg_key_name.trim(),
            parent_key_without_root: self.parent_key,
            root: self.get_hive(&self.parent_key_without_root),
            entry: reg_value_name,
            r#type: self.get_reg_entry_type(&reg_value_data, encoding),
            value: reg_value_data,
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

    fn get_reg_entry_type(&self, text_line: &str, text_encoding: &str) -> &'static str {
        ""
    }

    fn set_reg_entry_type(&self, reg_data_type: &str) -> &'static str {
        ""
    }
}