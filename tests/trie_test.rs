include!("../src/detection/trie.rs");


#[test]
fn trie_build_1() {
    let mut tree: TrieTree = TrieTree::new();
    let words = vec!["售卖&毒".to_string(), "小姐姐加微信".to_string(), "马冰".to_string()];
    for word in words {
        tree.insert(&word);
    }
    assert_eq!(tree.depth, 6);
    assert_eq!(tree.length, 3);
}

#[test]
fn trie_build_2() {
    let mut tree: TrieTree = TrieTree::new();
    let words = vec!["&& &   ".to_string()];
    for word in words {
        tree.insert(&word);
    }
    assert_eq!(tree.depth, 0);
    assert_eq!(tree.length, 0);
}

#[test]
fn trie_build_3() {
    let mut tree: TrieTree = TrieTree::new();
    let words = vec!["小姐姐&加微信".to_string(), "小姐姐&加微信  &".to_string(), "    小姐姐&加微信  &  ".to_string()];
    for word in words {
        tree.insert(&word);
    }
    assert_eq!(tree.depth, 3);
    assert_eq!(tree.length, 1);
}

#[test]
fn trie_find_1() {
    let mut tree: TrieTree = TrieTree::new();
    let words = vec!["售卖&毒".to_string(), "小姐姐".to_string(), "马冰".to_string()];
    for word in words {
        tree.insert(&word);
    }
    let matched_words = tree.find(&"马克准备约小姐姐一起售卖冰毒。".to_string());
    let right_matched_words = Vec::from_iter(vec!["售卖&毒".to_string(), "小姐姐".to_string()]);
    let mut mut_matched_words = matched_words.clone();
    let mut mut_right_matched_words = right_matched_words.clone();
    assert_eq!(mut_matched_words.sort(), mut_right_matched_words.sort());
}

#[test]
fn trie_find_2() {
    let mut tree: TrieTree = TrieTree::new();
    let words = vec![];
    for word in words {
        tree.insert(&word);
    }
    let matched_words = tree.find(&"马克准备约小姐姐一起售卖冰毒。".to_string());
    let right_matched_words:Vec<String> = Vec::new();
    let mut mut_matched_words = matched_words.clone();
    let mut mut_right_matched_words = right_matched_words.clone();
    assert_eq!(mut_matched_words.sort(), mut_right_matched_words.sort());
}