use rocksdb::{DB, Options, IteratorMode, Direction};
use crate::sorted_vector::{DocumentWordIndex, WordSorted, SortedVector, FieldIndex};
use byte_array::ByteArray;
use byte_array::BinaryBuilder;
use std::cell::{RefCell, RefMut};
use std::vec::Vec;
use std::mem::transmute;
use std::fmt::Debug;
use std::fs::read_to_string;
use im::Vector;
use std::convert::TryInto;
use std::rc::Rc;
use std::borrow::{BorrowMut, Borrow, Cow}; 


macro_rules! u64_to_barray {
    ($input:expr) => {
       unsafe { transmute($input.to_be()) }
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
    let id_raw: [u8; 8] = u64_to_barray!(document_word_index.doc_id);
    db.put(id_raw, ba.as_vec()).unwrap();
}

pub fn load_document_word_index(db: &DB, id: u64) -> DocumentWordIndex {
    let id_raw: [u8; 8] = u64_to_barray!(id);
    let res = db.get(id_raw).unwrap().unwrap();
    let mut ba = vec_to_bytearray(res);
    let dwi = DocumentWordIndex::from_raw(&mut ba).unwrap();
    return dwi;
}

pub fn delete_document_word_index(db: &DB, id: u64) {
    let id_raw: [u8; 8] = u64_to_barray!(id);
    db.delete(id_raw).unwrap();
}

fn dwi_and_ws_to_key(dwi: &DocumentWordIndex, ws: &WordSorted) -> ByteArray {
    let dwi_id_raw: [u8; 8] = u64_to_barray!(dwi.doc_id);
    let w = ws.value.clone();
    let word_as_bytes: &[u8] = w.as_bytes();
    let mut key = ByteArray::new();
    for b in word_as_bytes {
        key.write(b);
    }
    for b in &dwi_id_raw {
        key.write(b)
    }
    return key;
}

/// saves connection between a word and the dwi
/// key is word + dwi.id
/// value is just 1
pub fn save_dwi_to_words_sorted(db: &DB, dwi: &DocumentWordIndex, ws: &WordSorted) {
    let mut key = dwi_and_ws_to_key(dwi, ws);
    let val: [u8; 8] = u64_to_barray!(dwi.doc_id);
    db.put(key.as_vec(), val).unwrap();
}

/// delete connectino between a word and the dwi
pub fn delete_dwi_to_words_sorted(db: &DB, dwi: &DocumentWordIndex, ws: &WordSorted) {
    let mut key = dwi_and_ws_to_key(dwi, ws);
    db.delete(key.as_vec()).unwrap();
}

pub fn save_word(db: &DB, word: &str) {
    db.put(word.as_bytes(), "".as_bytes()).unwrap();
}

/// returns a list of all docs which are connected to a word in an index
pub fn load_word_sorted(db: &DB, word: &str) -> Vec<u64> {
    let mut res: Vec<u64> = Vec::new();
    let lookup_key = word.as_bytes();
    let iter = db.iterator(IteratorMode::From(word.as_bytes(), Direction::Forward));

    let mut do_break = false;
    for (key, value) in iter {
        for i in 0..lookup_key.len() {
            if lookup_key[i] != key[i] {
                do_break = true;
                break
            }
        }
        if do_break {
            break;
        }
        let dwi_id: u64 = unsafe { std::mem::transmute::<[u8; 8], u64>((*value).try_into().unwrap()) }.to_be();
        res.push(dwi_id);
    }
    return res;
}


/// build new WordSorted based on word
pub fn build_word_sorted<'a>(db_words: &'a DB, db_docs: &'a DB, word: String) -> WordSorted {
    let ws = WordSorted {
        value: word.clone(),
        freq: 0,
        docs: Rc::new(RefCell::new(vec![])),
        optimized: false
    };
    let doc_ids = load_word_sorted(db_words, &word.to_owned());
    for i in 0..doc_ids.len() {
        let doc = load_document_word_index(db_docs, doc_ids[i]);
        ws.insert(doc);
    }
    return ws;
}


