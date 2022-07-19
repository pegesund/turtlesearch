use rocksdb::{DB, Options, IteratorMode, Direction};
use crate::structures::{DocumentWordIndex, WordSorted, SortedVector, FieldIndex};
use std::rc::Rc;
use std::cell::RefCell;
use crate::rocks::*;
use std::str;


pub fn build_field_index_docs<'a>(db_words: &'a DB, db_words_docs: &'a DB, db_docs: &'a DB, field_name: String) -> FieldIndex<WordSorted> {
    let field_index = FieldIndex {
        name: field_name.clone(),
        index: Rc::new(RefCell::new(vec![]))
    };

    let iter = db_words.iterator(IteratorMode::Start);
    for (key, value) in iter {
        let word = str::from_utf8(&key).unwrap().to_string();
        let ws: WordSorted = build_word_sorted(db_words, db_docs, word);
        field_index.insert(ws);
    }
    return field_index;
}

#[cfg(test)]
mod tests {
    use rocksdb::{DB, Options};
    use crate::rocks::save_word;

    fn test_build_field_index_docs() {
        let path_db_words  = "/tmp/bfi_db_words";
        let path_db_words_docs  = "/tmp/bfi_db_words_docs";
        let path_db_docs  = "/tmp/bfi_db_docs";
        {
            let db_words = DB::open_default(path_db_words).unwrap();
            let db_words_docs = DB::open_default(path_db_words_docs).unwrap();
            let db_docs = DB::open_default(path_db_docs).unwrap();
            save_word(&db_words, &"one");
            save_word(&db_words, &"two");
            save_word(&db_words, &"three");
        }
        DB::destroy(&Options::default(), path_db_words).unwrap();
        DB::destroy(&Options::default(), path_db_words_docs).unwrap();
        DB::destroy(&Options::default(), path_db_docs).unwrap();
    }
}