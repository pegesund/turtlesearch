use rocksdb::{DB, Options};
use crate::structures::{DocumentWordIndex, DocumentIndex};
use byte_array::ByteArray;
use byte_array::BinaryBuilder;
use std::cell::{RefCell, RefMut};
use std::vec::Vec;
use std::mem::transmute;
use std::fmt::Debug;
use std::fs::read_to_string;


macro_rules! u64_to_barray {
    ($input:expr) => {
       unsafe { transmute($input.to_be()) };
    }
}

fn vec_to_bytearray(res: Vec<u8>) -> ByteArray {
    let mut ba = ByteArray::new();
    for r in res {
        ba.write(&r);
    }
    return ba;
}

pub fn save_document_word_index(db: &DB, document_word_index: &DocumentWordIndex) {
    let ba = &mut ByteArray::new();
    let raw = document_word_index.to_raw(ba);
    let id_raw: [u8; 8] = u64_to_barray!(document_word_index.id);
    db.put(id_raw, ba.as_vec()).unwrap();
}

pub fn load_document_word_index(db: &DB, id: u64) -> DocumentWordIndex {
    let id_raw: [u8; 8] = u64_to_barray!(id);
    let res = db.get(id_raw).unwrap().unwrap();
    let mut ba = vec_to_bytearray(res);
    let dwi = DocumentWordIndex::from_raw(&mut ba).unwrap();
    return dwi;
}

