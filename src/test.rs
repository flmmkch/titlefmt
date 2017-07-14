use super::*;

use std::collections::HashMap;

pub struct MetadataProvider<'a> {
    metadata_dict: HashMap<&'a str, &'a str>,
}

impl<'a> metadata::Provider for MetadataProvider<'a> {
    fn tag_value(&self, key: &str) -> Option<String> {
        let entry = self.metadata_dict.get(key);
        if let Some(value) = entry {
            let s = value.to_string();
            Some(s)
        }
        else {
            None
        }
    }
}

impl<'a> MetadataProvider<'a> {
    pub fn new(metadata_dict: HashMap<&'a str, &'a str>) -> MetadataProvider<'a> {
        MetadataProvider {
            metadata_dict,
        }
    }
}
