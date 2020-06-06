use crate::structures::{FieldIndex, WordSorted, DocumentWordIndex, HasChildrenNew, Document};
use std::rc::Rc;
use std::cell::RefCell;
use im::HashMap;
use std::hash::Hash;
use std::convert::TryInto;
use std::ptr;
use std::borrow::{BorrowMut, Borrow};

/// very simple tokenizer, lower case and split on space
fn simple_tokenizer(text: &str) -> Vec<String> {
    let text_without_special_chars: String = text.to_string().chars().enumerate().map(|(u, c)| c) .
        filter(|c| c.is_alphabetic() || c.is_digit(10) || c.is_whitespace()).collect();
    let text_vec = text_without_special_chars.to_lowercase().split(" ").map(|s| s.to_string()).collect();
    return text_vec;
}


fn find_pos(field_index: &FieldIndex<WordSorted>, w: &String) -> (usize, bool) {
    let ws_vec = field_index.get_vec().as_ref().borrow();
    let mut do_insert = false;
    let pos = match ws_vec.binary_search_by(|e| e.value.cmp(w)) {
        Ok(pos) => pos,
        Err(pos) =>  { do_insert = true; pos }
    };
    return (pos, do_insert)
}


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

fn add_multi_text_to_field_index(text: Vec<&str>, field_index: &mut FieldIndex<WordSorted>, doc: &mut Document) {

    let mut start: u32 = 0;
    let mut h: HashMap<String, Rc<RefCell<Vec<u32>>>> = HashMap::new();
    for doc_part in text {
        add_single_text_to_field_index(doc_part, &mut h, &start);
        start += 100;
    }

    for key in h.keys() {
        let (pos, do_insert) = find_pos(field_index, &key);
        if do_insert {
            println!("----------------- Inserting: {:?}", &key);
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
            id: 0,
            position: Rc::new(RefCell::new(val.to_vec())),
            doc: &mut *doc
        };
        words_sorted[pos].insert(dwi);
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::structures::Document;

    #[test]
    fn test_add_text_to_field() {

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
        let possible_doc = all_dwi_for_the_a_word[0].doc;
        println!("Field index: {:#?}", &all_dwi_for_the_a_word);
        unsafe {
            let doc2 :&mut Document = &mut *possible_doc ;
            assert_eq!(doc2.len, 99);
         }
    }
}
