include!("../data/mod.rs");
include!("../detection/mod.rs");
include!("../service.rs");

use md5;
use std::fmt;
use std::boxed::Box;
use serde::Serialize;
use std::collections::HashMap;
use trie::TrieTree;
use dhash::dhash;
use dhash::hamming_distance;
use image::load_from_memory;
use std::cmp::Ordering::Equal;
use std::str::FromStr;


#[derive(Debug)]
pub enum FilterType{
    TextWordMatch,
    ImageDhashMatch
}

impl FromStr for FilterType {
    type Err = ();

    fn from_str(input: &str) -> Result<FilterType, Self::Err> {
        match input {
            "TextWordMatch"  => Ok(FilterType::TextWordMatch),
            "ImageDhashMatch"  => Ok(FilterType::ImageDhashMatch),
            _ => Err(()),
        }
    }
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
    property_map: String,
    similarity: f32
}

impl Sieve {
    pub fn new(id: u64, target: &String, dr_md5: &String, dr_dhash: u64, property_map: &String, create_time: u64) -> Sieve {
        let sieve = Sieve{
            id: id,
            target: target.clone(),
            dr_md5: dr_md5.clone(),
            dr_dhash: dr_dhash,
            create_time: create_time,
            property_map: property_map.clone(),
            similarity: 1.0f32
        };
        sieve
    }

    pub fn set_similarity(&mut self, similarity: f32) {
        self.similarity = similarity;
    }
}

pub trait Filter: Send {
    fn count(&mut self) -> u64;
    fn add_sieve(&mut self, target: &String, property_map: &String) -> ServiceStatus;
    fn detect(&mut self, content: &String) -> (ServiceStatus, Vec<&Sieve>);
    fn calc_dhash(&mut self, target: &String) -> (ServiceStatus, u64);
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
        TextWordMatchFilter{
            filter_type: FilterType::TextWordMatch,
            sieves: sieves,
            target_to_id: target_to_id,
            filter_name: filter_name.clone(),
            labels: labels.clone(),
            trie_tree: trie_tree
        }
    }
}

pub struct ImageDhashMatchFilter{
    filter_type: FilterType,
    sieves: HashMap<u64, Sieve>,
    filter_name: String,
    labels: Vec<String>
}

impl ImageDhashMatchFilter {
    pub fn new(filter_name: &String, labels: &Vec<String>) -> ImageDhashMatchFilter {
        let mut sieves: HashMap<u64, Sieve> = HashMap::new();
        for (id, target, dr_md5, dr_dhash, property_map, create_time) in store::read_sieves(filter_name) {
            let added_sieve = Sieve::new(id, &target, &dr_md5, dr_dhash, &property_map, create_time);
            sieves.insert(id, added_sieve);
        }
        ImageDhashMatchFilter{
            filter_type: FilterType::ImageDhashMatch,
            sieves: sieves,
            filter_name: filter_name.clone(),
            labels: labels.clone()
        }
    }
}

impl Filter for ImageDhashMatchFilter {
    fn calc_dhash(&mut self, target: &String) -> (ServiceStatus, u64) {
        let mut dr_dhash: u64 = 18446744073709551615;
        let target_bytes = base64::decode(target);
        if let Err(_e) = target_bytes {
            return (ServiceStatus::InvalidImageError, dr_dhash)
        }
        let dyn_image = load_from_memory(&target_bytes.unwrap());
        if let Err(_e) = dyn_image {
            return (ServiceStatus::InvalidImageError, dr_dhash);
        }
        dr_dhash = dhash(&dyn_image.unwrap());
        (ServiceStatus::Success, dr_dhash)
    }

    fn count(&mut self) -> u64 {
        self.sieves.len() as u64
    }

    fn add_sieve(&mut self, target: &String, property_map: &String) -> ServiceStatus {
        let dr_md5 = format!("{:?}", md5::compute(target.as_bytes()));
        let (status, dr_dhash) = self.calc_dhash(target);
        if status != ServiceStatus::Success {
            return status;
        }
        if let Ok((id, create_time)) = store::add_sieve(&self.filter_name, target, &dr_md5, dr_dhash, property_map) {
            let added_sieve = Sieve::new(id, &target, &dr_md5, dr_dhash, property_map, create_time);
            self.sieves.insert(id, added_sieve);
            return ServiceStatus::Success;
        }
        ServiceStatus::SieveAddError
    }

    fn detect(&mut self, content: &String) -> (ServiceStatus, Vec<&Sieve>) {
        let mut matched_sieves: Vec<&Sieve> = Vec::new();
        let content_bytes = base64::decode(content);
        if let Err(_e) = content_bytes {
            return (ServiceStatus::InvalidImageError, matched_sieves)
        }
        let dyn_image = load_from_memory(&content_bytes.unwrap());
        if let Err(_e) = dyn_image {
            return (ServiceStatus::InvalidImageError, matched_sieves);
        }
        let dr_dhash = dhash(&dyn_image.unwrap());
        for sieve in self.sieves.values_mut() {
            let distance = hamming_distance(dr_dhash, sieve.dr_dhash);
            if distance <= 15 {
                sieve.set_similarity(1.0f32 - distance as f32 / 64f32);
                matched_sieves.push(sieve);
            }
        }
        matched_sieves.sort_by(|a, b| a.similarity.partial_cmp(&b.similarity).unwrap_or(Equal));
        matched_sieves.reverse();
        (ServiceStatus::Success, matched_sieves)
    }
}

impl Filter for TextWordMatchFilter {
    fn calc_dhash(&mut self, _target: &String) -> (ServiceStatus, u64) {
        let dr_dhash: u64 = 18446744073709551615;
        (ServiceStatus::Success, dr_dhash)
    }

    fn count(&mut self) -> u64 {
        self.sieves.len() as u64
    }

    fn add_sieve(&mut self, target: &String, property_map: &String) -> ServiceStatus {
        let dr_md5 = format!("{:?}", md5::compute(target.as_bytes()));
        let (_status, dr_dhash) = self.calc_dhash(target);

        if let Ok((id, create_time)) = store::add_sieve(&self.filter_name, target, &dr_md5, dr_dhash, property_map) {
            let added_sieve = Sieve::new(id, &target, &dr_md5, dr_dhash, property_map, create_time);
            self.sieves.insert(id, added_sieve);
            self.target_to_id.insert(target.clone(), id);
            self.trie_tree.insert(target);
            return ServiceStatus::Success;
        }
        ServiceStatus::SieveAddError
    }

    fn detect(&mut self, content: &String) -> (ServiceStatus, Vec<&Sieve>) {
        let matched_words = self.trie_tree.find(content);
        let mut matched_sieves: Vec<&Sieve> = Vec::new();
        for word in matched_words {
            let sieve_id = self.target_to_id.get(&word).unwrap();
            let matched_sieve = self.sieves.get(sieve_id).unwrap();
            matched_sieves.push(matched_sieve);
        }
        (ServiceStatus::Success, matched_sieves)
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
            } else if filter_type == FilterType::ImageDhashMatch.to_string() {
                let filter = ImageDhashMatchFilter::new(&raw_filter.2, &raw_filter.3);
                filters.insert(raw_filter.2.clone(), Box::new(filter));
            } else {
                println!("unsupported filter type: {:}", filter_type);
            }
        }
        FilterForest{filters: filters}
    }

    pub fn detect(&mut self, filter_name: &String, content: &String) -> (ServiceStatus, Vec<&Sieve>) {
        let matched_sieves: Vec<&Sieve> = Vec::new();
        if !self.filters.contains_key(filter_name) {
            return (ServiceStatus::FilterNotFoundError, matched_sieves);
        }
        let filter = self.filters.get_mut(filter_name).unwrap();
        filter.detect(content)
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
        if let Err(_e) = FilterType::from_str(filter_type) {
            return ServiceStatus::FilterTypeNotFoundError;
        }
        store::add_filter(filter_type, filter_name, labels);
        let filter = TextWordMatchFilter::new(filter_name, labels);
        self.filters.insert(filter_name.clone(), Box::new(filter));
        ServiceStatus::Success
    }
}
