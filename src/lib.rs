#![warn(missing_docs)]

//! A Rust port of [Registry Export File Parser](https://www.codeproject.com/Tips/125573/Registry-Export-File-reg-Parser) in C# by [Henryk Filipowicz](https://www.codeproject.com/script/Membership/View.aspx?mid=7577019).
//! 
//! Parses a Windows Registry Export (.reg) file and returns an object containing
//! the keys and values.
//! 

extern crate regex;
extern crate string_builder;
extern crate snafu;

/**
Custom App Errors module
 */
pub mod errors;

/// Registry File Object.
/// 
/// Contains the data parsed from the .reg file.
/// 
pub mod reg_file_object;

/// Registry Value Object
/// 
/// Represents the structure of a Registry Value and includes
/// functions to parse and retrieve said values from the .reg file
pub mod reg_value_object;

/// Select utilities that do not fit in any other module.
mod utils;

pub use reg_value_object::*;
pub use reg_file_object::*;
pub use errors::*;