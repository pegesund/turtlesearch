use std::vec::Vec;
use rocksdb::{DB, Options};
use byte_array::ByteArray;
use byte_array::BinaryBuilder;
use crate::structures::*;



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


impl BinaryBuilder for WordIndex {
    fn new() ->  
        WordIndex {
            let res = WordIndex {
            id: 0,
            position: Vec::new(),
            freq: 0
            };
        return res
    }
 
    fn from_raw(ba: &mut ByteArray) -> Option<Self> {
        let id = ba.read();
        let num: u64 = ba.read();
        let mut vec = Vec::new();
        for i in 0..num { vec.push(ba.read()) }
        let freq = ba.read();
        return Some(WordIndex {
            id,
            position: vec,
            freq
        });
    }
    fn to_raw(&self, mut ba: &mut ByteArray) {
        ba <<= &self.id;
        ba <<= &self.position.len();
        for i in 0..self.position.len() { ba <<= &self.position[i] }
        ba <<= &self.freq;
    }
 }





impl BinaryBuilder for DocumentIndex {
    fn new() ->
             DocumentIndex {
        let res = DocumentIndex {
            id: 0,
            words: Vec::new()
        };
        return res;
    }

    fn from_raw(ba: &mut ByteArray) -> Option<Self> {
        let id = ba.read();
        let num: u64 = ba.read();
        let mut words = Vec::new();
        for i in 0..num { words.push(ba.read()) }
        return Some(DocumentIndex {
            id,
            words
        });
    }
    fn to_raw(&self, mut ba: &mut ByteArray) {
        ba <<= &self.id;
        ba <<= &self.words.len();
        for i in 0..self.words.len() { ba <<= &self.words[i] }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serializing_word_index() {
        let mut wi = WordIndex {
            id: 199,
            position: Vec::new(),
            freq: 0
        };

        wi.insert(22);
        wi.insert(23);
        wi.insert(21);
        wi.insert(18);
        wi.insert(33);
        let mut found = wi.get_child_by_id(21);
        match found.as_mut() {
            Some(v) => *v = &42,
            None => {}
        }
        let ba = &mut ByteArray::new();
        println!("wi: {:?} {:?}", wi, found);
        let raw = wi.to_raw(ba);
        let wi2 = WordIndex::from_raw(ba).unwrap();
        println!("Here is wi2: {:?}", wi2);
        assert_eq!(wi, wi2);
    }

    #[test]
    fn serializing_doc_index() {
        let mut di = DocumentIndex {
            id: 99,
            words: Vec::new()
        };

        di.insert(WordIndex {
            id: 199,
            position: Vec::new(),
            freq: 0
        });

        di.insert(WordIndex {
            id: 10,
            position: Vec::new(),
            freq: 0
        });

        di.insert(WordIndex {
            id: 200,
            position: Vec::new(),
            freq: 0
        });


        let ba = &mut ByteArray::new();
        let raw = di.to_raw(ba);
        let di2 = DocumentIndex::from_raw(ba).unwrap();
        println!("Here is wi2: {:?}", di2);
        // check cloning
        assert_eq!(di, di2);
        // check sort order
        let word_ids: Vec<u64> = di2.words.iter().map(|i| i.id).collect();
        assert_eq!(word_ids, vec![10,199,200]);
    }

}