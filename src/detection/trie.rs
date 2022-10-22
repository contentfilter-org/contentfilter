//  References
//   [1] https://en.wikipedia.org/wiki/Trie
//

use std::collections::HashMap;
use std::collections::HashSet;


pub struct TrieNode {
    value: Option<char>,
    children: HashMap<char, TrieNode>,
    is_word: bool
}

pub struct TrieTree {
    root: TrieNode,
    length: usize,
    depth: usize,
    word_to_subword_count: HashMap<String, u32>,
    subword_to_words: HashMap<String, Vec<String>>,
}

impl TrieNode {
    pub fn new(c: char, is_word: bool) -> TrieNode {
        TrieNode{
            value: Option::Some(c),
            children: HashMap::new(),
            is_word: is_word
        }
    }

    pub fn insert(&mut self, c: char, is_word: bool) {
        if self.children.contains_key(&c) == false {
            self.children.insert(c, TrieNode::new(c, is_word));
        }
    }
}

impl TrieTree {
    pub fn new() -> TrieTree {
        TrieTree { 
            root: TrieNode::new('\0', false), 
            length: 0, 
            depth: 0,
            word_to_subword_count: HashMap::new(),
            subword_to_words: HashMap::new()
        }
    }

    fn insert_subword(&mut self, sub_word: &String) {
        let mut next = &mut self.root;
        let tokens: Vec<char> = sub_word.chars().collect();
        let mut last_index = 0;
        for i in 0..tokens.len(){
            if next.children.contains_key(&tokens[i]) {
                next = next.children.get_mut(&tokens[i]).unwrap();
            } else {
                break;
            }
            last_index = last_index + 1;
        }
        if last_index != tokens.len() {
            for j in last_index..tokens.len() {
                next.insert(tokens[j], false);
                next = next.children.get_mut(&tokens[j]).unwrap();
            }
        }
        next.is_word = true;
    }

    pub fn insert(&mut self, word: &String) {
        let trimed_word =  word.chars().filter(|c| !c.is_whitespace()).collect::<String>();
        let cleaned_word = trimed_word.trim_end_matches("&").trim_start_matches("&");
        if cleaned_word.len() > 0 && !self.word_to_subword_count.contains_key(cleaned_word) {
            let subwords = cleaned_word.split("&").collect::<Vec<&str>>();
            for i in 0..subwords.len() {
                self.insert_subword(&subwords[i].to_string());
                let chars_count = subwords[i].chars().count();
                if chars_count > self.depth {
                    self.depth = chars_count;
                }
                if !self.subword_to_words.contains_key(subwords[i]) {
                    let words_list: Vec<String> = Vec::new();
                    self.subword_to_words.insert(subwords[i].to_string(), words_list);
                }
                let words_list = self.subword_to_words.get_mut(subwords[i]).unwrap();
                words_list.push(cleaned_word.to_string());
            }
            self.word_to_subword_count.insert(cleaned_word.to_string(), subwords.len() as u32);
            self.length += 1;
        }
    }

    fn find_subwords(&mut self, text: &String) -> Vec<String> {
        let text_tokens: Vec<char> = text.chars().collect();
        let mut matched_words_set: HashSet<String> = HashSet::new();
        for i in 0..text_tokens.len() {
            let mut next = &mut self.root;
            let mut word_chars: Vec<char> = Vec::new();
            for j in i..text_tokens.len() {
                if next.children.contains_key(&text_tokens[j]) {
                    next = next.children.get_mut(&text_tokens[j]).unwrap();
                    word_chars.push(next.value.unwrap());
                } else {
                    break;
                }
                if next.is_word {
                    matched_words_set.insert(word_chars.iter().collect());
                }
            }
        }
        Vec::from_iter(matched_words_set)
    }

    pub fn find(&mut self, text: &String) -> Vec<String> {
        let matched_subwords = self.find_subwords(text);
        let mut word_to_subwords: HashMap<String, HashSet<String>> = HashMap::new();
        for subword in matched_subwords {
            let words = self.subword_to_words.get(&subword).unwrap();
            for word in words {
                if !word_to_subwords.contains_key(word) {
                    let subwords: HashSet<String> = HashSet::new();
                    word_to_subwords.insert(word.clone(), subwords);
                }
                let subwords = word_to_subwords.get_mut(word).unwrap();
                subwords.insert(subword.clone());
            }
        }
        let mut matched_words: Vec<String> = Vec::new();
        for (word, subwords) in word_to_subwords {
            let word_subwords_count = self.word_to_subword_count.get(&word).unwrap();
            if *word_subwords_count == subwords.len() as u32 {
                matched_words.push(word)
            }
        }
        matched_words
    }
}
