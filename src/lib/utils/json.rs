use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::ops::Deref;
use crate::{BASE_URL, BASE_LIST};

use super::errors::DataError;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MDFile{
    pub id: u32,
    pub url: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JsonFile( HashMap<String, Vec<MDFile>>);


impl Deref for JsonFile {
    type Target = HashMap<String, Vec<MDFile>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub async fn get_section(section: &str) -> Result<Vec<MDFile>, DataError> {
    let url = format!("{}{}", BASE_URL, BASE_LIST);
    match reqwest::get(&url).await.map_err(|e| DataError::Web(e))?.json::<JsonFile>().await {
       Ok(list) => {
            let list: Option<&Vec<MDFile>> = list.get(section);
            match list{
                Some(value) => Ok(value.clone()),
                None => Err(DataError::Err("Section Not Found"))
            }
        }
        Err(e) => Err(DataError::Web(e))
    }
}


pub async fn get_url<'a>(id: u32, data: Vec<MDFile>) -> Result<String, DataError<'a>> {
    match data.iter().find(|f| f.id == id){
        Some(target) => Ok(target.url.clone()),
        None => Err(DataError::Err("field not found!"))
    }
}


pub async fn get_markdown(url: &str) -> Result<String, DataError> {
    let url = format!("{}{}", BASE_URL, url);
    match reqwest::get(&url).await.map_err(|e| DataError::Web(e))?.text().await {
       Ok(text) => Ok(text),
        Err(e) => Err(DataError::Web(e))
    }
}



pub async fn get_section_markdown(section: &str) -> Result<Vec<String>, DataError> {
    let sections_metadata = get_section(section).await?;
    let mut section_markdown: Vec<String> = vec![];
    for section_metadata in sections_metadata.iter(){
        if let Ok(text) = get_markdown(section_metadata.url.as_str()).await {
           section_markdown.push(text.to_owned()); 
        }
    }
    Ok(section_markdown)
}
