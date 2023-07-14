//! This file contains code to add float/int/date to a field_index
//! for indexing of text content, look into the text_indexer
#![allow(clippy::transmute_num_to_bytes)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::explicit_counter_loop)]

use crate::structures::*;
use std::fmt::Debug;
use duplicate::{duplicate, duplicate_item};
use crate::sorted_vector::*;



pub trait PlainContent<G: Clone + Debug + Ord> {
    fn put_content(&mut self, content: G, doc_id: u64);
    fn get_ids(&mut self, content: G) -> Vec<u64>;
    fn delete_doc_id(&mut self, doc_id: u64);
}
 
#[duplicate_item(
the_class val_type;
[ IntegerSorted  ] [ i64 ];
[ FloatSorted  ] [ FloatWrapper ]; 
[ DateSorted  ] [ u64 ];
[ BoolSorted  ] [ bool ];
)]

#[allow(unused_assignments)]
impl PlainContent<val_type> for FieldIndex<the_class> {

    /// adds content to a index
    fn put_content(&mut self, content: val_type, doc_id: u64) {
        let mut do_insert:bool = false;
        {
            let children = self.get_vec();
            do_insert = match children.binary_search_by(|e| e.value.cmp(&content)) {
                Ok(pos) =>  {
                    let old_content_sorted = &mut children[pos];
                    old_content_sorted.insert(doc_id);
                    false
                }
                Err(pos) => true
            };
        }

        if do_insert {
            let element = the_class {
                value: content,
                doc_ids: vec![doc_id]
            };
            self.insert(element);
        }
    }

    /// get docs based on value query
    fn get_ids(&mut self, content: val_type) ->  Vec<u64> {
        let children = self.get_vec();
        match children.binary_search_by(|e| e.value.cmp(&content)) {
            Ok(pos) => children[pos].doc_ids.to_vec(),
            Err(pos) => vec![]
        }
    }

    /// delete doc from index
    /// pretty slow as it iterates all index to find the docs
    /// TODO: Fix speed
    fn delete_doc_id(&mut self, doc_id: u64) {
        let mut empty_values = vec![];
        {
            let children = self.get_vec();
            for i in 0..children.len() {
                let child = &mut children[i];
                let docs = child.get_vec();
                match docs.binary_search_by(|e| e.cmp(&doc_id)) {
                            Ok(pos) => {
                                docs.remove(pos);
                                if docs.is_empty() {
                                    empty_values.push(i);
                                }
                            },
                            Err(pos) => ()
                        };
            }
        }
        let children = self.get_vec();
        let mut i = 0;
        for doc_idx in empty_values {
            children.remove(doc_idx - i);
            i += 1;
        }
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_content_date() {
        let mut field_index = FieldIndex {
            name: "myfield".to_string(),
            index: vec![]
        };

        field_index.put_content(99_i64, 199);
        field_index.put_content(98_i64, 198);
        field_index.put_content(100_i64, 200);
        field_index.put_content(100_i64, 201);
        field_index.put_content(99_i64, 300);

        let d_99 = field_index.get_ids(99);
        assert_eq!(d_99, vec![199,300]);

        let mut field_index_float = FieldIndex {
            name: "myfield_float".to_string(),
            index: vec![]
        };

        field_index_float.put_content(FloatWrapper{value: 88.9}, 188);
        field_index_float.put_content(FloatWrapper{value: 39.9}, 139);
        field_index_float.put_content(FloatWrapper{value: 88.9}, 288);
        field_index_float.put_content(FloatWrapper{value: 99.9}, 199);

        assert_eq!(field_index_float.get_ids(FloatWrapper{value: 88.9}), vec![188,288]);

        let mut field_index_bool = FieldIndex {
            name: "myfield_bool".to_string(),
            index: vec![]
        };

        field_index_bool.put_content(true, 1);
        field_index_bool.put_content(false, 2);
        field_index_bool.put_content(true, 3);
        field_index_bool.put_content(false, 4);

        assert_eq!(field_index_bool.get_ids(true), vec![1,3]);

        field_index.delete_doc_id(201);
        assert_eq!(field_index.get_ids(100), vec![200]);
        field_index.delete_doc_id(200);
        let emtpy: Vec<u64> = vec![];
        assert_eq!(field_index.get_ids(100), emtpy);
    }
}
