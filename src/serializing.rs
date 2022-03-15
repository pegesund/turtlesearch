use std::vec::Vec;
use rocksdb::{DB, Options};
use byte_array::ByteArray;
use byte_array::BinaryBuilder;
use crate::structures::*;

use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::rc::Rc;
use std::borrow::Borrow;
use std::ptr;


/*
fn convert(from: Vec<u64>) -> Vec<u8> {
    from.
    return res;
}
*/

fn test_db() {
    let mut ba = ByteArray::new();
    ba.write(&8u8);
    let v = ba.as_vec();
    let path = "_path_for_rocksdb_storage";
{
   let db = DB::open_default(path).unwrap();
   db.put(b"my key", v).unwrap();
   match db.get(b"my key") {
       Ok(Some(value)) => println!("retrieved value {}", String::from_utf8(value).unwrap()),
       Ok(None) => println!("value not found"),
       Err(e) => println!("operational problem encountered: {}", e),
   }
   db.delete(b"my key").unwrap();
}
let _ = DB::destroy(&Options::default(), path);
}


impl BinaryBuilder for DocumentWordIndex {
    fn new() ->
             DocumentWordIndex {
            let res = DocumentWordIndex {
                doc_id: 0,
                position: Rc::new(RefCell::new(vec![]))
            };
        return res
    }
 
    fn from_raw(ba: &mut ByteArray) -> Option<Self> {
        let id: u64 = ba.read();
        let num: u64 = ba.read();
        let vec: Rc<RefCell<Vec<u32>>> = Rc::new(RefCell::new(vec![]));
        for i in 0..num {
            let v = ba.read();
            vec.borrow_mut().push(v)
        }
        return Some(DocumentWordIndex {
            doc_id: id,
            position: Rc::new(RefCell::new(vec![]))
        });
    }
    fn to_raw(&self, mut ba: &mut ByteArray) {
        ba <<= &self.doc_id;
        ba <<= &self.position.as_ref().borrow().len();
        let len = self.position.as_ref().borrow().len();
        for i in 0..len { ba <<= &self.position.as_ref().borrow()[i] }
    }
 }

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;

    #[test]
    fn serializing_word_index() {
        let wi = DocumentWordIndex {
            doc_id: 199,
            position: Rc::new(RefCell::new(vec![]))
        };

        wi.insert(22);
        wi.insert(23);
        wi.insert(21);
        wi.insert(18);
        wi.insert(33);
        let ba = &mut ByteArray::new();
        let raw = wi.to_raw(ba);
        let wi2 = DocumentWordIndex::from_raw(ba).unwrap();
        assert_eq!(wi, wi2);
    }


}

/*

#[test]
fn string_parse() {
    let str = "abc☯";
    let v_vec: Vec<u16> = str.chars().map(|c| c as u16).collect();
    println!("Value: {:?}", v_vec);

}


*/