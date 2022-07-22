#[allow(dead_code)]
use std::fmt::Debug;
use std::cmp::Ordering;
use byte_array::BinaryBuilder;
use duplicate::duplicate;


use float_cmp::ApproxEq;
use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::rc::Rc;
use std::borrow::{BorrowMut, Borrow, Cow};

use crate::comparator::{FieldType, FieldValue};
use crate::sorted_vector::SortedVector;
use crate::comparator::FieldValue::*;
use std::string::String;
/*

This file contains in-memory structures

Collection has many
    Field maybe has one
        FieldIndex contains one of the SortedVectors, for example 
            IntegerSorted 
            ...
            WordSorted (freq holds number of positions in total for this word)
                DodumentWordIdAndPositions

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

#[allow(dead_code)]
#[derive(Debug)]
#[derive(Clone)]  
pub struct Document {
    pub id: u64,
    pub external_id: FieldValue,
    pub values: Rc<RefCell<Vec<FieldValue>>>
}

impl BinaryBuilder for Document {
    
    fn new() -> Self {
        let res = Document {
            id: 0,
            external_id: I64 {value: 0},
            values: Rc::new(RefCell::new(vec![]))
        };
        return res
    }
     

    fn from_raw(ba: &mut byte_array::ByteArray) -> Option<Self> {
        todo!()
    }

    fn to_raw(&self, ba: &mut byte_array::ByteArray) {

        let values = &self.values.as_ref().borrow();
        for i in 0..values.len()  {
            // ba <<= &values[i]
        }
    }
}
