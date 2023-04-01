use rocksdb::{DB, Options, IteratorMode, Direction};
use crate::sorted_vector::*;
use crate::structures::Collection;
use crate::structures::Document;
use crate::structures::DocumentWordAndPositions;
use crate::structures::Field;
use crate::structures::FieldType;
use crate::structures::FieldValue;
use crate::structures::GetFieldInfo;
use byte_array::ByteArray;
use byte_array::BinaryBuilder;
use std::vec::Vec;
use std::mem::transmute;
use std::fmt::Debug;
use std::fs::read_to_string;
use im::Vector;
use std::convert::TryInto;


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

pub fn save_position_word_index(db: &DB, document_word_index: &DocumentWordAndPositions) {
    let ba = &mut ByteArray::new();
    let raw = document_word_index.to_raw(ba);
    let id_raw: [u8; 8] = u64_to_barray!(document_word_index.doc_id);
    db.put(id_raw, ba.as_vec()).unwrap();
}

pub fn load_position_word_index(db: &DB, id: u64) -> DocumentWordAndPositions {
    let id_raw: [u8; 8] = u64_to_barray!(id);
    let res = db.get(id_raw).unwrap().unwrap();
    let mut ba = vec_to_bytearray(res);
    let dwi = DocumentWordAndPositions::from_raw(&mut ba).unwrap();
    return dwi;
}

pub fn delete_document_position_index(db: &DB, id: u64) {
    let id_raw: [u8; 8] = u64_to_barray!(id);
    db.delete(id_raw).unwrap();
}

fn dwi_and_ws_to_key(dwi: &DocumentWordAndPositions, ws: &WordSorted) -> ByteArray {
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
pub fn save_dwi_to_words_sorted(db: &DB, dwi: &DocumentWordAndPositions, ws: &WordSorted) {
    let mut key = dwi_and_ws_to_key(dwi, ws);
    let val: [u8; 8] = u64_to_barray!(dwi.doc_id);
    db.put(key.as_vec(), val).unwrap();
}

/// delete connectino between a word and the dwi
pub fn delete_dwi_to_words_sorted(db: &DB, dwi: &DocumentWordAndPositions, ws: &WordSorted) {
    let mut key = dwi_and_ws_to_key(dwi, ws);
    db.delete(key.as_vec()).unwrap();
}

pub fn save_word_sorted(db: &DB, word: &str) {
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
    let mut ws = WordSorted {
        value: word.clone(),
        freq: 0,
        docs: vec![],
        optimized: false
    };
    let doc_ids = load_word_sorted(db_words, &word.to_owned());
    for i in 0..doc_ids.len() {
        let doc = load_position_word_index(db_docs, doc_ids[i]);
        ws.insert(doc);
    }
    return ws;
}



fn read_field_value_from_ba(ba: &mut ByteArray, field_type: FieldType) -> FieldValue {
    let val: FieldValue = match field_type {
        FieldType::I64 => FieldValue::I64 { value: ba.read::<i64>() },
        FieldType::U64 => FieldValue::U64 { value: ba.read::<u64>() },
        FieldType::Isize => FieldValue::Isize { value: ba.read::<isize>() },
        FieldType::I8 => FieldValue::I8 { value: ba.read::<i8>() },
        FieldType::I16 => FieldValue::I16 { value: ba.read::<i16>() },
        FieldType::I32 => FieldValue::I32 { value: ba.read::<i32>() },
        FieldType::Usize => FieldValue::Usize { value: ba.read::<usize>() },
        FieldType::U8 => FieldValue::U8 { value: ba.read::<u8>() },
        FieldType::U16 => FieldValue::U16 { value: ba.read::<u16>() },
        FieldType::U32 => FieldValue::U32 { value: ba.read::<u32>() },
        FieldType::F32 => FieldValue::F32 { value: ba.read::<f32>() },
        FieldType::F64 => FieldValue::F64 { value: ba.read::<f64>() },
        FieldType::String => FieldValue::String { value: ba.read::<String>() },
    };
    return val
}

fn write_field_value_to_ba (mut ba: &mut ByteArray, val: &FieldValue) {
    match val {
        FieldValue::I64 { value } => ba <<= value,
        FieldValue::U64 { value } => ba <<= value,
        FieldValue::Isize { value } => ba <<= value,
        FieldValue::I8 { value } => ba <<= value,
        FieldValue::I16 { value } => ba <<= value,
        FieldValue::I32 { value } => ba <<= value,
        FieldValue::Usize { value } => ba <<= value,
        FieldValue::U8 { value } => ba <<= value,
        FieldValue::U16 { value } => ba <<= value,
        FieldValue::U32 { value } => ba <<= value,
        FieldValue::F32 { value } => ba <<= value,
        FieldValue::F64 { value } => ba <<= value,
        FieldValue::String { value } => ba <<= value,
    }
}

fn write_doc_fields_to_ba (ba: &mut ByteArray, doc: &Document, collection: &Collection) {
    let fields = &collection.fields;
    for i in 0..fields.len() {
        let field = &fields[i];
        let field_value = &doc.values[i];
        write_field_value_to_ba(ba, field_value)
    }
}

fn read_doc_field_from_ba (ba: &mut ByteArray, collection: &Collection) -> Document {
    let mut field_values = vec![];
    let fields = &collection.fields;
    for i in 0..fields.len() {
        let field_enum = &fields[i];
        let field_type = field_enum.get_field_type();
        let field_value = read_field_value_from_ba(ba, field_type);
        field_values.push(field_value)

    }
    Document { id: 0, external_id: FieldValue::U64 { value: 1000 }, values: field_values }
}

#[cfg(test)]
mod tests {
    use std::{sync::{Arc, RwLock}, thread};

    use crate::structures::{DocumentWordAndPositions, FieldEnumStruct};

    use super::*;

    #[test]
    fn test_write_and_read_document_to_ba() {
        let mut fields =  vec![];
        let f1 = Field::<u64> {
            name: "Number1".to_string(),
            field_type: FieldType::U64,
            index: None,
            size: 0
        };

        fields.push(FieldEnumStruct::U64(f1));
        // fields.push(f1);
        let f2 = Field::<i32> {
            name: "Number2".to_string(),
            field_type: FieldType::I32,
            index: None,
            size: 0
        };
        // fields.push(f2);
        fields.push(FieldEnumStruct::I32(f2));
        let f3 = Field::<String> {
            name: "Number3".to_string(),
            field_type: FieldType::String,
            index: None,
            size: 0
        };

        fields.push(FieldEnumStruct::String(f3));

        let collection = Collection { name: "test-collection".to_string(), fields: fields };
        
        let mut field_values = vec![];

        field_values.push(FieldValue::U64 {value: 10});
        field_values.push(FieldValue::I32 {value: 20});
        field_values.push(FieldValue::String {value: "Hello!".to_string()});

        let doc = Document {
            id: 0,
            external_id: FieldValue::U64 { value: 1000 },
            values: field_values,
        };

        let mut ba = ByteArray::new();
        write_doc_fields_to_ba (&mut ba, &doc, &collection);

        assert_eq!(26, ba.len()); // 8 + 4 + 2*str-len + str-size

        let doc = read_doc_field_from_ba(&mut ba, &collection);
        println!("Doc: {:?}", doc.values[1]);

        assert_eq!(doc.values[0], FieldValue::U64 {value: 10});
        assert_eq!(doc.values[1], FieldValue::I32 {value: 20});
        assert_eq!(doc.values[2], FieldValue::String {value: "Hello!".to_string()});        

    }


    #[derive(Debug)]
    #[derive(Clone)]
    pub struct Bottom {
        pub values: Vec<u64>
    }

    #[derive(Debug)]
    #[derive(Clone)]
    pub struct Top {
        pub i: u64,
        pub value: Bottom
    }


    fn helper(top: Top){
        let rwlock = RwLock::new(top);
        let arc = Arc::new(rwlock);
        let local_arc = arc.clone();
        let mut threads = vec![];
        for _ in 0..10{
            let my_rwlock = arc.clone();
            let t = thread::spawn(move || {
                let mut writer = my_rwlock.write().unwrap();
                writer.i += 1;
                writer.value.values.push(88);
                writer.value.values.push(89);
                writer.value.values[0] = 32;
                println!("In thread..");
                // do some stuff
            });
            threads.push(t);
        }
        for child in threads {
            let _ = child.join();
        }

        let reader = local_arc.read().unwrap();
        println!("Done with threads: {:?}", reader);
    }

    #[test]
    fn test_arc() {
        let b = Bottom {
            values: vec![]
        };

        let t = Top { value: b, i: 88 };
        helper(t);

    }
    


        
    
    


}