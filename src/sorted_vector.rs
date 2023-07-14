#![allow(dead_code)]
use std::fmt::Debug;
use std::cmp::Ordering;
use duplicate::{duplicate, duplicate_item};

use std::sync::{RwLock};
use float_cmp::ApproxEq;
use std::collections::HashMap;


use crate::structures::DocumentWordAndPositions;



#[allow(dead_code)]
#[derive(Debug)]
#[derive(Clone)]
 #[derive(Copy)]
pub struct FloatWrapper {
    pub value: f64
}

#[duplicate_item(
    the_class sort_field;
    [ IntegerSorted ] [ value ];
    [ DateSorted ] [ value ];
    [ FloatSorted ] [ value ];
    [ BoolSorted ] [ value ];


)]
impl PartialEq for the_class {
    fn eq(&self, other: &Self) -> bool {
        self.sort_field == other.sort_field
    }
}

#[duplicate_item(
the_class sort_field;
[ WordSorted ] [ value ];
)]

impl <'a> PartialEq for the_class {
    fn eq(&self, other: &Self) -> bool {
        self.sort_field == other.sort_field
    }
}

impl PartialEq for FloatWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.value.approx_eq(other.value, (0.0, 2))
    }
}

impl Eq for FloatWrapper {

}

#[duplicate_item(
    the_class;
    [ WordSorted ];
    [ FloatSorted ];
    [ IntegerSorted ];
    [ DateSorted ];
    [ BoolSorted ];
)]

impl <'a> PartialOrd for the_class {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd for FloatWrapper {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { 
        let c = self.value.partial_cmp(&other.value);
        return c;
    }
}

#[duplicate_item(
    the_class sort_field;
    [ WordSorted ] [ value ];
    [ IntegerSorted ] [ value ];
    [ DateSorted  ] [ value ];
    [ FloatSorted  ] [ value ];
    [ BoolSorted  ] [ value ];
)]

impl <'a> Ord for the_class {
    fn cmp(&self, other: &Self) -> Ordering {
        self.sort_field.cmp(&other.sort_field)
    }
}

impl Ord for FloatWrapper {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.partial_cmp(&other.value).unwrap()
    }
}


#[allow(dead_code)]
#[derive(Debug)]
#[derive(Eq)]
#[derive(Clone)]
pub struct WordSorted {
    pub value: String,
    pub freq: u64,
    pub docs: Vec<DocumentWordAndPositions>,
    pub optimized: bool
}

#[allow(dead_code)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Eq)]
pub struct IntegerSorted {
    pub value: i64,
    pub doc_ids: Vec<u64>
}


#[allow(dead_code)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Eq)]
pub struct FloatSorted {
    pub value: FloatWrapper,
    pub doc_ids: Vec<u64>
}

#[allow(dead_code)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Eq)]
pub struct DateSorted {
    pub value: u64,
    pub doc_ids: Vec<u64>
}

#[allow(dead_code)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(Eq)]
pub struct BoolSorted {
    pub value: bool,
    pub doc_ids: Vec<u64>
}

impl SortedVector<u32> for DocumentWordAndPositions {
    fn get_vec(&mut self) -> &mut Vec<u32> {
        &mut self.position
    }
}

impl SortedVector<DocumentWordAndPositions> for WordSorted {
    fn get_vec(&mut self) -> &mut Vec<DocumentWordAndPositions> {
        &mut self.docs
    }
}

#[duplicate_item(
the_class val_type;
    [ IntegerSorted ] [ u64 ];
    [ DateSorted ] [ u64 ];
    [ BoolSorted ] [ u64 ];
    [ FloatSorted ] [ u64 ];
)]

impl SortedVector<val_type> for the_class {
    fn get_vec(&mut self) -> &mut Vec<val_type> {
        &mut self.doc_ids
    }
}

pub trait SortedVector<E: Debug + Clone + Ord> {
    fn get_vec(&mut self) -> &mut Vec<E>;

    fn insert(&mut self, element: E) {
        let insert_pos = match self.get_vec().binary_search(&element) {
            Ok(pos) => pos,
            Err(pos) => pos
        };
        
        let _ = &self.get_vec().insert(insert_pos, element);        
        }

    fn delete(&mut self, element: &E) {
        let delete_pos = match self.get_vec().binary_search(element) {
            Ok(pos) => Some(pos),
            Err(pos) => None
        };
        if let Some(pos) = delete_pos {
            self.get_vec().remove(pos);
        }   
    }
}

