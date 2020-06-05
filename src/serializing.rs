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
                id: 0,
                position: Rc::new(RefCell::new(vec![])),
                freq: 0,
                doc:  ptr::null_mut()
            };
        return res
    }
 
    fn from_raw(ba: &mut ByteArray) -> Option<Self> {
        let id: u64 = ba.read();
        let num: u64 = ba.read();
        let vec: Rc<RefCell<Vec<u64>>> = Rc::new(RefCell::new(vec![]));
        for i in 0..num { vec.borrow_mut().push(ba.read()) }
        let freq: u64 = ba.read();
        return Some(DocumentWordIndex {
            id: id,
            position: Rc::new(RefCell::new(vec![])),
            freq: freq,
            doc: ptr::null_mut()
        });
    }
    fn to_raw(&self, mut ba: &mut ByteArray) {
        ba <<= &self.id;
        ba <<= &self.position.as_ref().borrow().len();
        for i in 0..self.position.as_ref().borrow().len() { ba <<= &self.position.as_ref().borrow()[i] }
        ba <<= &self.freq;
    }
 }

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;

    #[test]
    fn serializing_word_index() {
        let wi = DocumentWordIndex {
            id: 199,
            position: Rc::new(RefCell::new(vec![])),
            freq: 0,
            doc: ptr::null_mut()
        };

        wi.insert(22);
        wi.insert(23);
        wi.insert(21);
        wi.insert(18);
        wi.insert(33);
        let ba = &mut ByteArray::new();
        println!("wi: {:?}", wi);
        let raw = wi.to_raw(ba);
        let wi2 = DocumentWordIndex::from_raw(ba).unwrap();
        println!("Here is wi2: {:?}", wi2);
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