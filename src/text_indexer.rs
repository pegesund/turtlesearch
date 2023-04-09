use std::collections::HashMap;
use std::hash::Hash;
use std::convert::TryInto;
use std::ptr;

use crate::field_indexer::PlainContent;
use crate::sorted_vector::*;
use crate::structures::{DocumentWordAndPositions, FieldIndex, Field};


/// This file holds functions to add/remove a document to a field index with text content


/// very simple tokenizer, lower case and split on space
/// to be moved out and genralized later
pub fn simple_tokenizer(text: &str) -> Vec<String> {
    let text_without_special_chars: String = text.to_string().chars().enumerate().map(|(u, c)| c) .
        filter(|c| c.is_alphabetic() || c.is_digit(10) || c.is_whitespace()).collect();
    let text_vec: Vec<String> = text_without_special_chars.to_lowercase().split(" ").map(|s | s.to_string()).collect();
    return text_vec
}

/// Finds index in search index vector vector for a word
/// returns true and insert position if index not is found
fn find_pos(field_index: &mut FieldIndex<WordSorted>, w: &String) -> (usize, bool) {
    let mut do_insert = false;
    let pos = match field_index.get_vec().binary_search_by(|e| e.value.cmp(w)) {
        Ok(pos) => pos,
        Err(pos) =>  { do_insert = true; pos }
    };
    return (pos, do_insert)
}


/// Called once for each multifield value
/// Add each word in the doc to the hash, and add the position of the word to the word entry
fn add_single_text_to_field_index(text_vec: &Vec<String>, h: &mut HashMap<String, Vec<u32>>, start: &u32) {
    for i in 0..text_vec.len() {
        let w = text_vec[i].clone();
        if h.contains_key(&w) {
            let old =  h.get_mut(&w).unwrap();
            old.push((i as u32) + start);
        } else {
            let mut new_vec = Vec::new();
            new_vec.push((i as u32) + start);
            h.insert(w, new_vec);
        }
    }
}

/// Add text content to a FieldIndex
/// For each text add 10 to position to avoid separate texts being positioned next to each other
pub fn add_multi_text_to_field_index(text: &Vec<Vec<String>>, field_index: &mut FieldIndex<WordSorted>, doc: u64) {

    let mut start: u32 = 0;
    let mut h: HashMap<String, Vec<u32>> = HashMap::new();
    for doc_part in text {
        add_single_text_to_field_index(doc_part, &mut h, &start);
        start += 10;
    }

    for key in h.keys() {
        let (pos, do_insert) = find_pos(field_index, &key);
        if do_insert {
            field_index.insert(WordSorted {
                value: key.clone(),
                freq: 0,
                docs: vec![],
                optimized: false
            })
        }

        // let &mut words_sorted = &mut field_index.index;
        let val = h.get(key).unwrap();
        field_index.index[pos].freq += val.len() as u64;
        let dwi = DocumentWordAndPositions {
            doc_id: doc,
            position: val.to_vec()
        };
        field_index.index[pos].insert(dwi);
    }
}


pub fn delete_word_from_index(field_index: &mut FieldIndex<WordSorted>, letter: String) {
    field_index.delete(&WordSorted {
        value: letter,
        freq: 0,
        docs: vec![],
        optimized: false
    })
}

/// delete all dwis connected to a doc from the field index
/// pretty slow as it iterates all dwis to to this
/// TODO: Fix speed by looking up each word instead of iterating all
pub fn delete_document_from_field_index(field_index: &mut FieldIndex<WordSorted>, doc: u64) {
    let mut remove_words = vec![];
    {
        
        for i in 0..field_index.get_vec().len() {
            // let word_sorted = field_index.get_vec()[i];
            let mut cj: usize = 0;
            let children_len = field_index.get_vec()[i].get_vec().len() ;
            let mut number_of_removed_dwis = 0;
            for j in 0.. children_len {
                if cj == children_len - number_of_removed_dwis { break }
                let other_doc_id = field_index.get_vec()[i].get_vec()[cj].doc_id ;
                if other_doc_id == doc {
                    field_index.get_vec()[i].get_vec().remove(cj);
                    field_index.get_vec()[i].freq -= 1;
                    number_of_removed_dwis += 1;
                    if field_index.get_vec()[i].get_vec().len() == 0 {
                        remove_words.push(field_index.get_vec()[i].value.clone());
                    } 
                }                
                cj += 1;
            }
        }
    }

    for word_id in remove_words {
        delete_word_from_index(field_index, word_id);
    }
}

/// this counts number of total dwis in a field index
/// deletes words also if the last dwi is deleted
/// used only for statistics
pub fn count_number_of_dwis_in_field_index(field_index: &mut FieldIndex<WordSorted>) -> u64 {
    let mut counter: u64 = 0;
    let words_sorted = &mut field_index.get_vec();
    for i in 0..words_sorted.len() {
        let word_sorted = &mut words_sorted[i];
        let word_sorted_children = word_sorted.get_vec();
        for j in 0..word_sorted_children.len() {
            let dwi = &word_sorted_children[j];
            counter += 1;
        }
    }
    return counter;
}


impl PlainContent<Vec<Vec<String>>> for FieldIndex<WordSorted> {
    fn put_content(&mut self, content: Vec<Vec<String>>, doc_id: u64) {
        add_multi_text_to_field_index(&content, self, doc_id);
    }

    fn get_ids(&mut self, content: Vec<Vec<String>>) -> Vec<u64> {
        vec![]
    }

    fn delete_doc_id(&mut self, doc_id: u64) {
        todo!()
    }
}


impl PlainContent<String> for FieldIndex<WordSorted> {
    fn put_content(&mut self, content: String, doc_id: u64) {
        let t = simple_tokenizer("This is Petter writing. This is a test.");
        let mut string_vec = vec![];
        string_vec.push(t);
        add_multi_text_to_field_index(&string_vec, self, doc_id);
    }


    fn get_ids(&mut self, content: String) -> Vec<u64> {
        let children = self.get_vec();
        return match children.binary_search_by(|e| e.value.cmp(&content)) {
            Ok(pos) => {
                let docs_and_pos: Vec<DocumentWordAndPositions>= children[pos].docs.to_vec();
                docs_and_pos.iter().map(|d| d.doc_id).collect()
            },
            Err(pos) => vec![]
        };
    }

    fn delete_doc_id(&mut self, doc_id: u64) {
        delete_document_from_field_index(self, doc_id);
    }
}




#[cfg(test)]
mod tests {
    use crate::field_indexer;

    use super::*;

    #[test]
    fn test_add_text_to_field() {

        // let t = "This is Petter writing. This is a test.";
        let mut field_index = FieldIndex {
            name: "".to_string(),
            index: vec![]
        };

        let t1 = simple_tokenizer("This is Petter writing. This is a test.");
        let t2 = simple_tokenizer("This is Petter writing. This is a test.");
        let mut string_vec = vec![];
        string_vec.push(t1);
        string_vec.push(t2);

        let doc = 88;
        add_multi_text_to_field_index(&string_vec, &mut field_index, doc);
        let children = field_index.get_vec();
        assert_eq!(children.len(), 6);
        let all_dwi_for_the_a_word = children[0].get_vec();
        let all_positions_for_the_a_word = all_dwi_for_the_a_word[0].get_vec();
        assert_eq!(all_positions_for_the_a_word.to_vec(), vec![6,16]);
        assert_eq!(children[0].freq, 2); // there should be two a
        assert_eq!(children[1].freq, 4); // there should 4 of is
    }

    #[test]
    fn test_delete_doc_from_field_index() {
        let mut field_index = FieldIndex {
            name: "".to_string(),
            index: vec![]
        };

        let t1 = simple_tokenizer("This is Petter writing. This is a test newword.");
        let t2 = simple_tokenizer("This is Petter writing. This is a test.");
        let mut string_vec1 = vec![];
        let mut string_vec2 = vec![]; 
        string_vec1.push(t1);
        string_vec2.push(t2);

        let doc1 = 88;
        let doc2 = 888;

        add_multi_text_to_field_index(&string_vec1, &mut field_index, doc1);
        add_multi_text_to_field_index(&string_vec2, &mut field_index, doc2);
        let c1 = count_number_of_dwis_in_field_index(&mut field_index);
        delete_document_from_field_index(&mut field_index, doc1);
        let c2 = count_number_of_dwis_in_field_index(&mut field_index);
        assert_eq!(c1, 13);
        assert_eq!(c2, 6);
    }

    #[test]
    fn test_plain_text_trait() {
        let mut field_index = FieldIndex {
            name: "".to_string(),
            index: vec![]
        };
        let t1 = simple_tokenizer("a");
        let t2 = simple_tokenizer("b a d");
        let t3 = simple_tokenizer("c a");
        let mut string_vec = vec![];
        string_vec.push(t1);
        let mut string_vec2 = vec![];
        string_vec2.push(t2);
        let mut string_vec3 = vec![];
        string_vec3.push(t3);
        field_index.put_content(string_vec, 100);
        field_index.put_content(string_vec2, 101);
        field_index.put_content(string_vec3, 102);
        assert_eq!(field_index.get_ids( String::from("a")), vec![100,101,102]);
        assert_eq!(field_index.get_ids( String::from("b")), vec![101]);
        assert_eq!(field_index.get_ids( String::from("d")), vec![101]);
        PlainContent::<String>::delete_doc_id(&mut field_index, 101);
        println!("New index: {:?}", field_index);
        assert_eq!(field_index.get_ids( String::from("a")), vec![100, 102]);
        let empty: Vec<u64> = vec![];
        assert_eq!(field_index.get_ids( String::from("b")), empty);
        assert_eq!(field_index.get_ids( String::from("d")), empty);
    }
}
