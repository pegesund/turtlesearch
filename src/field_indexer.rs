//! This file contains code to add float/int/date to a field_index
//! for indexing of text content, look into the text_indexer

use crate::structures::*;
use std::fmt::Debug;
use std::rc::Rc;
use std::cell::RefCell;
use duplicate::duplicate;
use crate::sorted_vector::*;



pub trait PlainContent<G: Clone + Debug + Ord> {
    fn put_content(&self, content: G, doc_id: u64);
    fn get_ids(&self, content: G) -> Option<Vec<u64>>;
    fn delete_doc(&self, doc_id: u64);
}
 
#[duplicate(
the_class val_type;
[ IntegerSorted  ] [ i64 ];
[ FloatSorted  ] [ FloatWrapper ];
[ DateSorted  ] [ u64 ];
[ BoolSorted  ] [ bool ];
)]

#[allow(unused_assignments)]
impl PlainContent<val_type> for FieldIndex<the_class> {

    /// adds content to a index
    fn put_content(&self, content: val_type, doc_id: u64) {
        let mut do_insert = false;
        {
            let mut children = self.get_vec().as_ref().borrow_mut();
            do_insert = match children.binary_search_by(|e| e.value.cmp(&content)) {
                Ok(pos) =>  {
                    let old_date_sorted = &mut children[pos];
                    old_date_sorted.insert(doc_id);
                    false
                }
                Err(pos) => true
            };
        }

        if do_insert == true {
            let element = the_class {
                value: content,
                doc_ids: Rc::new(RefCell::new(vec![doc_id]))
            };
            self.insert(element);
        }
    }

    /// get docs based on value query
    fn get_ids(&self, content: val_type) -> Option<Vec<u64>> {
        let children = self.get_vec().as_ref().borrow();
        return match children.binary_search_by(|e| e.value.cmp(&content)) {
            Ok(pos) => Some(children[pos].doc_ids.as_ref().borrow_mut().to_vec()),
            Err(pos) => None
        };
    }

    /// delete doc from index
    /// pretty slow as it iterates all index to find the docs
    /// TODO: Fix speed
    fn delete_doc(&self, doc_id: u64) {
        let mut empty_values = vec![];
        {
            let mut children = self.get_vec().as_ref().borrow_mut();
            for i in 0..children.len() {
                let child = &mut children[i];
                let mut docs = child.get_vec().as_ref().borrow_mut();
                match docs.binary_search_by(|e| e.cmp(&doc_id)) {
                            Ok(pos) => {
                                docs.remove(pos);
                                if docs.len() == 0 {
                                    empty_values.push(i);
                                }
                            },
                            Err(pos) => ()
                        };
            }
        }
        let mut children = self.get_vec().as_ref().borrow_mut();
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
        let field_index = FieldIndex {
            name: "myfield".to_string(),
            index: Rc::new(RefCell::new(vec![]))
        };

        field_index.put_content(99 as i64, 199);
        field_index.put_content(98 as i64, 198);
        field_index.put_content(100 as i64, 200);
        field_index.put_content(100 as i64, 201);
        field_index.put_content(99 as i64, 300);

        let d_99 = field_index.get_ids(99);
        assert_eq!(d_99, Some(vec![199,300]));

        let field_index_float = FieldIndex {
            name: "myfield_float".to_string(),
            index: Rc::new(RefCell::new(vec![]))
        };

        field_index_float.put_content(FloatWrapper{value: 88.9}, 188);
        field_index_float.put_content(FloatWrapper{value: 39.9}, 139);
        field_index_float.put_content(FloatWrapper{value: 88.9}, 288);
        field_index_float.put_content(FloatWrapper{value: 99.9}, 199);

        assert_eq!(field_index_float.get_ids(FloatWrapper{value: 88.9}), Some(vec![188,288]));

        let field_index_bool = FieldIndex {
            name: "myfield_bool".to_string(),
            index: Rc::new(RefCell::new(vec![]))
        };

        field_index_bool.put_content(true, 1);
        field_index_bool.put_content(false, 2);
        field_index_bool.put_content(true, 3);
        field_index_bool.put_content(false, 4);

        assert_eq!(field_index_bool.get_ids(true), Some(vec![1,3]));

        field_index.delete_doc(201);
        assert_eq!(field_index.get_ids(100), Some(vec![200]));
        field_index.delete_doc(200);
        assert_eq!(field_index.get_ids(100), None);
    }
}