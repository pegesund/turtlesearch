use rocksdb::{DB, Options};
use crate::structures::{DocumentWordIndex};
use byte_array::ByteArray;
use byte_array::BinaryBuilder;
use std::cell::{RefCell, RefMut};
use std::vec::Vec;
use std::mem::transmute;
use std::fmt::Debug;
use std::fs::read_to_string;
use crate::structures::*;
use crate::rocks::*;


#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;
    use crate::structures::SortedVector;
    use std::ptr;

    #[test]
    fn test_save_document_word_index() {
        let path = "/tmp/document_index.rock";

        let dwi = DocumentWordIndex {
            doc_id: 199,
            position: Rc::new(RefCell::new(vec![]))
        };
        dwi.insert(88);
        dwi.insert(89);
        {
        let db = DB::open_default(path);
        let db = match db {
            Err(err) => { println!("Go error while opening: {:?}", err); panic!("db trouble") }
            Ok(db) => db
        };

        save_document_word_index(&db, &dwi);
        let dwi2 = load_document_word_index(&db, dwi.doc_id);
        assert_eq!(dwi.doc_id, dwi2.doc_id);
        assert_eq!(dwi.position, dwi.position);
        assert_eq!(dwi.position.borrow().len(), 2);

    }
    DB::destroy(&Options::default(), path).unwrap();
    println!("document index word saved to db");
    }

    #[test]
    fn test_save_and_load_word_sorted() {
        let dwi1 = DocumentWordIndex {
            doc_id: 1,
            position: Rc::new(RefCell::new(vec![]))
        };
        let dwi2 = DocumentWordIndex {
            doc_id: 2,
            position: Rc::new(RefCell::new(vec![]))
        };
        let dwi3 = DocumentWordIndex {
            doc_id: 3,
            position: Rc::new(RefCell::new(vec![]))
        };

        let dwi4 = DocumentWordIndex {
            doc_id: 4,
            position: Rc::new(RefCell::new(vec![]))
        };

        let word_sorted1 = WordSorted {
            value:"hupp".to_string(),
            freq: 100,
            docs: Rc::new(RefCell::new(vec![])),
            optimized: false
        };

        let word_sorted2 = WordSorted {
            value:  "hypp".to_string(),
            freq: 100,
            docs: Rc::new(RefCell::new(vec![])),
            optimized: false
        };

        let path = "/tmp/words_sorted.rock";
        {
            let db = DB::open_default(path).unwrap();
            save_dwi_to_words_sorted(&db, &dwi1, &word_sorted1);
            save_dwi_to_words_sorted(&db, &dwi2, &word_sorted1);
            save_dwi_to_words_sorted(&db, &dwi3, &word_sorted1);
            save_dwi_to_words_sorted(&db, &dwi4, &word_sorted2);
            let dwi_ids = load_word_sorted(&db, &"hupp");
            assert_eq!(dwi_ids, vec![1,2,3]);
            let dwi_ids2 = load_word_sorted(&db, &"hypp");
            assert_eq!(dwi_ids2, vec![4]);
        }
        DB::destroy(&Options::default(), path).unwrap();
    }

    #[test]
    fn test_new_comparator() {
        let path = "/tmp/new_comparator.rock";
        {
            let mut opts = Options::default();
            opts.create_if_missing(true);
            DB::open(&opts, path).unwrap();
        }
        DB::destroy(&Options::default(), path).unwrap();
    }
}