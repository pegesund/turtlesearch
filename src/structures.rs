#[allow(dead_code)]
use std::fmt::Debug;
use std::cmp::Ordering;
use byte_array::BinaryBuilder;
use duplicate::duplicate;
use enum_dispatch::enum_dispatch;
use float_cmp::ApproxEq;
use num::FromPrimitive;
use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::rc::Rc;
use std::borrow::{BorrowMut, Borrow, Cow};
use num_derive::FromPrimitive;    
use crate::sorted_vector::{SortedVector, FloatWrapper};
use std::string::String;
use crate::structures::FieldValue::I64;


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

#[derive(FromPrimitive, Clone, Debug)]
pub enum FieldType {
    I64,
    U64,
    Isize,
    I8,
    I16,
    I32,
    Usize,
    U8,
    U16,
    U32,
    F32,
    F64,
    String
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum FieldValue {
    I64 { value: i64 },
    U64 { value: u64 },
    Isize { value: isize },
    I8 { value: i8 },
    I16 { value: i16 },
    I32 { value: i32 },
    Usize { value: usize },
    U8 { value: u8 },
    U16 { value: u16 },
    U32 { value: u32 },
    F32 { value: f32 },
    F64 { value: f64 },
    String { value: String }
}

#[allow(dead_code)]
#[derive(Debug)]
#[derive(Clone)]
pub enum FieldEnumStructs {
    I64(Field::<i64>),
    U64(Field::<u64>),
    Isize(Field::<isize>),
    I8(Field::<i8>),
    I16(Field::<i16>),
    I32(Field::<i32>),
    Usize(Field::<usize>),
    U8(Field::<u8>),
    U16(Field::<u16>),
    U32(Field::<u32>),
    F64(Field::<FloatWrapper>),
    String(Field::<String>)
}



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
    pub size: u64
}


///
/// Collections
/// 

#[allow(dead_code)]
#[derive(Debug)]
#[derive(Clone)]
pub struct Collection {
    pub name: String,
    pub fields:  Rc<RefCell<Vec<FieldEnumStructs>>>
}

#[allow(dead_code)]
#[derive(Debug)]
#[derive(Clone)]  
pub struct Document {
    pub id: u64,
    pub external_id: FieldValue,
    pub values: Rc<RefCell<Vec<FieldValue>>>
}


