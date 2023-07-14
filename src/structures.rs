#![allow(dead_code)]
use std::fmt::Debug;
use std::cmp::Ordering;
use byte_array::BinaryBuilder;
use duplicate::{duplicate, duplicate_item};
use enum_dispatch::enum_dispatch;
use float_cmp::ApproxEq;
use num::FromPrimitive;
use std::collections::HashMap;
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

#[derive(FromPrimitive, Clone, Debug, Copy)]
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

pub enum FieldValue2 {
    I66(i64),
}

#[allow(dead_code)]
#[derive(Debug)]
#[derive(Clone)]

#[enum_dispatch]
pub enum FieldEnumStruct {
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

#[duplicate_item(
    val_type;
    [i64];
    [u64];
    [isize];
    [i8];
    [i16];
    [i32];
    [usize];
    [u8];
    [u16];
    [u32];
    [FloatWrapper];
    [String]
    )]
      
impl GetFieldInfo for Field<val_type> {
    fn get_field_type(&self) -> FieldType {
        self.field_type
    }
}

#[enum_dispatch(FieldEnumStruct)]
pub trait GetFieldInfo {
    fn get_field_type(&self) -> FieldType;
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
        pub position: Vec<u32>,
    } 

    impl PartialEq for DocumentWordAndPositions {
        fn eq(&self, other: &Self) -> bool {
            self.doc_id == other.doc_id
        }
    }


impl PartialOrd for DocumentWordAndPositions {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DocumentWordAndPositions {
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
    pub index:  Vec<G>
}

impl<E: Debug + Clone + Ord > SortedVector<E> for FieldIndex<E> {
    fn get_vec(&mut self) -> &mut Vec<E> {
        &mut self.index
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
    pub fields:  Vec<FieldEnumStruct>
}

#[allow(dead_code)]
#[derive(Debug)]
#[derive(Clone)]  
pub struct Document {
    pub id: u64,
    pub external_id: FieldValue,
    pub values: Vec<FieldValue>
}


