use rocksdb::{DB, Options, IteratorMode, Direction};
use crate::structures::{DocumentWordIndex, WordSorted, HasChildrenNew, FieldIndex};
use std::rc::Rc;
use std::cell::RefCell;
use crate::rocks::*;
use std::str;


pub fn build_field_index_docs<'a>(db_words: &'a DB, db_words_docs: &'a DB, db_docs: &'a DB) -> FieldIndex<WordSorted> {
    let field_index = FieldIndex {
        name: "".to_string(),
        index: Rc::new(RefCell::new(vec![]))
    };

    let iter = db_words.iterator(IteratorMode::Start);
    for (key, value) in iter {
        let word = str::from_utf8(&*key).unwrap().to_string();
        let ws: WordSorted = build_word_sorted(db_words, db_docs, word);
        field_index.insert(ws);
    }

    return field_index;
}
