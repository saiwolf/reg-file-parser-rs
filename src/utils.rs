use regex::Regex;

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

pub fn strip_continue_char(line: &'static str) -> String {
    let re = Regex::new(r"\\\\\r\n[ ]*").expect("Error parsing Regex");
    re.replace(line, " ").to_string()
}