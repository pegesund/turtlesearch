use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;
use std::str;

use byte_array::ByteArray;
use byte_array::BinaryBuilder;
use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;
use std::vec::Vec;
use std::convert::TryFrom;

struct WrappedU8Vec {vec: Vec<u8>}

#[derive(FromPrimitive, Clone)]
enum RocksType {
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
enum RocksValue {
    I64 { value: i64 },
    U64 { value: u64 },
    Isize { value: isize },
    I8 { value: i8 },
    I16 { value: i16 },
    I32 { value: i32 },
    Usize { value: usize },
    U8 { value: u8 },
    U16 { value: u16 },
    U32 { value: i32 },
    F32 { value: f32 },
    F64 { value: f64 },
    String { value: String }
}




fn get_value(ba: &mut ByteArray) -> RocksValue {

    let rocks_raw_type: u32 =  ba.read();
    println!("Value: {:?}",  rocks_raw_type);
    let rocks_type: RocksType = FromPrimitive::from_u32(rocks_raw_type).unwrap();
    let res: RocksValue = match rocks_type {
        RocksType::I8 => RocksValue::I8 {value: ba.read()},
        RocksType::I16 => RocksValue::I16 {value: ba.read()},
        RocksType::I32 => RocksValue::I32 {value: ba.read()},
        RocksType::I64 => RocksValue::I64 {value: ba.read()},
        RocksType::U8 => RocksValue::U8 {value: ba.read()},
        RocksType::U16 => RocksValue::U16 {value: ba.read()},
        RocksType::U32 => RocksValue::U32 {value: ba.read()},
        RocksType::U64 => RocksValue::U64 {value: ba.read()},
        RocksType::Usize => RocksValue::Usize {value: ba.read()},
        RocksType::Isize => RocksValue::Isize {value: ba.read()},
        RocksType::F32 => RocksValue::F32 {value: ba.read::<f32>()},
        RocksType::F64 => RocksValue::F64 {value: ba.read::<f64>()},
        RocksType::String => RocksValue::String {value: ba.read::<String>() }
    };
    return res
}

fn put_value(mut ba: &mut ByteArray, val: &RocksValue) {
    match val {
        RocksValue::I64 { value } => { ba <<= &(RocksType::I64 as u32); ba <<= value },
        RocksValue::U64 { value } => { ba <<= &(RocksType::U64 as u32); ba <<= value },
        RocksValue::Isize { value } => { ba <<= &(RocksType::Isize as u32); ba <<= value },
        RocksValue::I8 { value } => { ba <<= &(RocksType::I8 as u32); ba <<= value },
        RocksValue::I16 { value } => { ba <<= &(RocksType::I16 as u32); ba <<= value },
        RocksValue::I32 { value } => { ba <<= &(RocksType::I32 as u32); ba <<= value },
        RocksValue::Usize { value } => { ba <<= &(RocksType::Usize as u32); ba <<= value },
        RocksValue::U8 { value } => { ba <<= &(RocksType::U8 as u32); ba <<= value },
        RocksValue::U16 { value } => { ba <<= &(RocksType::U16 as u32); ba <<= value },
        RocksValue::U32 { value } => { ba <<= &(RocksType::U32 as u32); ba <<= value },
        RocksValue::F32 { value } => { ba <<= &(RocksType::F32 as u32); ba <<= value },
        RocksValue::F64 { value } => { ba <<= &(RocksType::F64 as u32); ba <<= value },
        RocksValue::String { value } => { ba <<= &(RocksType::String as u32); ba <<= value }
    }
}

/*
    Todo: currently copies the arrays into a vector, which is slow.. Large potential for optimize.
*/
fn compare(one: &[u8], two: &[u8]) -> Ordering {
    let mut ba1 = ByteArray {raw: one.to_vec(), pointer: 0 };
    let mut ba2 = ByteArray {raw: two.to_vec(), pointer: 0 };
    while ba1.bytes_available() > 0 {
        let v1:RocksValue = get_value(&mut ba1);
        let v2:RocksValue = get_value(&mut ba2);
        println!("Values: {:?} - {:?}", v1, v2);
        if v1 < v2 { return Ordering::Less } else
        if v1 > v2 { return Ordering::Greater};
    };
    Ordering::Equal
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn put_and_get_simple_type() {
        let mut ba = ByteArray::new();
        let val = RocksValue::I64{value: 23};
        put_value(&mut ba, &val);
        ba.seek_first();
        let val2 = get_value(&mut ba);
        assert_eq!(val, val2);
    }


    #[test]
    fn put_and_get_simple_type2() {
        let mut ba = ByteArray::new();
        let val = RocksValue::F32{value: 23.0};
        put_value(&mut ba, &val);
        ba.seek_first();
        let val2 = get_value(&mut ba);
        assert_eq!(val, val2);
    }


    #[test]
    fn put_and_get_strings() {
        let mut ba = ByteArray::new();
        let val1 = RocksValue::String {value: String::from("Hello1") };
        let val2 = RocksValue::String {value: String::from("Hello2") };
        put_value(&mut ba, &val1);
        put_value(&mut ba, &val2);
        ba.seek_first();
        let val1_copy = get_value(&mut ba);
        let val2_copy = get_value(&mut ba);
        assert_eq!(val1, val1_copy);
        assert_eq!(val2, val2_copy);
    }

    #[test]
    fn put_and_get_string_value_string() {
        let mut ba = ByteArray::new();
        let val1 = RocksValue::String {value: String::from("Hello1") };
        let val2 = RocksValue::U64 {value: 99 };
        let val3 = RocksValue::String {value: String::from("Hello2") };
        put_value(&mut ba, &val1);
        put_value(&mut ba, &val2);
        put_value(&mut ba, &val3);
        ba.seek_first();
        let val1_copy = get_value(&mut ba);
        let val2_copy = get_value(&mut ba);
        let val3_copy = get_value(&mut ba);
        assert_eq!(val1, val1_copy);
        assert_eq!(val2, val2_copy);
        assert_eq!(val3, val3_copy);        
    }


    #[test]
    fn test_compare_strings() {
        let mut ba1 = ByteArray::new();
        let mut ba2 = ByteArray::new();
        let val1 = RocksValue::String {value: String::from("Hello1") };
        let val2 = RocksValue::String {value: String::from("Hello2") };
        put_value(&mut ba1, &val1);
        put_value(&mut ba2, &val2);
        let a1: &[u8] = ba1.as_vec().as_slice();
        let a2: &[u8] = ba2.as_vec().as_slice();
        let res = compare(a1, a2);
        assert_eq!(res, Ordering::Less)
    }

    #[test]
    fn test_compare_simple() {
        let mut ba1 = ByteArray::new();
        let mut ba2 = ByteArray::new();
        let val1 = RocksValue::U64 {value: 1 };
        let val2 = RocksValue::U64 {value: 2 };
        put_value(&mut ba1, &val1);
        put_value(&mut ba2, &val2);
        let a1: &[u8] = ba1.as_vec().as_slice();
        let a2: &[u8] = ba2.as_vec().as_slice();
        let res = compare(a1, a2);
        assert_eq!(res, Ordering::Less)
    }
    #[test]
    fn test_compare_compound1() {
        let mut ba1 = ByteArray::new();
        let mut ba2 = ByteArray::new();
        let val11 = RocksValue::U64 {value: 1 };
        let val22 = RocksValue::U64 {value: 2 };
        let val1 = RocksValue::String {value: String::from("Hello") };
        let val2 = RocksValue::String {value: String::from("Hello") };
        put_value(&mut ba1, &val1);
        put_value(&mut ba1, &val11);
        put_value(&mut ba2, &val2);
        put_value(&mut ba2, &val22);
        let a1: &[u8] = ba1.as_vec().as_slice();
        let a2: &[u8] = ba2.as_vec().as_slice();
        let res = compare(a1, a2);
        assert_eq!(res, Ordering::Less)
    }

    fn test_compare_compound2() {
        let mut ba1 = ByteArray::new();
        let mut ba2 = ByteArray::new();
        let val11 = RocksValue::U64 {value: 1 };
        let val22 = RocksValue::U64 {value: 1 };
        let val1 = RocksValue::String {value: String::from("Hello") };
        let val2 = RocksValue::String {value: String::from("Hello") };
        put_value(&mut ba1, &val1);
        put_value(&mut ba1, &val11);
        put_value(&mut ba2, &val2);
        put_value(&mut ba2, &val22);
        let a1: &[u8] = ba1.as_vec().as_slice();
        let a2: &[u8] = ba2.as_vec().as_slice();
        let res = compare(a1, a2);
        assert_eq!(res, Ordering::Equal)
    }

    
}

