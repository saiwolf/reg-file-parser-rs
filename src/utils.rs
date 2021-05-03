#![allow(dead_code)]

use regex::Regex;

/// Strips specified leading chars from a `&str` and returns the result as a `String`
/// 
/// * `line` - The line to strip
/// * `lead_char` - The starting string to strip
/// 
pub fn strip_leading_chars(line: &str, lead_char: &str) -> String {
    let tmp_string = line.trim();
    if tmp_string.starts_with(lead_char) && tmp_string.ends_with(lead_char)
    {
        tmp_string[1..tmp_string.len() - 2].to_string()
    }
    else {
        tmp_string.to_string()
    }
}

/// Strips braces ('[' and ']') from a given `&str` and returns the result as a `String`
/// 
/// * `line` - The line to strip
/// 
pub fn strip_braces(line: &str) -> String {
    let value = line.trim();
    if line.starts_with('[') && line.ends_with(']')
    {
        value[1..value.len() - 2].to_string()
    }
    else {
        value.to_string()
    }
}

/// Removes the ending backslashes from the given `&str` and returns the result as a `String`
/// 
/// * `line` - The line to strip
/// 
pub fn strip_continue_char(line: &'static str) -> String {
    let re = Regex::new(r"\\\\\r\n[ ]*").expect("Error parsing Regex");
    re.replace(line, " ").to_string()
}

/// Parses raw registry file content to determine encoding from registry file version.
/// 
/// Parsing is done via regex with the `regex` crate.
/// 
/// # Arguments
/// 
/// * `content` - Raw file contents to parse
/// 
/// # Notes
/// 
/// If the string "REGEDIT4" is found in `content`, then the encoding is set to ANSI, otherwise it is set to UTF8
/// 
pub fn get_encoding(content: String) -> String {
    let re = Regex::new(r"(?i)([ ]*(\r\n)*)(?i)REGEDIT4").unwrap();
    match re.is_match(&content[..]) {
        true => "ANSI".to_string(),
        false => "UTF8".to_string(),
    }
}