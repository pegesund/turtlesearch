
use std::rc::Rc;
use std::cell::RefCell;
use im::HashMap;
use std::hash::Hash;
use std::convert::TryInto;
use std::ptr;
use std::borrow::{BorrowMut, Borrow};

use crate::sorted_vector::*;
use crate::structures::Document;


/// This file holds functions to add/remove a document to a field index with text content


/// very simple tokenizer, lower case and split on space
/// to be moved out and genralized later
pub fn simple_tokenizer(text: &str) -> Vec<String> {
    let text_without_special_chars: String = text.to_string().chars().enumerate().map(|(u, c)| c) .
        filter(|c| c.is_alphabetic() || c.is_digit(10) || c.is_whitespace()).collect();
    let text_vec = text_without_special_chars.to_lowercase().split(" ").map(|s| s.to_string()).collect();
    return text_vec;
}

/// Finds index in search index vector vector for a word
/// returns true and insert position if index not is found
fn find_pos(field_index: &FieldIndex<WordSorted>, w: &String) -> (usize, bool) {
    let ws_vec = field_index.get_vec().as_ref().borrow();
    let mut do_insert = false;
    let pos = match ws_vec.binary_search_by(|e| e.value.cmp(w)) {
        Ok(pos) => pos,
        Err(pos) =>  { do_insert = true; pos }
    };
    return (pos, do_insert)
}


/// Called once for each multifield value
fn add_single_text_to_field_index(text: &str, h: &mut HashMap<String, Rc<RefCell<Vec<u32>>>>, start: &u32) {
    let text_vec = simple_tokenizer(text);
    for i in 0..text_vec.len() {
        let w = text_vec[i].clone();
        if h.contains_key(&w) {
            let mut old =  h.get(&w).unwrap().as_ref().borrow_mut();
            old.push((i as u32) + *start);
        } else {
            let mut new_vec = Vec::new();
            new_vec.push((i as u32) + *start);
            h.insert(w, Rc::new(RefCell::new(new_vec)));
        }
    }
}

/// Add text content to a FieldIndex
pub fn add_multi_text_to_field_index(text: Vec<&str>, field_index: &FieldIndex<WordSorted>, doc: &mut Document) {

    let mut start: u32 = 0;
    let mut h: HashMap<String, Rc<RefCell<Vec<u32>>>> = HashMap::new();
    for doc_part in text {
        add_single_text_to_field_index(doc_part, &mut h, &start);
        start += 100;
    }

    for key in h.keys() {
        let (pos, do_insert) = find_pos(field_index, &key);
        if do_insert {
            field_index.insert(WordSorted {
                value: key.clone(),
                freq: 0,
                docs: Rc::new(RefCell::new(vec![])),
                optimized: false
            })
        }

        let mut words_sorted = field_index.index.as_ref().borrow_mut();
        let val = h.get(key).unwrap().as_ref().borrow();
        words_sorted[pos].freq += val.len() as u64;
        let dwi = DocumentWordIndex {
            doc_id: doc.id,
            position: Rc::new(RefCell::new(val.to_vec()))
        };
        words_sorted[pos].insert(dwi);
    }
}


/// delete all dwis connected to a doc from the field index
/// pretty slow as it iterates all dwis to to this
pub fn delete_document_from_field_index(field_index: &mut FieldIndex<WordSorted>, doc: &Document) {
    let mut remove_words = vec![];
    {
        let words_sorted = field_index.get_vec().as_ref().borrow_mut();
        for i in 0..words_sorted.len() {
            let word_sorted = &words_sorted[i];
            let mut word_sorted_children = word_sorted.get_vec().as_ref().borrow_mut();
            let mut cj: usize = 0;
            // println!("Number of children: {:?}", word_sorted_children.len());
            let children_len = word_sorted_children.len();
            let mut number_of_removed_dwis = 0;
            for j in 0.. children_len {
                if cj == children_len - number_of_removed_dwis { break }
                let dwi = &word_sorted_children[cj];
                if dwi.doc_id == doc.id {
                    word_sorted_children.remove(cj);
                    number_of_removed_dwis += 1;
                    if word_sorted_children.len() == 0 {
                        remove_words.push(word_sorted.value.clone());
                    } else {
                        cj += 1;
                    }
                }
            }
        }
    }

    for word_id in remove_words {
        field_index.delete(&WordSorted {
            value: word_id,
            freq: 0,
            docs: Rc::new(RefCell::new(vec![])),
            optimized: false
        })
    }
}

/// this counts number of total dwis in a field index
/// deletes words also if the last dwi is deleted
/// used only for statistics
pub fn count_number_of_dwis_in_field_index(field_index: &FieldIndex<WordSorted>) -> u64 {
    let mut counter: u64 = 0;
    let words_sorted = field_index.get_vec().as_ref().borrow();
    for i in 0..words_sorted.len() {
        let word_sorted = &words_sorted[i];
        let word_sorted_children = word_sorted.get_vec().as_ref().borrow();
        for j in 0..word_sorted_children.len() {
            let dwi = &word_sorted_children[j];
            counter += 1;
        }
    }
    return counter;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structures::Document;

    #[test]
    fn test_add_text_to_field() {
        let ws = WordSorted {
            value: "".to_string(),
            freq: 0,
            docs: Rc::new(RefCell::new(vec![])),
            optimized: false
        };
        let t = "This is Petter writing. This is a test.";
        let mut field_index = FieldIndex {
            name: "".to_string(),
            index: Rc::new(RefCell::new(vec![]))
        };

        let t1 = "This is Petter writing. This is a test.";
        let t2 = "This is Petter writing. This is a test.";
        let mut string_vec = vec![];
        string_vec.push(t1);
        string_vec.push(t2);

        let mut doc = Document {
            id: 88,
            len: 99
        };
        add_multi_text_to_field_index(string_vec, &mut field_index, &mut doc);
        let children = field_index.get_vec().as_ref().borrow();
        assert_eq!(children.len(), 6);
        let all_dwi_for_the_a_word = children[0].get_vec().as_ref().borrow();
        let all_positions_for_the_a_word = all_dwi_for_the_a_word[0].get_vec().as_ref().borrow();
        assert_eq!(all_positions_for_the_a_word.to_vec(), vec![6,106]);
        assert_eq!(children[0].freq, 2); // there should be two a
        assert_eq!(children[1].freq, 4); // there should 4 of is
    }

    #[test]
    fn test_delete_doc_from_field_index() {
        let mut field_index = FieldIndex {
            name: "".to_string(),
            index: Rc::new(RefCell::new(vec![]))
        };

        let t1 = "This is Petter writing. This is a test newword.";
        let t2 = "This is Petter writing. This is a test.";
        let mut string_vec1 = vec![];
        let mut string_vec2 = vec![];
        string_vec1.push(t1);
        string_vec2.push(t2);

        let mut doc1 = Document {
            id: 88,
            len: 99
        };

        let mut doc2 = Document {
            id: 888,
            len: 999
        };

        add_multi_text_to_field_index(string_vec1, &field_index, &mut doc1);
        add_multi_text_to_field_index(string_vec2, &field_index, &mut doc2);
        let c1 = count_number_of_dwis_in_field_index(&field_index);
        // println!("Petters Field index: {:#?}", &field_index);
        delete_document_from_field_index(&mut field_index, &doc1);
        let c2 = count_number_of_dwis_in_field_index(&field_index);
        assert_eq!(c1, 13);
        assert_eq!(c2, 6);
    }
}
