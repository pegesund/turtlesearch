#[allow(dead_code)]
use std::fmt::Debug;
use std::cmp::Ordering;
use duplicate::duplicate;


use float_cmp::ApproxEq;
use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::rc::Rc;
use std::borrow::{BorrowMut, Borrow, Cow};

use crate::comparator::FieldType;
use crate::sorted_vector::SortedVector;

/*

This file contains in-memory structures
 
FieldIndex contains one of the SortedVectors, for example 
    IntegerSorted 
    ...
    WordSorted
        DodumentWordAndPositions (word id and positions in the docs)

*/



///
/// DocumentWordIndex
///

    #[allow(dead_code)]
    #[derive(Debug)]
    #[derive(Clone)]
    #[derive(Eq)]
    pub struct DocumentWordAndPositions {
        pub doc_id: u64,
        pub position: Rc<RefCell<Vec<u32>>>,
    } 

    impl PartialEq for DocumentWordAndPositions {
        fn eq(&self, other: &Self) -> bool {
            self.doc_id == other.doc_id
        }
    }


impl <'a> PartialOrd for DocumentWordAndPositions {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl <'a> Ord for DocumentWordAndPositions {
    fn cmp(&self, other: &Self) -> Ordering {
        self.doc_id.cmp(&other.doc_id)
    }
}

///
/// FieldIndex
///

#[allow(dead_code)]
#[derive(Debug)]
#[derive(Clone)]
pub struct FieldIndex<G:Debug + Clone + Ord > {
    pub name: String,
    pub index:  Rc<RefCell<Vec<G>>>
}

impl<G: Debug + Clone + Ord > SortedVector<G> for FieldIndex<G> {
    fn get_vec(&self) -> &Rc<RefCell<Vec<G>>> {
        return &self.index;
    }
}


///
/// Field
/// 
#[allow(dead_code)]
#[derive(Debug)]
#[derive(Clone)]

pub struct Field <G:Debug + Clone + Ord > {
    pub name: String,
    pub field_type: FieldType,
    pub index:  Option<FieldIndex<G>>,
    pub size:  Rc<RefCell<u64>>
}


///
/// Collections
/// 

#[allow(dead_code)]
#[derive(Debug)]
#[derive(Clone)]
pub struct Collection <G:Debug + Clone + Ord > {
    pub name: String,
    pub fields:  Rc<RefCell<Vec<Field<G>>>>
}