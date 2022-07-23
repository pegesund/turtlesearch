use std::vec::Vec;
use rocksdb::{DB, Options};
use byte_array::ByteArray;
use byte_array::BinaryBuilder;
// use crate::structures::*;
use crate::sorted_vector::*;
use crate::structures::DocumentWordAndPositions;
use std::collections::HashMap;
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


impl BinaryBuilder for DocumentWordAndPositions {
    fn new() ->
             DocumentWordAndPositions {
            let res = DocumentWordAndPositions {
                doc_id: 0,
                position:vec![]
            };
        return res
    }
 
    fn from_raw(ba: &mut ByteArray) -> Option<Self> {
        let id: u64 = ba.read();
        let num: u64 = ba.read();
        let mut vec: Vec<u32> = vec![];
        for i in 0..num {
            let v = ba.read();
            vec.push(v)
        }
        return Some(DocumentWordAndPositions {
            doc_id: id,
            position: vec![]
        });
    }
    fn to_raw(&self, mut ba: &mut ByteArray) {
        ba <<= &self.doc_id;
        ba <<= &self.position.len();
        let len = self.position.len();
        for i in 0..len { ba <<= &self.position[i] }
    }
 }

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;

    #[test]
    fn serializing_word_index() {
        let mut wi = DocumentWordAndPositions {
            doc_id: 199,
            position: vec![]
        };

        wi.insert(22);
        wi.insert(23);
        wi.insert(21);
        wi.insert(18);
        wi.insert(33);
        let ba = &mut ByteArray::new();
        let raw = wi.to_raw(ba);
        let wi2 = DocumentWordAndPositions::from_raw(ba).unwrap();
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