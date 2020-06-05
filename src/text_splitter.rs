use crate::structures::{FieldIndex, WordSorted, DocumentWordIndex, HasChildrenNew};
use std::rc::Rc;
use std::cell::RefCell;
use im::HashMap;
use std::hash::Hash;
use std::convert::TryInto;

/// very simple tokenizer, lower case and split on space
fn simple_tokenizer(text: &str) -> Vec<String> {
    let text_vec = text.to_lowercase().split(" ").map(|s| s.to_string()).collect();
    return text_vec;
}

fn add_text_to_field_index(text: &str, mut word_sorted: WordSorted) {
    let text_vec = simple_tokenizer(text);
    let mut h: HashMap<String, Rc<RefCell<Vec<u64>>>> = HashMap::new();
    for i in 0..text_vec.len() {
        let w = text_vec[i].clone();
        if h.contains_key(&w) {
            let mut old =  h.get(&w).unwrap().borrow_mut();
            old.push(i as u64);
        } else {
            let mut new_vec = Vec::new();
            new_vec.push(i as u64);
            h.insert(w, Rc::new(RefCell::new(new_vec)));
        }
        println!("Hashmap: {:?}", h);
    }
    for (key, value) in h {
        let freq = value.borrow();
        let number_of_word_occurences = freq.len() as u64;
        let dwi = DocumentWordIndex {
            id: 0,
            position: Rc::new(RefCell::new(vec![])),
            freq: number_of_word_occurences
        };
        word_sorted.insert(dwi);
        word_sorted.freq += number_of_word_occurences;

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_text_to_field() {
        let ws = WordSorted {
            value:"myfield".to_string(),
            freq: 0,
            docs: Rc::new(RefCell::new(vec![]))
        };
        let t = "This is Petter writing. This is a test.";
        add_text_to_field_index(&t, ws);
    }
}