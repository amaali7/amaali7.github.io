use std::fmt::Display;
use serde_json::Error as JError;

#[derive(Debug)]
pub enum DataError<'a> {
    Web(reqwest::Error),
    Json(JError),
    Err(&'a str)
}

impl <'a> Display for DataError<'a> {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match self {
           DataError::Web(e) => f.write_str(format!("{}", e).as_str()),
            DataError::Json(e) => f.write_str(format!("{}", e).as_str()),
            DataError::Err(e) => f.write_str(e)
       }
   } 
}

impl<'a> std::error::Error for DataError<'a>{}
