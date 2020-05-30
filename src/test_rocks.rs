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
    use crate::structures::HasChildrenNew;

    #[test]
    fn test_save_document_word_index() {
        let path = "/tmp/document_index.rock";

        let dwi = DocumentWordIndex {
            id: 199,
            position: Rc::new(RefCell::new(vec![])),
            freq: 127
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
        let dwi2 = load_document_word_index(&db, dwi.id);
        assert_eq!(dwi.id, dwi2.id);
        assert_eq!(dwi.freq, dwi.freq);
        assert_eq!(dwi.position, dwi.position);
        assert_eq!(dwi.position.borrow().len(), 2);

    }
    DB::destroy(&Options::default(), path).unwrap();
    println!("document index word saved to db");
    }
}