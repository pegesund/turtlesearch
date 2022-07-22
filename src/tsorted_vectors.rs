#[allow(dead_code)]
use std::fmt::Debug;
use std::cmp::Ordering;
use duplicate::duplicate;

use std::sync::{RwLock};
use float_cmp::ApproxEq;
use std::collections::HashMap;


use crate::structures::DocumentWordAndPositions;


#[allow(dead_code)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Eq)]
pub struct IntegerSorted {
    pub value: i64,
    pub doc_ids: Vec<u64>
}

#[duplicate(
    the_class sort_field;
    [ IntegerSorted ] [ value ];
)]
impl PartialEq for the_class {
    fn eq(&self, other: &Self) -> bool {
        self.sort_field == other.sort_field
    }
}


#[duplicate(
    the_class;
    [ IntegerSorted ];
)]

impl <'a> PartialOrd for the_class {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[duplicate(
    the_class sort_field;
    [ IntegerSorted ] [ value ];
)]


impl <'a> Ord for the_class {
    fn cmp(&self, other: &Self) -> Ordering {
        self.sort_field.cmp(&other.sort_field)
    }
}

pub trait SortedVector<E: Debug + Clone + Ord> {
    fn get_vec(&mut self) -> &mut Vec<E>;

    fn insert(&mut self, element: E) -> () {
        let insert_pos = match self.get_vec().binary_search(&element) {
            Ok(pos) => pos,
            Err(pos) => pos
        };
        
        let _ = &self.get_vec().insert(insert_pos, element);        
        }

    fn delete(&mut self, element: &E) {
        let delete_pos = match self.get_vec().binary_search(&element) {
            Ok(pos) => Some(pos),
            Err(pos) => None
        };
        match delete_pos {
            Some(pos) =>  { self.get_vec().remove(pos); () },
            _ => ()
        };
    }
}


impl <'a> SortedVector<u64> for IntegerSorted {
    fn get_vec(&mut self) -> &mut Vec<u64> {
        return &mut self.doc_ids;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let mut is1 = IntegerSorted {
            value: 0,
            doc_ids: vec![]
        };
        is1.insert(10);
        is1.insert(20);
        is1.insert(15);
        println!("Vector: {:?}", is1);
        assert_eq!(is1.doc_ids, vec![10, 15, 20] );
    }

}