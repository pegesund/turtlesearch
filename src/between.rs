use std::fmt::Debug; 
use crate::sorted_vector::*;
use crate::structures::*;
use std::cmp::Ordering;
use duplicate::{duplicate, duplicate_item};

pub trait Between<B: Clone + Debug + Ord> {
    fn between(&mut self, start: B, stop: B) -> (usize, usize);
}

pub trait GetValue<V: Clone + Debug + Ord> {
    fn get_value(&self) -> V;
}

#[duplicate_item(
the_class val_type;
[ IntegerSorted  ] [ i64 ];
[ DateSorted  ] [ u64 ];
[ BoolSorted  ] [ bool ];
[ FloatSorted ] [ FloatWrapper ];
)]
impl  GetValue<val_type> for the_class {
    fn get_value(&self) -> val_type {
        self.value
    }
}

#[duplicate_item(
the_class val_type;
[ IntegerSorted ] [ i64 ];
[ DateSorted ][ u64 ];
[ FloatSorted ][ FloatWrapper ];
)]
impl Between<val_type> for FieldIndex<the_class> {

    fn between(&mut self,start: val_type, stop: val_type) -> (usize, usize) {

        let index = self.get_vec_immutable();


        let mut start_index = match index.binary_search_by_key(&start, |e| e.value) {
            Ok(pos) => pos,
            Err(pos) => pos
        };

        let stop_index = match index.binary_search_by_key(&start, |e| e.value) {
            Ok(pos) => pos,
            Err(pos) => pos
        };

        while index[start_index].value == start && start_index > 0{
            start_index -= 1
        }


        while index[stop_index].value == stop && stop_index < index.len() - 1 {
            start_index += 1
        }

        (start_index, stop_index)

    }
}



