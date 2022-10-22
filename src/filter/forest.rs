include!("../data/mod.rs");
include!("../detection/mod.rs");
include!("../service.rs");

use md5;
use std::fmt;
use std::boxed::Box;
use serde::Serialize;
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

#[derive(Serialize, Debug)]
pub struct Sieve {
    id: u64,
    target: String,
    dr_md5: String,  // dr means dense representation
    dr_dhash: u64,
    create_time: u64,
    property_map: String
}

impl Sieve {
    pub fn new(id: u64, target: &String, dr_md5: &String, dr_dhash: u64, property_map: &String, create_time: u64) -> Sieve {
        let sieve = Sieve{
            id: id,
            target: target.clone(),
            dr_md5: dr_md5.clone(),
            dr_dhash: dr_dhash,
            create_time: create_time,
            property_map: property_map.clone()
        };
        sieve
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
        for (id, target, dr_md5, dr_dhash, property_map, create_time) in store::read_sieves(filter_name) {
            let added_sieve = Sieve::new(id, &target, &dr_md5, dr_dhash, &property_map, create_time);
            sieves.insert(id, added_sieve);
            target_to_id.insert(target.clone(), id);
            trie_tree.insert(&target);
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
        let dr_md5 = format!("{:?}", md5::compute(target.as_bytes()));
        let dr_dhash: u64 = 0;
        if let Ok((id, create_time)) = store::add_sieve(&self.filter_name, target, &dr_md5, dr_dhash, property_map) {
            let added_sieve = Sieve::new(id, &target, &dr_md5, dr_dhash, property_map, create_time);
            self.sieves.insert(id, added_sieve);
            self.target_to_id.insert(target.clone(), id);
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

    pub fn detect(&mut self, filter_name: &String, content: &String) -> (Option<Vec<&Sieve>>, ServiceStatus) {
        if !self.filters.contains_key(filter_name) {
            return (None, ServiceStatus::FilterNotFoundError);
        }
        let filter = self.filters.get_mut(filter_name).unwrap();
        let matched_sieves = filter.detect(content);
        (Some(matched_sieves), ServiceStatus::Success)
    }

    pub fn add_sieve(&mut self, filter_name: &String, target: &String, property_map: &String) -> ServiceStatus {
        if !self.filters.contains_key(filter_name) {
            return ServiceStatus::FilterNotFoundError;
        }
        let filter = self.filters.get_mut(filter_name).unwrap();
        filter.add_sieve(target, property_map);
        ServiceStatus::Success
    }

    pub fn add_filter(&mut self, filter_type: &String, filter_name: &String, labels: &Vec<String>) -> ServiceStatus {
        if self.filters.contains_key(filter_name) {
            return ServiceStatus::FilterExistsError;
        }
        if *filter_type != FilterType::TextWordMatch.to_string() {
            return ServiceStatus::FilterTypeNotFoundError;
        }
        store::add_filter(filter_type, filter_name, labels);
        let filter = TextWordMatchFilter::new(filter_name, labels);
        self.filters.insert(filter_name.clone(), Box::new(filter));
        ServiceStatus::Success
    }
}
