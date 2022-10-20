include!("../data/mod.rs");
include!("../detection/mod.rs");

use serde_json;
use md5;
use std::fmt;
use std::boxed::Box;
use std::time::Instant;
use std::collections::HashMap;
use text::trie::TrieTree;


#[derive(Debug)]
pub enum FilterType{
    TextWordMatch,
    ImageDhashMatch,
    ImageFaceSimilaityMatch 
}

impl fmt::Display for FilterType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Sieve {
    target: String,
    target_id: u64,
    target_md5: String,
    create_time: u64,
    property_map: String
}

impl Sieve {
    pub fn new(target_id: u64, target: &String, target_md5: &String, property_map: &String, create_time: u64) -> Sieve {
        let sieve = Sieve{
            target: target.clone(),
            target_id: target_id,
            target_md5: target_md5.clone(),
            create_time: create_time,
            property_map: property_map.clone()
        };
        sieve
    }

    fn id(&self) -> u64 {
        self.target_id
    }

    fn serilize(&self) -> serde_json::Value {
        let obj = serde_json::json!(
            {
                "target": self.target,
                "target_id": self.target_id,
                "target_md5": self.target_md5,
                "create_time": self.create_time,
                "property_map": self.property_map
            }
        );
        obj
    }
}

pub trait Filter: Send {
    fn count(&mut self) -> u64;
    fn add_sieve(&mut self, target: &String, property_map: &String);
    fn detect(&mut self, content: &String) -> Vec<&Sieve>;
}

pub struct TextWordMatchFilter{
    filter_type: FilterType,
    sieves: HashMap<u64, Sieve>,
    target_to_id: HashMap<String, u64>,
    filter_name: String,
    labels: Vec<String>,
    trie_tree: TrieTree
}

impl TextWordMatchFilter {
    pub fn new(filter_name: &String, labels: &Vec<String>) -> TextWordMatchFilter {
        let mut sieves: HashMap<u64, Sieve> = HashMap::new();
        let mut trie_tree = TrieTree::new();
        let mut target_to_id: HashMap<String, u64> = HashMap::new();
        for sieve_tup in store::read_sieves(filter_name) {
            let added_sieve = Sieve::new(
                sieve_tup.0, &sieve_tup.1, &sieve_tup.2, &sieve_tup.3, sieve_tup.4
            );
            sieves.insert(sieve_tup.0, added_sieve);
            target_to_id.insert(sieve_tup.1.clone(), sieve_tup.0);
            trie_tree.insert(&sieve_tup.1);
        }
        let filter = TextWordMatchFilter{
            filter_type: FilterType::TextWordMatch,
            sieves: sieves,
            target_to_id: target_to_id,
            filter_name: filter_name.clone(),
            labels: labels.clone(),
            trie_tree: trie_tree
        };
        filter
    }
}

impl Filter for TextWordMatchFilter {
    fn count(&mut self) -> u64 {
        self.sieves.len() as u64
    }

    fn add_sieve(&mut self, target: &String, property_map: &String) {
        let target_md5 = format!("{:?}", md5::compute(target.as_bytes()));
        if let Ok((sieve_id, create_time)) = store::add_sieve(&self.filter_name, target, &target_md5, property_map) {
            let added_sieve = Sieve::new(sieve_id, target, &target_md5, property_map, create_time);
            self.sieves.insert(sieve_id, added_sieve);
            self.target_to_id.insert(target.clone(), sieve_id);
            self.trie_tree.insert(target);
        }
    }

    fn detect(&mut self, content: &String) -> Vec<&Sieve> {
        let matched_words = self.trie_tree.find(content);
        let mut matched_sieves: Vec<&Sieve> = Vec::new();
        for word in matched_words {
            let sieve_id = self.target_to_id.get(&word).unwrap();
            let matched_sieve = self.sieves.get(sieve_id).unwrap();
            matched_sieves.push(matched_sieve);
        }
        matched_sieves
    }
}

#[derive(Debug)]
pub struct DetectError;

impl fmt::Display for DetectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "can not use filter to detect content!")
    }
}

pub struct FilterForest {
    filters: HashMap<String, Box<dyn Filter>>
}

impl FilterForest {
    pub fn new() -> FilterForest {
        store::init_forestdb();
        let raw_filters: Vec<(u64, String, String, Vec<String>, u64)> = store::read_filters();
        let mut filters: HashMap<String, Box<dyn Filter>> = HashMap::new();
        for i in 0..raw_filters.len() {
            let raw_filter = raw_filters.get(i).unwrap();
            let filter_type = raw_filter.1.clone();
            if filter_type == FilterType::TextWordMatch.to_string() {
                let filter = TextWordMatchFilter::new(&raw_filter.2, &raw_filter.3);
                filters.insert(raw_filter.2.clone(), Box::new(filter));
            } else {
                println!("unsupported filter type: {:}", filter_type);
            }
        }
        FilterForest{filters: filters}
    }

    pub fn detect(&mut self, filter_name: &String, content: &String) -> Result<serde_json::Value, DetectError> {
        let start_time = Instant::now();
        for key in self.filters.keys() {
            println!("{:}", key);
        }
        if !self.filters.contains_key(filter_name) {
            return Err(DetectError);
        }
        let filter = self.filters.get_mut(filter_name).unwrap();
        let macthed_sieves = filter.detect(content);
        let mut matched_sieves_json: Vec<serde_json::Value> = Vec::new();
        for sieve in macthed_sieves{
            matched_sieves_json.push(sieve.serilize());
        }
        let duration = start_time.elapsed();
        let resp = serde_json::json!(
            {
                "success": true,
                "hits": matched_sieves_json,
                "time": duration.as_secs_f32()
            }
        );
        Ok(resp)
    }

    pub fn add_sieve(&mut self, filter_name: &String, target: &String, property_map: &String) {
        if self.filters.contains_key(filter_name) {
            self.filters.get_mut(filter_name).unwrap().add_sieve(target, property_map);
        }
    }

    pub fn add_filter(&mut self, filter_type: &String, filter_name: &String, labels: &Vec<String>) {
        let twmstr = FilterType::TextWordMatch.to_string();
        if *filter_type == twmstr && !self.filters.contains_key(filter_name) {
            store::add_filter(filter_type, filter_name, labels);
            let filter = TextWordMatchFilter::new(filter_name, labels);
            self.filters.insert(filter_name.clone(), Box::new(filter));
        } else {
            println!("unsupported filter type or duplicated filter names!");
        }
    }
}
