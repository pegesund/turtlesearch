use std::cmp::Ordering;
use std::str;

use byte_array::ByteArray;
use byte_array::BinaryBuilder;
use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;
use std::vec::Vec;
use std::convert::TryFrom;

use crate::structures::FieldType;
use crate::structures::FieldValue;

struct WrappedU8Vec {vec: Vec<u8>}



fn get_value(ba: &mut ByteArray) -> FieldValue {

    let rocks_raw_type: u32 =  ba.read();
    println!("Value: {:?}",  rocks_raw_type);
    let rocks_type: FieldType = FromPrimitive::from_u32(rocks_raw_type).unwrap();
    let res: FieldValue = match rocks_type {
        FieldType::I8 => FieldValue::I8 {value: ba.read()},
        FieldType::I16 => FieldValue::I16 {value: ba.read()},
        FieldType::I32 => FieldValue::I32 {value: ba.read()},
        FieldType::I64 => FieldValue::I64 {value: ba.read()},
        FieldType::U8 => FieldValue::U8 {value: ba.read()},
        FieldType::U16 => FieldValue::U16 {value: ba.read()},
        FieldType::U32 => FieldValue::U32 {value: ba.read()},
        FieldType::U64 => FieldValue::U64 {value: ba.read()},
        FieldType::Usize => FieldValue::Usize {value: ba.read()},
        FieldType::Isize => FieldValue::Isize {value: ba.read()},
        FieldType::F32 => FieldValue::F32 {value: ba.read::<f32>()},
        FieldType::F64 => FieldValue::F64 {value: ba.read::<f64>()},
        FieldType::String => FieldValue::String {value: ba.read::<String>() }
    };
    return res
}

fn put_value(mut ba: &mut ByteArray, val: &FieldValue) {
    match val {
        FieldValue::I64 { value } => { ba <<= &(FieldType::I64 as u32); ba <<= value },
        FieldValue::U64 { value } => { ba <<= &(FieldType::U64 as u32); ba <<= value },
        FieldValue::Isize { value } => { ba <<= &(FieldType::Isize as u32); ba <<= value },
        FieldValue::I8 { value } => { ba <<= &(FieldType::I8 as u32); ba <<= value },
        FieldValue::I16 { value } => { ba <<= &(FieldType::I16 as u32); ba <<= value },
        FieldValue::I32 { value } => { ba <<= &(FieldType::I32 as u32); ba <<= value },
        FieldValue::Usize { value } => { ba <<= &(FieldType::Usize as u32); ba <<= value },
        FieldValue::U8 { value } => { ba <<= &(FieldType::U8 as u32); ba <<= value },
        FieldValue::U16 { value } => { ba <<= &(FieldType::U16 as u32); ba <<= value },
        FieldValue::U32 { value } => { ba <<= &(FieldType::U32 as u32); ba <<= value },
        FieldValue::F32 { value } => { ba <<= &(FieldType::F32 as u32); ba <<= value },
        FieldValue::F64 { value } => { ba <<= &(FieldType::F64 as u32); ba <<= value },
        FieldValue::String { value } => { ba <<= &(FieldType::String as u32); ba <<= value }
    }
}

/*
    Todo: currently copies the arrays into a vector, which is slow.. Large potential for optimize.
*/
pub fn rocks_compare(one: &[u8], two: &[u8]) -> Ordering {
    let mut ba1 = ByteArray {raw: one.to_vec(), pointer: 0 };
    let mut ba2 = ByteArray {raw: two.to_vec(), pointer: 0 };
    while ba1.bytes_available() > 0 {
        let v1:FieldValue = get_value(&mut ba1);
        let v2:FieldValue = get_value(&mut ba2);
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
        let val = FieldValue::I64{value: 23};
        put_value(&mut ba, &val);
        ba.seek_first();
        let val2 = get_value(&mut ba);
        assert_eq!(val, val2);
    }


    #[test]
    fn put_and_get_simple_type2() {
        let mut ba = ByteArray::new();
        let val = FieldValue::F32{value: 23.0};
        put_value(&mut ba, &val);
        ba.seek_first();
        let val2 = get_value(&mut ba);
        assert_eq!(val, val2);
    }


    #[test]
    fn put_and_get_strings() {
        let mut ba = ByteArray::new();
        let val1 = FieldValue::String {value: String::from("Hello1") };
        let val2 = FieldValue::String {value: String::from("Hello2") };
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
        let val1 = FieldValue::String {value: String::from("Hello1") };
        let val2 = FieldValue::U64 {value: 99 };
        let val3 = FieldValue::String {value: String::from("Hello2") };
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
        let val1 = FieldValue::String {value: String::from("Hello1") };
        let val2 = FieldValue::String {value: String::from("Hello2") };
        put_value(&mut ba1, &val1);
        put_value(&mut ba2, &val2);
        let a1: &[u8] = ba1.as_vec().as_slice();
        let a2: &[u8] = ba2.as_vec().as_slice();
        let res = rocks_compare(a1, a2);
        assert_eq!(res, Ordering::Less)
    }

    #[test]
    fn test_compare_simple() {
        let mut ba1 = ByteArray::new();
        let mut ba2 = ByteArray::new();
        let val1 = FieldValue::U64 {value: 1 };
        let val2 = FieldValue::U64 {value: 2 };
        put_value(&mut ba1, &val1);
        put_value(&mut ba2, &val2);
        let a1: &[u8] = ba1.as_vec().as_slice();
        let a2: &[u8] = ba2.as_vec().as_slice();
        let res = rocks_compare(a1, a2);
        assert_eq!(res, Ordering::Less)
    }
    #[test]
    fn test_compare_compound1() {
        let mut ba1 = ByteArray::new();
        let mut ba2 = ByteArray::new();
        let val11 = FieldValue::U64 {value: 1 };
        let val22 = FieldValue::U64 {value: 2 };
        let val1 = FieldValue::String {value: String::from("Hello") };
        let val2 = FieldValue::String {value: String::from("Hello") };
        put_value(&mut ba1, &val1);
        put_value(&mut ba1, &val11);
        put_value(&mut ba2, &val2);
        put_value(&mut ba2, &val22);
        let a1: &[u8] = ba1.as_vec().as_slice();
        let a2: &[u8] = ba2.as_vec().as_slice();
        let res = rocks_compare(a1, a2);
        assert_eq!(res, Ordering::Less)
    }

    fn test_compare_compound2() {
        let mut ba1 = ByteArray::new();
        let mut ba2 = ByteArray::new();
        let val11 = FieldValue::U64 {value: 1 };
        let val22 = FieldValue::U64 {value: 1 };
        let val1 = FieldValue::String {value: String::from("Hello") };
        let val2 = FieldValue::String {value: String::from("Hello") };
        put_value(&mut ba1, &val1);
        put_value(&mut ba1, &val11);
        put_value(&mut ba2, &val2);
        put_value(&mut ba2, &val22);
        let a1: &[u8] = ba1.as_vec().as_slice();
        let a2: &[u8] = ba2.as_vec().as_slice();
        let res = rocks_compare(a1, a2);
        assert_eq!(res, Ordering::Equal)
    }

    
}

