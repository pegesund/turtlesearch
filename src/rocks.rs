use rocksdb::{DB, Options};
use crate::structures::{DocumentWordIndex, DocumentIndex};
use byte_array::ByteArray;
use byte_array::BinaryBuilder;
use std::cell::{RefCell, RefMut};
use std::vec::Vec;
use std::mem::transmute;
use std::fmt::Debug;


macro_rules! u64_to_barray {
    ($input:expr) => {
       unsafe { transmute($input.to_be()) };
    }
}

fn save_document_word_index(db: &DB, document_word_index: &DocumentWordIndex) {
    let ba = &mut ByteArray::new();
    let raw = document_word_index.to_raw(ba);
    let id_raw: [u8; 8] = u64_to_barray!(document_word_index.id);
    db.put(id_raw, ba.as_vec()).unwrap();
}

fn load_document_word_index(db: &DB, id: u64) -> DocumentWordIndex {
    let id_raw: [u8; 8] = u64_to_barray!(id);
    let res = db.get(id_raw).unwrap().unwrap();
    let ba = &mut ByteArray::new();
    for r in res {
        ba.write(&r);
    }
    let dwi = DocumentWordIndex::from_raw(ba).unwrap();
    return dwi;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;
    use crate::structures::HasChildrenNew;

    #[test]
    fn test_save_document_word_index() {
        let path = "/var/tmp/doc_index.rock";
        let dwi = DocumentWordIndex {
            id: 199,
            position: Rc::new(RefCell::new(vec![])),
            freq: 127
        };
        dwi.insert(88);
        dwi.insert(89);

        let db= DB::open_default(path);
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

        // DB::destroy(&Options::default(), path).unwrap();
        println!("document index word saved to db");
    }
}