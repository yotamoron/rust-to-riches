/*
 * 10. Custom Error Types
 *   Write a program that simulates reading a configuration file. 
 *   Create a custom enum called ConfigError with variants for FileNotFound, 
 *   PermissionDenied, and ParseError. Implement the std::error::Error trait for it. 
 *   Write a function returning Result<Config, ConfigError> that purposefully 
 *   triggers one of these errors, and use the ? operator to propagate it.
 */

use std::error::Error;
use std::fmt;
use std::fmt::Debug;

enum ConfigError {
    FileNotFound(String),
    PermissionDenied(String),
    ParseError(String)
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Config Error: {}", self)
        }    
}

impl Debug for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FileNotFound(arg0) => f.debug_tuple("FileNotFound").field(arg0).finish(),
            Self::PermissionDenied(arg0) => f.debug_tuple("PermissionDenied").field(arg0).finish(),
            Self::ParseError(arg0) => f.debug_tuple("ParseError").field(arg0).finish(),
        }
    }
}

impl Error for ConfigError {}

fn fnf() -> Result<String, ConfigError> {
    Err(ConfigError::FileNotFound("File not found".to_string()))
}

fn pd() -> Result<String, ConfigError> {
    Err(
        ConfigError::PermissionDenied(
            "Permission denied".to_string()
        )
    )
}

fn pe() -> Result<String, ConfigError> {
    Err(ConfigError::ParseError("Parse Error".to_string()))
}

fn main() {
    println!("{:?} {:?} {:?}", fnf().err().unwrap(), pd().err().unwrap(), pe().err().unwrap());
}
